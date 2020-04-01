// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


bitflags!
{
	/// Flags to use for epoll `add()`.
	pub struct EPollModifyFlags: u32
	{
		/// The associated file descriptor is available for read operations.
		const Input = EPOLLIN;

		/// The associated file descriptor has urgent or out-of-band data available.
		///
		/// For TCP socket file descriptors, this usually means that out-of-band data (which is a deprecated concept) is available.
		const InputPriority = EPOLLPRI;

		/// The associated file descriptor is available for write operations.
		const Output = EPOLLOUT;

		// TODO: Review https://stackoverflow.com/questions/6437879/how-do-i-use-epollhup
		/// Stream socket peer closed connection, or shut down writing half of connection.
		///
		/// This flag is especially useful for writing simple code to detect peer shutdown when using Edge Triggered monitoring.
		///
		/// Valid since Linux 2.6.17.
		/// Valid on Solaris.
		const ReadShutdown = EPOLLRDHUP;

		/// Flag to prevent suspend while epoll events are ready.
		///
		/// If `OneShort` and `EdgeTriggered` are clear and the process has the `CAP_BLOCK_SUSPEND` capability, ensures that the system does not enter "suspend" or "hibernate" while this event is pending or being processed.
		///
		/// The event is considered as being "processed" from the time when it is returned by a call to `epoll_wait()` until the next call to `epoll_wait()` on the same epoll file descriptor, the closure of that file descriptor, the removal of the event file descriptor with `delete()`, or the clearing of `WakeUp` for the event file descriptor with `modify()`.
		///
		/// Might be buggy.
		///
		/// Valid since Linux 3.5.
		const WakeUp = EPOLLWAKEUP;

		/// Sets the one-shot behavior for the associated file descriptor.
		///
		/// This means that after an event is pulled out with `epoll_wait()` the associated file descriptor is internally disabled and no other events will be reported by the epoll interface.
		///
		/// The user must call `modify()` to rearm the file descriptor with a new event mask.
		///
		/// ***In effect, use of this is expensive as it substantially increases the number of syscalls that need to be made.***
		///
		/// Valid since Linux 2.6.2.
		/// Valid on Solaris.
		const OneShot = EPOLLONESHOT;

		/// Sets the Edge Triggered behavior for the associated file descriptor.
		///
		/// The default behavior for epoll is Level Triggered.
		///
		/// Valid on Linux.
		/// Valid on Solaris.
		const EdgeTriggered = EPOLLET;
	}
}

impl Into<EPollAddFlags> for EPollModifyFlags
{
	#[inline(always)]
	fn into(self) -> EPollAddFlags
	{
		unsafe { transmute(self) }
	}
}

impl EPollModifyFlags
{
	/// Equivalent to `bits()` but constant.
	#[inline(always)]
	pub const fn const_bits(self) -> u32
	{
		self.bits
	}
}
