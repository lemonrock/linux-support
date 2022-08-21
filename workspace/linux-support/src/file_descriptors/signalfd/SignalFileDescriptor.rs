// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use crate::syscall::SystemCallResult;

/// Represents a signal instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalFileDescriptor(RawFd);

impl Drop for SignalFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// Please see <http://austingroupbugs.net/view.php?id=529> and <http://austingroupbugs.net/view.php?id=529> for why ignoring the `EINTR` error on close is actually sane.
		//
		// Frankly, the defects here are those of POSIX: (a) signals, and (b) using a file descriptor so small that it isn't thread safe.
		//
		// To be far, both signals and file descriptors predate threads by a long way.
		unsafe { close(self.0) };
	}
}

impl AsRawFd for SignalFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for SignalFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for SignalFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for SignalFileDescriptor
{
}

impl SignalFileDescriptor
{
	/// Creates a new instance for all signals, and returns itself and the signal mask used; blocks all signals that are in the mask if creation of a new instance was successful.
	#[inline(always)]
	pub fn new_with_filled_signal_mask() -> Result<(Self, sigset_t), CreationError>
	{
		let signal_mask = Signals::filled_signal_mask();
		Self::new(&signal_mask).map(|this|
		{
			Signals::block_signals(&signal_mask);

			(this, signal_mask)
		})
	}

	/// Creates a new instance.
	#[inline(always)]
	pub fn new(signal_mask: &sigset_t) -> Result<Self, CreationError>
	{
		use self::CreationError::*;
		
		let result = unsafe { signalfd(-1, signal_mask, SFD_NONBLOCK | SFD_CLOEXEC) };
		match result
		{
			raw_file_descriptor @ SystemCallResult::InclusiveMinimumRawFileDescriptor_i32 ..= SystemCallResult::InclusiveMaximumRawFileDescriptor_i32 => Ok(SignalFileDescriptor(raw_file_descriptor)),
			
			-1 => Err
			(
				match SystemCallErrorNumber::from_errno_panic()
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					ENOMEM => KernelWouldBeOutOfMemory,
					EINVAL => panic!("Invalid arguments"),
					ENODEV => panic!("Could not mount (internal) anonymous inode device"),
					unexpected_error @ _ => unexpected_error!(signalfd, unexpected_error),
				}
			),
			
			_ => unexpected_result!(signalfd, "new", result),
		}
	}

	/// Updates the signal mask
	///
	/// The `initial_value` can not be `u64::MAX`.
	#[inline(always)]
	pub fn update_mask(&self, signal_mask: &sigset_t) -> Result<(), CreationError>
	{
		use self::CreationError::*;
		
		let result = unsafe { signalfd(self.0, signal_mask, SFD_NONBLOCK | SFD_CLOEXEC) };
		
		match result
		{
			SystemCallResult::InclusiveMinimumRawFileDescriptor_i32 ..= SystemCallResult::InclusiveMinimumRawFileDescriptor_i32 if result == self.0 => Ok(()),
			
			SystemCallResult::InclusiveMinimumRawFileDescriptor_i32 ..= SystemCallResult::InclusiveMinimumRawFileDescriptor_i32 => unexpected_result!(signalfd, "update mask had mismatched file descriptor", result),
			
			-1 => Err
			(
				match SystemCallErrorNumber::from_errno_panic()
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					ENOMEM => KernelWouldBeOutOfMemory,
					EBADF => panic!("Invalid signalfd"),
					EINVAL => panic!("Invalid arguments"),
					ENODEV => panic!("Could not mount (internal) anonymous inode device"),
					unexpected_error @ _ => unexpected_error!(signalfd, unexpected_error),
				}
			),
			
			_ => unexpected_result!(signalfd, "update mask", result),
		}
		
		if likely!(result != -1)
		{
			Ok(())
		}
		else
		{
		
		}
	}

	/// Reads signals.
	///
	/// Use this only after a read-ready event notification is received (using edge-triggered events).
	#[inline(always)]
	pub fn read_signals(&self, uninitialized_buffer: (NonNull<signalfd_siginfo>, usize), mut signal_handler: impl FnMut(ParsedSignal)) -> Result<(), StructReadError>
	{
		let structures = self.read_internal(uninitialized_buffer)?;
		for index in 0 .. structures
		{
			let siginfo = unsafe { & * (uninitialized_buffer.0.as_ptr().add(index)) };
			signal_handler(siginfo.parse_signal())
		}
		Ok(())
	}

	#[inline(always)]
	fn read_internal(&self, uninitialized_buffer: (NonNull<signalfd_siginfo>, usize)) -> Result<usize, StructReadError>
	{
		use self::StructReadError::*;

		const SizeOfRead: usize = size_of::<signalfd_siginfo>();

		let result = unsafe { libc::read(self.as_raw_fd(), uninitialized_buffer.0.as_ptr() as *mut _, SizeOfRead * uninitialized_buffer.1) };

		let structures = result / result % (SizeOfRead as isize);

		if likely!(structures != 0)
		{
			let structures = structures as usize;
			debug_assert!(structures <= uninitialized_buffer.1);
			Ok(structures)
		}
		else
		{
			match result
			{
				-1 =>
				{
					match SystemCallErrorNumber::from_errno_panic()
					{
						EAGAIN => Err(WouldBlock),
						ECANCELED => Err(Cancelled),
						EINTR => Err(Interrupted),
						EIO => Err(Cancelled),
						EBADF => panic!("`fd` is not a valid file descriptor or is not open for reading"),
						EFAULT => panic!("`buf` is outside your accessible address space"),
						EINVAL => panic!("`fd` is attached to an object which is unsuitable for reading OR was created via a call to `timerfd_create()` and the wrong size buffer was given to `read()`"),
						EISDIR => panic!("`fd` refers to a directory"),

						// ERESTARTSYS is possible but should not occur.
						
						unexpected_error @ _ => unexpected_error!(read, "signal file descriptor", unexpected_error),
					}
				}

				0 => panic!("End of file but we haven't closed the file descriptor"),

				unexpected @ _ => unexpected_result!(read, "signal file descriptor", unexpected),
			}
		}
	}
}
