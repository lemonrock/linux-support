// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(deprecated)] use std::mem::uninitialized;
use crate::bit_set::*;
use crate::capabilities_and_privileges::Capability;
use crate::cpu::*;
use crate::file_descriptors::CreationError;
use crate::file_descriptors::process_identifier::ProcessIdentifierFileDescriptor;
use crate::memory::numa::*;
use crate::paths::*;
use crate::signals::Signal;
use crate::strings::*;
use crate::strings::parse_number::*;
use errno::errno;
use indexmap::set::IndexSet;
use libc::EFAULT;
use libc::EPERM;
use libc::ESRCH;
use libc::getegid;
use libc::geteuid;
use libc::getgid;
use libc::getpgid;
use libc::getpid;
use libc::getresgid;
use libc::getresuid;
use libc::getsid;
use libc::getuid;
use libc::gid_t;
use libc::mode_t;
use libc::pid_t;
use libc::uid_t;
use libc_extra::linux::errno::program_invocation_short_name;
use likely::*;
use likely::unlikely;
use self::status::*;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error;
use std::ffi::CStr;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io;
use std::num::NonZeroI32;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::ptr::write;


/// `/proc/<N>/stat`.
pub mod stat;


/// `/proc/<N>/statm`.
pub mod statm;


/// `/proc/<N>/status`.
pub mod status;


include!("get_program_name.rs");
include!("GroupIdentifier.rs");
include!("GroupIdentifiers.rs");
include!("Groups.rs");
include!("ProcessGroupIdentifier.rs");
include!("ProcessGroupIdentifierChoice.rs");
include!("ProcessIdentifier.rs");
include!("ProcessIdentifierChoice.rs");
include!("ProcessState.rs");
include!("UserOrGroupIdentifier.rs");
include!("UserIdentifier.rs");
include!("UserIdentifiers.rs");
