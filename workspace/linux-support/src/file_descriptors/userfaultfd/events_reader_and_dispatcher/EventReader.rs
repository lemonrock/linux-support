// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct EventReader
{
	file_descriptor: Arc<UserFaultFileDescriptor>,
	dereferenced_file_descriptor: RawFd,
}

impl EventReader
{
	#[cold]
	fn new(file_descriptor: &Arc<UserFaultFileDescriptor>) -> Self
	{
		let file_descriptor = file_descriptor.clone();
		
		Self
		{
			dereferenced_file_descriptor: file_descriptor.0,
			file_descriptor,
		}
	}
	
	#[inline(always)]
	fn blocking_read_events<'a>(&self, buffer: NonNull<[uffd_msg]>) -> NonZeroUsize
	{
		debug_assert!(self.file_descriptor.is_blocking());
		
		let number_of_messages = self.read_events(buffer, || unreachable_code_const("EAGAIN should not be possible"));
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
	fn non_blocking_read_events<'a>(&self, buffer: NonNull<[uffd_msg]>) -> usize
	{
		debug_assert!(self.file_descriptor.is_non_blocking());
		
		self.read_events(buffer, || 0)
	}
	
	#[inline(always)]
	fn read_events(&self, buffer: NonNull<[uffd_msg]>, handle_EAGAIN: impl Fn() -> usize) -> usize
	{
		let buffer_pointer = buffer.as_mut_ptr() as *mut c_void;
		let maximum_number_of_messages_to_read = buffer.len();
		
		loop
		{
			const MessageSize: usize = size_of::<uffd_msg>();
			
			let result = unsafe { libc::read(self.dereferenced_file_descriptor, buffer_pointer, MessageSize * maximum_number_of_messages_to_read) };
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
					// This code path seems to only be possible if fork events can be raised.
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
}
