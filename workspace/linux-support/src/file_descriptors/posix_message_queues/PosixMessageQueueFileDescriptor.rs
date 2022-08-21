// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct PosixMessageQueueFileDescriptor(mqd_t);

impl Drop for PosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for PosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for PosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for PosixMessageQueueFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for PosixMessageQueueFileDescriptor
{
}

impl PosixMessageQueueFileDescriptor
{
	/// Creates a new instance.
	#[inline(always)]
	pub(crate) fn new(name: &CStr, send_or_receive: PosixMessageQueueCreateSendOrReceive, open_or_create: &OpenOrCreatePosixMessageQueue) -> Result<(Self, PosixMessageQueueConstraints), CreationError>
	{
		match open_or_create.invoke_mq_open(send_or_receive, name)
		{
			Err(error) => Err(error),

			Ok(this) =>
			{
				let attributes = this.queue_attributes();

				Ok
				(
					(
						this,
						PosixMessageQueueConstraints
						{
							maximum_number_of_enqueued_messages: attributes.maximum_number_of_enqueued_messages(),
							maximum_message_size_in_bytes: attributes.maximum_message_size_in_bytes(),
						}

					)
				)
			}
		}
	}

	pub(crate) fn send(&self, message_buffer: &[u8], message_priority: PosixMessagePriority) -> Result<(), StructWriteError>
	{
		let result = unsafe { mq_timedsend(self.0, message_buffer.as_ptr() as *const _ as *const _, message_buffer.len(), message_priority.into(), null()) };

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::StructWriteError::*;

			Err
			(
				match SystemCallErrorNumber::from_errno_panic()
				{
					EAGAIN => WouldBlock,
					EINTR => Interrupted,

					EMSGSIZE => panic!("`msg_len` was greater than the `mq_msgsize` attribute of the message queue"),
					EBADF => panic!("The descriptor specified in `mqdes` was invalid or not opened for reading"),
					EINVAL => panic!("The call would have blocked, and `abs_timeout` was invalid, either because `tv_sec` was less than zero, or because `tv_nsec` was less than zero or greater than 1000 million"),
					ETIMEDOUT => panic!("The call timed out before a message could be transferred"),
					
					unexpected_error @ _ => unexpected_error!(mq_timedsend, unexpected_error),
				}
			)
		}
		else
		{
			unexpected_result!(mq_timedsend, result)
		}
	}

	/// Returns a tuple of `(message_size, message_priority)`.
	///
	/// Fails with a panic if the `message_buffer` is too small for the queue's configured message size.
	pub(crate) fn receive(&self, message_buffer: &mut [u8]) -> Result<(usize, PosixMessagePriority), StructReadError>
	{
		let mut priority = unsafe_uninitialized();
		let result = unsafe { mq_timedreceive(self.0, message_buffer.as_mut_ptr() as *mut _ as *mut _, message_buffer.len(), &mut priority, null()) } ;
		if likely!(result >= 0)
		{
			Ok((result as usize, PosixMessagePriority(priority as u16)))
		}
		else if likely!(result == -1)
		{
			use self::StructReadError::*;

			Err
			(
				match SystemCallErrorNumber::from_errno_panic()
				{
					EAGAIN => WouldBlock,
					EINTR => Interrupted,

					EMSGSIZE => panic!("`msg_len` was less than the `mq_msgsize` attribute of the message queue"),
					EBADF => panic!("The descriptor specified in `mqdes` was invalid or not opened for reading"),
					EINVAL => panic!("The call would have blocked, and `abs_timeout` was invalid, either because `tv_sec` was less than zero, or because `tv_nsec` was less than zero or greater than 1000 million"),
					ETIMEDOUT => panic!("The call timed out before a message could be transferred"),
					
					unexpected_error @ _ => unexpected_error!(mq_timedreceive, unexpected_error),
				}
			)
		}
		else
		{
			unexpected_result!(mq_timedreceive, result);
		}
	}

	#[inline(always)]
	pub(crate) fn queue_depth(&self) -> usize
	{
		self.queue_attributes().queue_depth()
	}

	/// Obtains queue attributes.
	///
	/// Not particularly useful except for sizing send or receive buffers.
	#[inline(always)]
	pub(crate) fn queue_attributes(&self) -> mq_attr
	{
		let mut attributes = unsafe_zeroed();
		let result = unsafe { mq_getattr(self.as_raw_fd(), &mut attributes) };

		if likely!(result == 0)
		{
			attributes
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				EBADF => panic!("The message queue descriptor specified in `mqdes` is invalid"),
				EINVAL => panic!("`newattr.mq_flags` contained set bits other than `O_NONBLOCK`"),
				
				unexpected_error @ _ => unexpected_error!(mq_getattr, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(mq_getattr, result)
		}
	}

	pub(crate) fn guard_name(name: &CStr)
	{
		if cfg!(debug_assertions)
		{
			let bytes = name.to_bytes();
			let length = bytes.len();
			debug_assert!(length > 1, "name must be 2 bytes or more long (excluding the trailing NUL)");
			debug_assert!(length < 256, "name must be 255 bytes or less long (excluding the trailing NUL)");

			debug_assert_eq!(bytes[0], b'/', "name must start with a slash");
			for byte in name.to_bytes()[ 1 .. ].iter()
			{
				debug_assert_ne!(byte, &b'/', "name contains more than one slash");
			}
		}
	}
}
