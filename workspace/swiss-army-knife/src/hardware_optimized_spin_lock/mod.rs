// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::intel_hardware_lock_elision::__hle_acquire_exchange_n1;
use crate::intel_hardware_lock_elision::__hle_release_store_n1;


include!("AtomicBoolSpinLock.rs");
include!("BestForCompilationTargetSpinLock.rs");
include!("busy_wait_spin_loop_hint.rs");
#[cfg(target_arch = "x86_64")] include!("IntelHardwareOptimizedLockSpinLock.rs");
include!("SpinLock.rs");
