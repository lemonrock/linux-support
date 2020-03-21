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


include!("Capability.rs");
include!("disable_dumpable.rs");
include!("lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities.rs");
include!("no_new_privileges.rs");
