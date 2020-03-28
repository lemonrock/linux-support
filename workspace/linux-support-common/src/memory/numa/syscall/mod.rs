// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::syscall::SYS;
use super::GetMemoryPolicyFlags;
use bitflags::bitflags;
use libc::c_void;


include!("get_mempolicy.rs");
include!("mbind.rs");
include!("migrate_pages.rs");
include!("move_pages.rs");
include!("MemoryBindFlags.rs");
include!("set_mempolicy.rs");
