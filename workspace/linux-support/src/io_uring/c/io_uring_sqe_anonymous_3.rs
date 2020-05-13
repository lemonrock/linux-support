// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Copy, Clone)]
#[repr(C)]
pub(super) union io_uring_sqe_anonymous_3
{
	/// This is specified for read and write operations.
	pub(super) rw_flags: __kernel_rwf_t,
	pub(super) read_vectored_flags: ReadVectoredFlags,
	pub(super) write_vectored_flags: WriteVectoredFlags,

	/// See the descriptions of `O_SYNC` and `O_DSYNC` in the `open()` manual page for more information.
	///
	/// May be empty, in which case a normal `O_SYNC` is performed.
	pub(super) fsync_flags: FileSynchronize,

	/// The bits that may be set are defined in `<poll.h>`, and documented in `poll()`.
	pub(super) poll_events: PollRequestFlags,

	pub(super) sync_range_flags: SynchronizeFileRangeFlags,

	/// Can be either of:-
	///
	/// * `ReceiveFlags`.
	/// * `SendFlags`.
	pub(super) msg_flags: SendOrReceiveFlags,

	pub(super) timeout_flags: RelativeOrAbsoluteTimeout,

	/// `SOCK_*` flags such as:-
	///
	/// * `SOCK_NONBLOCK`.
	/// * `SOCK_CLOEXEC`.
	pub(super) accept_flags: u32,

	/// Always `0`.
	pub(super) cancel_flags: u32,

	pub(super) open_flags: u32,

	pub(super) statx_flags: u32,

	pub(super) fadvise_advice: FileOrMemoryAdvice,

	/// splice_flags are:-
	///
	/// * `SPLICE_F_MOVE`.
	/// * `SPLICE_F_NONBLOCK`.
	/// * `SPLICE_F_MORE`.
	/// * `SPLICE_F_FD_IN_FIXED`.
	pub(super) splice_flags: u32,
}

impl Default for io_uring_sqe_anonymous_3
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for io_uring_sqe_anonymous_3
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "io_uring_sqe_anonymous_3({:?})", unsafe { self.rw_flags })
	}
}

impl PartialEq for io_uring_sqe_anonymous_3
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.rw_flags == other.rw_flags }
	}
}

impl Eq for io_uring_sqe_anonymous_3
{
}

impl PartialOrd for io_uring_sqe_anonymous_3
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.rw_flags.partial_cmp(&other.rw_flags) }
	}
}

impl Ord for io_uring_sqe_anonymous_3
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.rw_flags.cmp(&other.rw_flags) }
	}
}

impl Hash for io_uring_sqe_anonymous_3
{
	#[inline(always)]
	fn hash<H>(&self, state: &mut H) where H: Hasher
	{
		unsafe { self.rw_flags.hash(state) }
	}
}
