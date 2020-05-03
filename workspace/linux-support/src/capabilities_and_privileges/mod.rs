// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use crate::bit_set::*;
use crate::strings::Radix;
use crate::strings::parse_number::ParseNumberError;
use crate::thread::ThreadIdentifier;
use errno::errno;
use libc::*;
use likely::*;
use libc_extra::android_linux::linux::capability;
use libc_extra::android_linux::linux::securebits::SECBIT_NOROOT;
use libc_extra::android_linux::linux::securebits::SECBIT_NOROOT_LOCKED;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_SETUID_FIXUP;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_SETUID_FIXUP_LOCKED;
use libc_extra::android_linux::linux::securebits::SECBIT_KEEP_CAPS;
use libc_extra::android_linux::linux::securebits::SECBIT_KEEP_CAPS_LOCKED;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_CAP_AMBIENT_RAISE;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED;
use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::mem::transmute;
use std::mem::zeroed;


mod c;


include!("AmbientCapabilityError.rs");
include!("AmbientCapabilitySet.rs");
include!("BoundingCapabilitySet.rs");
include!("Capability.rs");
include!("disable_dumpable.rs");
include!("lock_secure_bits_so_capabilities_are_always_enforced.rs");
include!("no_new_privileges.rs");
include!("PermittedEffectiveAndInheritableCapabilitySets.rs");
include!("set_io_flusher.rs");
