// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Passed in for `io_uring_setup()`.
/// Copied back with updated info on success
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(super) struct io_uring_params
{
	pub(super) sq_entries: u32,

	pub(super) cq_entries: u32,

	pub(super) flags: SetupFlags,

	pub(super) sq_thread_cpu: u32,

	pub(super) sq_thread_idle: u32,

	pub(super) features: ParametersFeatureFlags,

	/// This is the file descriptor of another io_uring.
	pub(super) wq_fd: RawFd,

	pub(super) resv: [u32; 3],

	pub(super) sq_off: io_sqring_offsets,

	pub(super) cq_off: io_cqring_offsets,
}
