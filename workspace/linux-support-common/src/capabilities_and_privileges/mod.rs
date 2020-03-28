// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::bit_set::BitSetAware;
use crate::bit_set::BitSetAwareTryFromU16Error;
use errno::errno;
use libc::*;
use likely::unlikely;
use libc_extra::android_linux::linux::capability;
use libc_extra::android_linux::linux::securebits::SECBIT_NOROOT;
use libc_extra::android_linux::linux::securebits::SECBIT_NOROOT_LOCKED;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_SETUID_FIXUP;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_SETUID_FIXUP_LOCKED;
use libc_extra::android_linux::linux::securebits::SECBIT_KEEP_CAPS_LOCKED;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_CAP_AMBIENT_RAISE;
use libc_extra::android_linux::linux::securebits::SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED;
use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::mem::transmute;


include!("Capability.rs");
include!("disable_dumpable.rs");
include!("lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities.rs");
include!("no_new_privileges.rs");
