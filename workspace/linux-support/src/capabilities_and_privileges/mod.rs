// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::c::*;
use crate::thread::ThreadIdentifier;


mod c;


include!("AmbientCapabilityError.rs");
include!("AmbientCapabilitySet.rs");
include!("BoundingCapabilitySet.rs");
include!("Capabilities.rs");
include!("Capability.rs");
include!("disable_dumpable.rs");
include!("lock_secure_bits_so_capabilities_are_always_enforced.rs");
include!("no_new_privileges.rs");
include!("PermittedEffectiveAndInheritableCapabilitySets.rs");
include!("set_io_flusher.rs");
