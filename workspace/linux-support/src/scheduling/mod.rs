// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::c::*;
use crate::paths::ProcPath;
use crate::paths::PathExt;
use crate::process::ProcessIdentifierChoice;
use crate::process::ProcessGroupIdentifierChoice;
use crate::strings::Radix;
use crate::strings::into_line_feed_terminated_byte_string::UnpaddedDecimalInteger;
use crate::strings::parse_number::*;
use crate::thread::ThreadIdentifier;
use crate::thread::ThreadIdentifierChoice;
use crate::user_and_groups::UserIdentifier;
use crate::user_and_groups::assert_effective_user_id_is_root;


mod c;


include!("Nice.rs");include!("PerThreadSchedulerPolicyAndFlags.rs");
include!("RealTimePriority.rs");
include!("ReservedCpuTimeForNonRealTimeSchedulerPolicies.rs");
include!("RoundRobinInterval.rs");
include!("RoundRobinQuantumMilliseconds.rs");
include!("SchedulerPolicy.rs");
include!("SchedulerPolicyFlags.rs");
