// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use crate::bpf::BpfProgram;
use crate::bpf::c::*;
use crate::capabilities_and_privileges::no_new_privileges;
use crate::signals::AuditArchitecture;
use crate::syscall::SYS;
use crate::thread::ThreadIdentifier;
use bitflags::bitflags;
use errno::errno;
use indexmap::indexset;
use indexmap::set::IndexSet;
use libc::EOPNOTSUPP;
use libc::prctl;
use likely::*;
use memoffset::offset_of;
use serde::Deserialize;
use serde::Serialize;
use std::io;
use std::ptr::null_mut;
use std::num::NonZeroI32;
use std::ops::*;
use crate::file_descriptors::seccomp_user_notification::SeccompUserNotificationFileDescriptor;


/// C definitions.
pub mod c;


/// libseccomp backed filtering.
#[cfg(feature = "libseccomp")]
pub mod libseccomp;


include!("disabled_seccomp.rs");
include!("PermittedSyscalls.rs");
include!("SeccompProgram.rs");
include!("strict_seccomp.rs");
include!("SyscallOutcome.rs");
include!("UserNotificationFlags.rs");
