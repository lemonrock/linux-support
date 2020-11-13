// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a negative user space `SI_*` code in common use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UserspaceSignalCode
{
	/// Supposedly caused by the libc function `sigqueue()`, but the syscall this calls can take any *negative* `SI_*` value (ie not `SI_KERNEL` or `SI_USER`) apart from `SI_TKILL`.
	///
	/// Typically used for realtime signals, and indicates (indirectly) that this signal is from a queue, ie multiple signals of the same number are not made idempotent.
	Queue,

	/// Caused by libc implementations of POSIX message queues.
	PosixMessageQueue,

	/// Caused by libc implementations of POSIX AIO.
	PosixAsynchronousIo,

	/// Obsolete; not used by the Linux kernel.
	///
	/// Linux source comment: "Sent by `execve()` killing subsidiary threads".
	Dethread,

	/// Only used by glibc for asynchronous (DNS) name look up.
	///
	/// Not used by musl.
	AsynchronousNameLookUp,

	/// Any other negative value (can never be zero or positive).
	///
	/// This should be rarely encountered unless an application is using system calls directly.
	Other(NonZeroI32),
}

impl Into<i32> for UserspaceSignalCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		use self::UserspaceSignalCode::*;

		match self
		{
			Queue => SI_QUEUE,
			PosixMessageQueue => SI_MESGQ,
			PosixAsynchronousIo => SI_ASYNCIO,
			Dethread => SI_DETHREAD,
			AsynchronousNameLookUp => SI_ASYNCNL,
			Other(code) => code.get(),
		}
	}
}

impl UserspaceSignalCode
{
	#[inline(always)]
	pub(crate) fn from_si_code(si_code: i32) -> Self
	{
		debug_assert!(si_code < 0, "si_code is not negative");

		use self::UserspaceSignalCode::*;

		match si_code
		{
			SI_QUEUE => Queue,
			SI_MESGQ => PosixMessageQueue,
			SI_ASYNCIO => PosixAsynchronousIo,
			SI_DETHREAD => Dethread,
			SI_ASYNCNL => AsynchronousNameLookUp,
			other @ _ => Other(new_non_zero_i32(other)),
		}
	}
}
