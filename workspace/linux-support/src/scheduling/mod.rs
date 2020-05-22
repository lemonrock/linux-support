// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use crate::paths::ProcPath;
use crate::paths::PathExt;
use crate::process::ProcessIdentifierChoice;
use crate::process::ProcessGroupIdentifierChoice;
use crate::strings::Radix;
use crate::strings::into_line_feed_terminated_byte_string::UnpaddedDecimalInteger;
use crate::strings::parse_number::*;
use crate::thread::ThreadIdentifierChoice;
use crate::thread::ThreadIdentifier;
use bitflags::bitflags;
use errno::errno;
use libc::E2BIG;
use libc::EACCES;
use libc::EBUSY;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOSYS;
use libc::EPERM;
use libc::ESRCH;
use libc::pid_t;
use libc::PRIO_PGRP;
use libc::PRIO_PROCESS;
use libc::PRIO_USER;
use libc::setpriority;
use libc::sched_rr_get_interval;
use libc::uid_t;
use likely::*;
use serde::Deserialize;
use serde::Serialize;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::{io, fmt};
#[allow(deprecated)] use std::mem::uninitialized;
use std::mem::transmute;
use std::num::NonZeroU8;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::time::Duration;
use std::io::ErrorKind;
use crate::user_and_groups::{UserIdentifier, assert_effective_user_id_is_root};


mod c;


include!("Nice.rs");
include!("PerThreadSchedulerPolicyAndFlags.rs");
include!("RealTimePriority.rs");
include!("ReservedCpuTimeForNonRealTimeSchedulerPolicies.rs");
include!("RoundRobinInterval.rs");
include!("RoundRobinQuantumMilliseconds.rs");
include!("SchedulerPolicy.rs");
include!("SchedulerPolicyFlags.rs");
