// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::c::*;
use super::PageSize;
use super::huge_pages::*;
use super::information::*;
use crate::cpu::HyperThread;
use crate::cpu::HyperThreads;
use crate::current_numa_node_and_hyper_thread;
use crate::paths::*;
use crate::process::*;
use crate::process::status::Status;
use crate::syscall::SystemCallNumber;
use crate::user_and_groups::assert_effective_user_id_is_root;


pub(crate) mod c;


include!("GetMemoryPolicyFlags.rs");
include!("MemoryLatencyRelativeCost.rs");
include!("NumaNode.rs");
include!("NumaNodes.rs");
include!("NumaZoneReclaimMode.rs");
include!("PageMoveError.rs");
include!("PageMoveStatus.rs");
include!("PerPageMoveError.rs");
include!("MemoryPolicy.rs");
include!("MemoryPolicyDynamism.rs");
include!("SetMemoryPolicy.rs");
include!("SetMemoryPolicyStrictness.rs");
