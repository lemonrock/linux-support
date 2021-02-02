// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
	
	const TerminatedError: Result<(), bool> = Err(true);
	
	const PollError: Result<(), bool> = Err(false);
	
	#[inline(always)]
	fn new(file_descriptor: &Arc<UserFaultFileDescriptor>, user_fault_event_handler: UFEH, terminate: &Arc<T>, initial_number_of_events_to_read_at_once: NonZeroUsize) -> Self
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
					events: PollRequestFlags::In.bits(),
					revents: 0,
				},
			],
			events:
			{
				let length = initial_number_of_events_to_read_at_once.get();
				let events = Vec::with_capacity(length);
				unsafe { events.set_len(length) };
				events
			},
		}
	}
	
	// TODO: Potential blocking problem: we can terminate, but there are still page faults to read or handle.
	#[inline(always)]
	pub fn poll_and_read_events(&mut self) -> Result<(), bool>
	{
		self.poll()?;
		
		self.read_events()
	}
	
	#[inline(always)]
	fn read_events(&mut self) -> Result<(), bool>
	{
		loop
		{
			let number_of_messages = self.file_descriptor.read_events(&mut events[..]);
			
			for event in &self.events[0 .. number_of_messages]
			{
				self.handle_event(event)
			}
			
			if unlikely!(number_of_messages == self.events.capacity())
			{
				self.increase_events_capacity()
			}
			else
			{
				return Ok(())
			}
			
			if self.terminate.should_continue()
			{
				return Self::TerminatedError
			}
		}
	}
	
	#[inline(always)]
	fn poll(&mut self) -> Result<(), bool>
	{
		while self.terminate.should_continue()
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
						// On Linux, `EAGAIN` is not supposed to be possible as the result is `0`.
						EINTR | ENOMEM | EAGAIN => continue,
						
						EFAULT => panic!("fds points outside the process's accessible address space. The array given as argument was not contained in the calling program's address space."),
						
						EINVAL => panic!("The nfds value exceeds the RLIMIT_NOFILE value"),
						
						_ => panic!("Unexpected errno `{}`", errno)
					}
				}
				
				_ => unreachable_code(format_args!("poll() returned unexpected result {}", result)),
			}
		}
		
		Self::TerminatedError
	}
	
	#[inline(always)]
	fn handle_event(&mut self, event: &uffd_msg)
	{
		use self::UserFaultEvent::*;
		
		let arg = event.arg;
		
		match event.event
		{
			PageFault =>
			{
				let page_fault = unsafe { arg.pagefault };
				let virtual_address = VirtualAddress::from(page_fault.address);
				let page_fault_event_flags = page_fault.flags;
				let thread_identifier = unsafe { page_fault.feat.ptid };
				self.user_fault_event_handler.page_fault(virtual_address, page_fault_event_flags, thread_identifier)
			}
			
			Fork =>
			{
				let fork = unsafe { arg.fork };
				let child_process_user_fault_file_descriptor = FileDescriptorCopy::new(fork.ufd);
				self.user_fault_event_handler.fork(child_process_user_fault_file_descriptor)
			}
			
			Remap =>
			{
				let remap = unsafe { arg.remap };
				let from_mapped_absolute_memory_range = FastAbsoluteMemoryRange
				{
					inclusive_absolute_start: VirtualAddress::from(remap.from),
					length: remap.len as usize,
				};
				let to = VirtualAddress::from(remap.to);
				self.user_fault_event_handler.remap(from_mapped_absolute_memory_range, to)
			}
			
			Remove => self.user_fault_event_handler.remove(Self::remove_to_mapped_absolute_memory_range(arg)),
			
			Unmap => self.user_fault_event_handler.unmap(Self::remove_to_mapped_absolute_memory_range(arg)),
			
			_ =>
			{
				let reserved = unsafe { arg.reserved };
				self.user_fault_event_handler.future_unsupported_event(reserved.reserved1, reserved.reserved2, reserved.reserved3)
			}
		}
	}
	
	#[inline(always)]
	fn remove_to_mapped_absolute_memory_range(arg: uffd_msg_arg) -> FastAbsoluteMemoryRange
	{
		let remove = unsafe { arg.remove };
		let inclusive_absolute_start = VirtualAddress::from(remove.start);
		let end = VirtualAddress::from(remove.end);
		FastAbsoluteMemoryRange
		{
			inclusive_absolute_start,
			length: end - inclusive_absolute_start,
		}
	}
	
	#[inline(always)]
	fn increase_events_capacity(&mut self)
	{
		self.events.reserve_exact(self.events.capacity());
		unsafe { self.events.set_len(self.events.capacity()) }
	}
	
	#[inline(always)]
	fn response(&self) -> Result<(), bool>
	{
		let response = unsafe { PollResponseFlags::from_bits_unchecked(self.poll.get_unchecked_safe(0).revents) };
		
		if response == PollResponseFlags::In
		{
			Ok(())
		}
		else if response == PollResponseFlags::Error
		{
			Self::PollError
		}
		else
		{
			unreachable_code(format_args!("response 0x{:02X} contains other flags", response))
		}
	}
}
