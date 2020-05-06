// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::PersonalityCredentialsIdentifier;
use crate::signals::c::_NSIG;
use crate::syscall::SYS;
use bitflags::bitflags;
use libc::c_int;
use libc::c_uint;
use libc::c_void;
use libc::sigset_t;
use std::cmp::Ordering;
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::hash::Hash;
use std::hash::Hasher;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
#[allow(deprecated)] use std::mem::uninitialized;
use std::mem::zeroed;
use std::os::unix::io::RawFd;


include!("__kernel_rwf_t.rs");
include!("CompletionQueueEntryFlags.rs");
include!("EnterFlags.rs");
include!("fsync_flags.rs");
include!("io_cqring_offsets.rs");
include!("io_sqring_offsets.rs");
include!("io_uring_cqe.rs");
include!("io_uring_enter.rs");
include!("io_uring_files_update.rs");
include!("IO_URING_OP_.rs");
include!("io_uring_params.rs");
include!("io_uring_probe.rs");
include!("io_uring_probe_op.rs");
include!("io_uring_register.rs");
include!("io_uring_setup.rs");
include!("io_uring_sqe.rs");
include!("io_uring_sqe_anonymous_1.rs");
include!("io_uring_sqe_anonymous_2.rs");
include!("io_uring_sqe_anonymous_3.rs");
include!("io_uring_sqe_anonymous_4.rs");
include!("io_uring_sqe_anonymous_4_anonymous_1.rs");
include!("io_uring_sqe_anonymous_4_anonymous_1_anonymous_1.rs");
include!("IORING_CQE.rs");
include!("IORING_CQE_F_.rs");
include!("IORING_ENTER_.rs");
include!("IORING_FEAT_.rs");
include!("IORING_FSYNC_.rs");
include!("IORING_OFF_.rs");
include!("IORING_OP.rs");
include!("IORING_REGISTER_.rs");
include!("IORING_SETUP_.rs");
include!("IORING_SQ_.rs");
include!("IORING_TIMEOUT_.rs");
include!("IOSQE.rs");
include!("IOSQE_.rs");
include!("ParametersFeatureFlags.rs");
include!("ProbeFlags.rs");
include!("RegisterOperation.rs");
include!("SPLICE_F_.rs");
include!("splice_flags.rs");
include!("SetupFlags.rs");
include!("SubmissionQueueEntryFlags.rs");
include!("timeout_flags.rs");


#[repr(transparent)]
pub(super) struct SubmissionQueueEntry(io_uring_sqe);

#[repr(transparent)]
pub(super) struct CompletionQueueEntry(io_uring_cqe);


bitflags!
{
	/// `sq_ring->flags`.
	pub(super) struct SubmissionQueueRingFlags: u32
	{
		const NeedsIoUringEnterWakeUp = IORING_SQ_NEED_WAKEUP;
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(super) struct io_sq_ring
{
	pub(super) head: *mut c_int,

	pub(super) tail: *mut c_int,

	pub(super) ring_mask: *mut c_int,

	pub(super) ring_entries: *mut c_int,

	pub(super) flags: *mut SubmissionQueueRingFlags,

	pub(super) array: *mut c_int,
}

impl io_sq_ring
{
	#[inline(always)]
	pub(super) fn needs_submission_queue_wake_up(&self) -> bool
	{
		(unsafe { *self.flags }).contains(SubmissionQueueRingFlags::NeedsIoUringEnterWakeUp)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct io_cq_ring
{
	pub(super) head: *mut c_int,

	pub(super) tail: *mut c_int,

	pub(super) ring_mask: *mut c_int,

	pub(super) ring_entries: *mut c_int,

	pub(super) cqes: *mut io_uring_cqe,
}
