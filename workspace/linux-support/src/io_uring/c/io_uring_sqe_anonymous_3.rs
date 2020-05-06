// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Copy, Clone)]
#[repr(C)]
pub(super) union io_uring_sqe_anonymous_3
{
	/// This is specified for read and write operations.
	///
	/// It contains a bitwise OR of per-I/O flags, as described in the `preadv2()` man page.
	pub(super) rw_flags: __kernel_rwf_t,

	/// See the descriptions of `O_SYNC` and `O_DSYNC` in the `open()` manual page for more information.
	///
	/// May be empty, in which a normal `O_SYNC` is performed.
	pub(super) fsync_flags: fsync_flags,

	/// The bits that may be set are defined in `<poll.h>`, and documented in `poll()`.
	pub(super) poll_events: u16,

	pub(super) sync_range_flags: u32,

	pub(super) msg_flags: u32,

	pub(super) timeout_flags: timeout_flags,

	pub(super) accept_flags: u32,

	pub(super) cancel_flags: u32,

	pub(super) open_flags: u32,

	pub(super) statx_flags: u32,

	pub(super) fadvise_advice: u32,

	pub(super) splice_flags: splice_flags,
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
		write!(f, "io_uring_sqe_anonymous_3({:?})", unsafe { self.fsync_flags })
	}
}
