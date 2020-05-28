// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::memory::mapping::MemoryAdvice;
use crate::signals::c::_NSIG;
use crate::syscall::SYS;


include!("__kernel_rwf_t.rs");
include!("__kernel_time64_t.rs");
include!("__kernel_timespec.rs");
include!("CompletionQueueEntryFlags.rs");
include!("EnterFlags.rs");
include!("FileOrMemoryAdvice.rs");
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
include!("SetupFlags.rs");
include!("SubmissionQueueRingFlags.rs");
