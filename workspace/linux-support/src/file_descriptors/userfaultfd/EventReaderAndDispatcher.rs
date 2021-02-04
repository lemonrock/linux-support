// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct EventReaderAndDispatcher<UFEH: UserFaultEventHandler>
{
	file_descriptor: Arc<UserFaultFileDescriptor>,
	user_fault_event_handler: UFEH,
	events: Vec<uffd_msg>,
}

impl<UFEH: UserFaultEventHandler> EventReaderAndDispatcher<UFEH>
{
	#[inline(always)]
	fn new(file_descriptor: &Arc<UserFaultFileDescriptor>, initial_number_of_events_to_read_at_once: NonZeroUsize, user_fault_event_handler: UFEH) -> Self
	{
		Self
		{
			file_descriptor: file_descriptor.clone(),
			user_fault_event_handler,
			events:
			{
				let length = initial_number_of_events_to_read_at_once.get();
				let mut events = Vec::with_capacity(length);
				unsafe { events.set_len(length) };
				events
			},
		}
	}
	
	#[inline(always)]
	fn read_and_dispatch_events_blocking(&mut self)
	{
		let number_of_messages = self.blocking_read_events();
		self.dispatch_events(number_of_messages.get());
	}
	
	/// Returns `true` if it is believed there are more events to read immediately.
	#[inline(always)]
	fn read_and_dispatch_events_non_blocking(&mut self) -> bool
	{
		let number_of_messages = self.non_blocking_read_events();
		self.dispatch_events(number_of_messages)
	}
	
	#[inline(always)]
	fn blocking_read_events<'a>(&mut self) -> NonZeroUsize
	{
		debug_assert!(self.file_descriptor.is_blocking());
		
		let number_of_messages = self.read_events(|| unreachable_code_const("EAGAIN should not be possible"));
		debug_assert_ne!(number_of_messages, 0, "zero in this case means end-of-file, which is not possible for an userfaultfd");
		new_non_zero_usize(number_of_messages)
	}
	
	/// Called from polling thread; loops until does not receive `EINTR`.
	///
	/// Ideally should be done after `poll()` or `epoll()`.
	/// Assumes that `self` still has the flag `O_NONBLOCK` (checked for in debug builds).
	///
	/// Only ever returns `0` if the underlying read returned `EAGAIN`; a return of `0` does not mean end-of-file (EOF).
	#[inline(always)]
	fn non_blocking_read_events<'a>(&mut self) -> usize
	{
		debug_assert!(self.file_descriptor.is_non_blocking());
		
		self.read_events(|| 0)
	}
	
	#[inline(always)]
	fn read_events<'a>(&mut self, handle_EAGAIN: impl Fn() -> usize) -> usize
	{
		const MessageSize: usize = size_of::<uffd_msg>();
		
		let maximum_number_of_messages_to_read = self.events.len();
		let buf = self.events.as_mut_ptr() as *mut c_void;
		loop
		{
			let result = unsafe { libc::read(self.file_descriptor.0, buf, MessageSize * maximum_number_of_messages_to_read) };
			if likely!(result > 0)
			{
				let bytes_read = result as usize;
				
				let number_of_messages = bytes_read / MessageSize;
				debug_assert!(number_of_messages <= maximum_number_of_messages_to_read, "Read more than the size of events (?how)");
				debug_assert_eq!(bytes_read % MessageSize, 0, "Partial read of a message (?how)");
				
				return number_of_messages
			}
			else if likely!(result == -1)
			{
				let errno = errno();
				match errno.0
				{
					EINTR => continue,
					
					// Occurs internally in `userfaultfd_ctx_read()` in Linux source if `no_wait` is true.
					// Is only then returned to user space if no messages have been read at all and the file descriptor is non-blocking.
					EAGAIN => return handle_EAGAIN(),
					
					EBADF => panic!("We hold the file descriptor through an Arc"),
					
					EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or size of `buf` was less than the size of `struct uffd_msg`"),
					
					EFAULT => panic!("`buf` does not point to a valid memory address"),
					
					// Internally, `read()` calls `userfaultfd_read()` which calls `userfaultfd_ctx_read()` which calls `resolve_userfault_fork()` which calls `anon_inode_getfd()` and this can error (?with what).
					// This  code path seems to only be possible if fork events can be raised.
					_ => panic!("Unexpect errno `{}` from userfaultfd non-blocking read()", errno),
				}
			}
			else if likely!(result == 0)
			{
				unreachable_code_const("End-of-File")
			}
			else
			{
				unreachable_code(format_args!("Unexpected result {}", result));
			}
		}
	}
	
	#[inline(always)]
	fn dispatch_events(&mut self, number_of_messages: usize) -> bool
	{
		for index in 0 .. number_of_messages
		{
			Self::dispatch_event(self.events.get_unchecked_safe(index), &mut self.user_fault_event_handler)
		}
		
		if unlikely!(number_of_messages == self.events.capacity())
		{
			self.increase_events_capacity();
			true
		}
		else
		{
			false
		}
	}
	
	#[inline(always)]
	fn dispatch_event(event: &uffd_msg, user_fault_event_handler: &mut UFEH)
	{
		use self::UserFaultEvent::*;
		
		let arg = event.arg;
		
		match event.event
		{
			PageFault =>
			{
				let (page_fault_event_type, address_access_that_caused_page_fault, thread_identifier) = Self::page_fault_arguments(arg);
				
				use self::PageFaultEventType::*;
				
				match page_fault_event_type
				{
					MissingReadFault => user_fault_event_handler.missing_read_page_fault(address_access_that_caused_page_fault, thread_identifier),
					
					MissingWriteFault => user_fault_event_handler.missing_write_page_fault(address_access_that_caused_page_fault, thread_identifier),
					
					WriteProtectionFault => user_fault_event_handler.write_protection_page_fault(address_access_that_caused_page_fault, thread_identifier),
				}
			}
			
			Fork => user_fault_event_handler.fork(Self::fork_arguments(arg)),
			
			Remap =>
			{
				let (from_registered_memory_subrange, to) = Self::remap_arguments(arg);
				user_fault_event_handler.remap(from_registered_memory_subrange, to)
			}
			
			Remove => user_fault_event_handler.remove(Self::remove_or_unmap_arguments(arg)),
			
			Unmap => user_fault_event_handler.unmap(Self::remove_or_unmap_arguments(arg)),
		}
	}
	
	#[inline(always)]
	fn page_fault_arguments(arg: uffd_msg_arg) -> (PageFaultEventType, VirtualAddress, Option<ThreadIdentifier>)
	{
		let page_fault = unsafe { arg.pagefault };
		let page_fault_event_type = page_fault.flags;
		let address_access_that_caused_page_fault = VirtualAddress::from(page_fault.address);
		let thread_identifier = unsafe { page_fault.feat.ptid };
		(page_fault_event_type, address_access_that_caused_page_fault, thread_identifier)
	}
	
	#[inline(always)]
	fn fork_arguments(arg: uffd_msg_arg) -> UserFaultFileDescriptor
	{
		let fork = unsafe { arg.fork };
		UserFaultFileDescriptor(fork.ufd)
	}
	
	#[inline(always)]
	fn remap_arguments(arg: uffd_msg_arg) -> (FastAbsoluteMemoryRange, VirtualAddress)
	{
		let remap = unsafe { arg.remap };
		let from_registered_memory_subrange = FastAbsoluteMemoryRange::new(VirtualAddress::from(remap.from), remap.len as usize);
		let to = VirtualAddress::from(remap.to);
		(from_registered_memory_subrange, to)
	}
	
	#[inline(always)]
	fn remove_or_unmap_arguments(arg: uffd_msg_arg) -> FastAbsoluteMemoryRange
	{
		let remove = unsafe { arg.remove };
		let inclusive_absolute_start = VirtualAddress::from(remove.start);
		let end = VirtualAddress::from(remove.end);
		FastAbsoluteMemoryRange::new(inclusive_absolute_start, end - inclusive_absolute_start)
	}
	
	#[inline(always)]
	fn increase_events_capacity(&mut self)
	{
		self.events.reserve_exact(self.events.capacity());
		unsafe { self.events.set_len(self.events.capacity()) }
	}
}
