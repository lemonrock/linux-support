// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


bitflags!
{
	/// Flags to use for epoll `add()`.
	pub struct EPollAddFlags: u32
	{
		/// The associated file descriptor is available for read operations.
		const Input = EPOLLIN;

		/// The associated file descriptor has urgent or out-of-band data available.
		///
		/// For TCP socket file descriptors, this usually means that out-of-band data (which is a deprecated concept) is available.
		const InputPriority = EPOLLPRI;

		/// The associated file descriptor is available for write operations.
		const Output = EPOLLOUT;

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

		/// Flag for exclusive wakeup mode when an event source fd is attached to multiple epoll fds but they should not all receive the events.
		///
		/// Sets an exclusive wakeup mode for the epoll file descriptor that is being attached to the target file descriptor, `fd`.
		/// When a wakeup event occurs and multiple epoll file descriptors are attached to the same target file using `Exclusive`, one or more of the epoll file descriptors will receive an event with `epoll_wait()`.
		/// The default in this scenario (when `Exclusive` is not set) is for all epoll file descriptors to receive an event.
		/// `Exclusive` is thus useful for avoiding thundering herd problems in certain scenarios.
		///
		/// If the same file descriptor is in multiple epoll instances, some with the `Exclusive` flag, and others without, then events will be provided to all epoll instances that did not specify `Exclusive`, and at least one of the epoll instances that did specify `Exclusive`.
		///
		/// The following values may be specified in conjunction with `Exclusive`: `Input`, `Output`, `WakeUp`, and `EdgeTriggered`.
		/// Attempts to specify other values in `events` yield an error.
		/// `Exclusive` may be used only in an `add()` operation; attempts to employ it with `modify()` yield an error.
		/// If `Exclusive` has been set using `epoll_ctl()`, then a subsequent `modify()` on the same `epfd`, `fd` pair yields an error.
		/// A call to `epoll_ctl()` that specifies `Exclusive` in `events` and specifies the target file descriptor `fd` as an epoll instance will likewise fail.
		/// The error in all of these cases is `EINVAL`.
		///
		/// Valid since Linux 4.5.
		/// Valid on Solaris.
		const Exclusive = EPOLLEXCLUSIVE;

		/// Used for signalfd, eventfd, timerfd and similar file descriptors.
		const EdgeTriggeredInput = EPOLLIN | EPOLLET;

		/// Used for listening sockets.
		const EdgeTriggeredInputExclusive = EPOLLIN | EPOLLET | EPOLLEXCLUSIVE;

		/// Used for TCP streaming sockets.
		const Streaming = EPOLLIN | EPOLLPRI | EPOLLOUT | EPOLLRDHUP | EPOLLET;
	}
}

impl Into<EPollModifyFlags> for EPollAddFlags
{
	#[inline(always)]
	fn into(mut self) -> EPollModifyFlags
	{
		self.remove(EPollAddFlags::Exclusive);
		unsafe { transmute(self) }
	}
}
