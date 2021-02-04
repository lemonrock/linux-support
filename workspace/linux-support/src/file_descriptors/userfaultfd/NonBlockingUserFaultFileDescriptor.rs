// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A structure to be used on a dedicated thread for uncooperative monitoring of processes.
pub struct NonBlockingUserFaultFileDescriptor<UFEH: UserFaultEventHandler, T: Terminate>
{
	event_reader_and_handler: EventReaderAndDispatcher<UFEH>,
	file_descriptor: Arc<UserFaultFileDescriptor>,
	poll: [pollfd; 1],
	terminate: Arc<T>,
}

impl<UFEH: UserFaultEventHandler, T: Terminate> NonBlockingUserFaultFileDescriptor<UFEH, T>
{
	const DefaultTimeoutInMilliseconds: i32 = 1000;
	
	const TerminatedError: Result<(), ()> = Err(());
	
	#[inline(always)]
	fn new(file_descriptor: &Arc<UserFaultFileDescriptor>, initial_number_of_events_to_read_at_once: NonZeroUsize, user_fault_event_handler: UFEH, terminate: &Arc<T>) -> Self
	{
		Self
		{
			event_reader_and_handler: EventReaderAndDispatcher::new(file_descriptor, initial_number_of_events_to_read_at_once, user_fault_event_handler),
			file_descriptor: file_descriptor.clone(),
			poll:
			[
				pollfd
				{
					fd: file_descriptor.0,
					events: PollRequestFlags::In.bits() as i16,
					revents: 0,
				}
			],
			terminate: terminate.clone(),
		}
	}
	
	/// Poll and read events.
	///
	/// __NOTE__: If used to monitor a process which contains the thread executing `poll_and_read_events()` then it is possible to enter a deadlock-akin situation:-
	///
	/// 1. A thread on the monitored processes requests termination by calling `Terminate::begin_termination()`.
	/// 2. The same thread, or another, tries to read or write to memory which causes a user land page-fault event notification that this thread, executing `poll_and_read_events()`, should handle.
	/// 3. The thread in (2) above is blocked (suspended) until the page fault is resolved (ie `copy_registered_memory_subrange(wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange = true)`, `zero_registered_memory_subrange(wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange = true)` or `wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange()` in `UserFaultFileDescriptor` has not be called).
	/// 4. However, `poll_and_read_events()` returns because `Terminate::should_continue()` is now false, and it never reads the page fault notification event.
	/// 5. Deadlock. The thread in (2) is suspended forever.
	#[inline(always)]
	pub fn poll_and_read_events(&mut self) -> Result<(), ()>
	{
		self.poll()?;
		
		self.read_and_handle_events()
	}
	
	#[inline(always)]
	fn poll(&mut self) -> Result<(), ()>
	{
		while self.should_continue()
		{
			match unsafe { poll(self.poll.as_mut_ptr(), 1, Self::DefaultTimeoutInMilliseconds) }
			{
				1 => return self.process_poll_response(),
				
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
	fn read_and_handle_events(&mut self) -> Result<(), ()>
	{
		while self.should_continue()
		{
			let more_events_to_read_immediately = self.event_reader_and_handler.read_and_dispatch_events_non_blocking();
			
			if unlikely!(more_events_to_read_immediately)
			{
				continue
			}
			else
			{
				return Ok(())
			}
		}
		
		Self::TerminatedError
	}
	
	#[inline(always)]
	fn process_poll_response(&self) -> Result<(), ()>
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
	
	#[inline(always)]
	fn should_continue(&self) -> bool
	{
		self.terminate.should_continue()
	}
}
