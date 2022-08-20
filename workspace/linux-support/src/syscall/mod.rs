// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::bpf::c::bpf_attr;
use crate::bpf::c::bpf_cmd;
use crate::file_descriptors::directory::DirectoryFileDescriptor;
use crate::file_descriptors::directory::c::open_how;
use crate::file_descriptors::directory::c::statx;
use crate::io_priority::c::IOPRIO_WHO;
use crate::io_uring::c::EnterFlags;
use crate::io_uring::c::io_uring_params;
use crate::io_uring::c::RegisterOperation;
use crate::perf_event::c::perf_event_attr;
use crate::scheduling::c::sched_attr;
use crate::signals::c::_NSIG;
use crate::memory::numa::GetMemoryPolicyFlags;
use crate::memory::numa::c::MemoryBindFlags;
use libc::INT_MAX;
use crate::file_descriptors::directory::c::dirent;
use std::ops::Neg;
use super::*;
use swiss_army_knife::from_unchecked::FromUnchecked;
use swiss_army_knife::get_unchecked::AsUsizeIndex;
use swiss_army_knife::non_zero::new_non_zero_i16;
use swiss_army_knife::non_zero::new_non_zero_i32;
use swiss_army_knife::non_zero::new_non_zero_i64;
use swiss_army_knife::non_zero::new_non_zero_i128;
use swiss_army_knife::non_zero::new_non_zero_isize;
use swiss_army_knife::non_zero::new_non_zero_u32;
use swiss_army_knife::non_zero::new_non_zero_u64;
use swiss_army_knife::non_zero::new_non_zero_u128;
use swiss_army_knife::non_zero::new_non_zero_usize;


include!("system_call_.rs");
include!("SystemCallArguments.rs");
include!("SystemCallErrorNumber.rs");
include!("SystemCallNumber.rs");
include!("SystemCallResult.rs");
include!("SystemCallResultOkValue.rs");
include!("UnconstrainedSystemCallNumber.rs");
