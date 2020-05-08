// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use crate::process::ProcessIdentifierChoice;
use crate::process::ProcessGroupIdentifierChoice;
use crate::user_and_groups::UserIdentifier;
use errno::errno;
use libc::EINVAL;
use libc::EPERM;
use libc::ESRCH;
use likely::*;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::mem::transmute;
use crate::thread::ThreadIdentifier;


mod c;


include!("CompressedIoPriority.rs");
include!("IoPriority.rs");
include!("RealTimeOrBestEffortIoPriorityLevel.rs");
