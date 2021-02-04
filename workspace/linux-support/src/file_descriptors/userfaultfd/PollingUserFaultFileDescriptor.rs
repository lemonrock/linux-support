// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A structure to be used on a dedicated thread.
pub struct PollingUserFaultFileDescriptor<UFEH: UserFaultEventHandler, T: Terminate>
{
	file_descriptor: Arc<UserFaultFileDescriptor>,
	user_fault_event_handler: UFEH,
	terminate: Arc<T>,
	poll: [pollfd; 1],
	events: Vec<uffd_msg>,
}

impl<UFEH: UserFaultEventHandler, T: Terminate> PollingUserFaultFileDescriptor<UFEH, T>
{
	const DefaultTimeoutInMilliseconds: i32 = 1000;
	
	const TerminatedError: Result<(), ()> = Err(());
	
	#[inline(always)]
	fn new(file_descriptor: &Arc<UserFaultFileDescriptor>, terminate: &Arc<T>, initial_number_of_events_to_read_at_once: NonZeroUsize, user_fault_event_handler: UFEH) -> Self
	{
		Self
		{
			file_descriptor: file_descriptor.clone(),
			user_fault_event_handler,
			terminate: terminate.clone(),
			poll:
			[
				pollfd
				{
					fd: file_descriptor.0,
					events: PollRequestFlags::In.bits() as i16,
					revents: 0,
				}
			],
			events:
			{
				let length = initial_number_of_events_to_read_at_once.get();
				let mut events = Vec::with_capacity(length);
				unsafe { events.set_len(length) };
				events
			},
		}
	}
	
	// TODO: Potential blocking problem: we can terminate, but there are still page faults to read or handle.
	/// Poll and read events.
	#[inline(always)]
	pub fn poll_and_read_events(&mut self) -> Result<(), ()>
	{
		self.poll()?;
		
		self.read_events()
	}
	
	#[inline(always)]
	fn poll(&mut self) -> Result<(), ()>
	{
		while self.should_continue()
		{
			match unsafe { poll(self.poll.as_mut_ptr(), 1, Self::DefaultTimeoutInMilliseconds) }
			{
				1 => return self.response(),
				
				0 => continue,
				
				-1 =>
				{
					let errno = errno();
					match errno.0
					{
						// On Linux as opposed to the POSIX standard, `EAGAIN` is not possible as the result is `0`.
						EINTR | ENOMEM => continue,
						
						EFAULT => panic!("`fds` points outside the process's accessible address space. The array given as argument was not contained in the calling program's address space."),
						
						EINVAL => panic!("The `nfds` value exceeds the `RLIMIT_NOFILE` value"),
						
						_ => panic!("Unexpected errno `{}`", errno)
					}
				}
				
				result @ _ => unreachable_code(format_args!("poll() returned unexpected result {}", result)),
			}
		}
		
		Self::TerminatedError
	}
	
	#[inline(always)]
	fn read_events(&mut self) -> Result<(), ()>
	{
		while self.should_continue()
		{
			let number_of_messages = self.file_descriptor.non_blocking_read_events(&mut self.events[..]);
			
			for index in 0 .. number_of_messages
			{
				Self::handle_event(self.events.get_unchecked_safe(index), &mut self.user_fault_event_handler)
			}
			
			if unlikely!(number_of_messages == self.events.capacity())
			{
				self.increase_events_capacity()
			}
			else
			{
				return Ok(())
			}
		}
		
		Self::TerminatedError
	}
	
	#[inline(always)]
	fn should_continue(&self) -> bool
	{
		self.terminate.should_continue()
	}
	
	#[inline(always)]
	fn handle_event(event: &uffd_msg, user_fault_event_handler: &mut UFEH)
	{
		use self::UserFaultEvent::*;
		
		let arg = event.arg;
		
		match event.event
		{
			PageFault =>
			{
				let (address_access_that_caused_page_fault, page_fault_event_flags, thread_identifier) = Self::page_fault_arguments(arg);
				user_fault_event_handler.page_fault(address_access_that_caused_page_fault, page_fault_event_flags, thread_identifier)
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
	fn page_fault_arguments(arg: uffd_msg_arg) -> (VirtualAddress, PageFaultEventFlags, Option<ThreadIdentifier>)
	{
		let page_fault = unsafe { arg.pagefault };
		let address_access_that_caused_page_fault = VirtualAddress::from(page_fault.address);
		let page_fault_event_flags = page_fault.flags;
		let thread_identifier = unsafe { page_fault.feat.ptid };
		(address_access_that_caused_page_fault, page_fault_event_flags, thread_identifier)
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
	
	#[inline(always)]
	fn response(&self) -> Result<(), ()>
	{
		let response = unsafe { PollResponseFlags::from_bits_unchecked(self.poll.get_unchecked_safe(0).revents as u16) };
		
		if likely!(response == PollResponseFlags::In)
		{
			Ok(())
		}
		else
		{
			if response == PollResponseFlags::HangUp
			{
				unreachable_code_const("An examination of userfaultfd_poll() in Linux source file fs/userfaultfd.c suggests (E)POLLHUP can only occur when the user fault file descriptor has been released (ie all referenced instances have been `close()`d; since we hold a reference in this structure through an Arc, this should be impossible")
			}
			else if response == PollResponseFlags::Error
			{
				debug_assert!(self.file_descriptor.is_non_blocking(), "User fault file descriptor has been set to blocking");
				
				unreachable_code_const("An examination of userfaultfd_poll() in Linux source file fs/userfaultfd.c suggests (E)POLLERR can only occur if either (a) the UFFDIO_API ioctl has not been made (it has as part of construction of an UserFaultFileDescriptor) or (b) the file descriptor is blocking (it is created non-blocking with O_NONBLOCK)");
			}
			else
			{
				unreachable_code(format_args!("response 0x{:02X} contains an unexpected combination of flags", response))
			}
		}
	}
}
