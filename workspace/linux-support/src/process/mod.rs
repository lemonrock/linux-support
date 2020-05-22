// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(deprecated)] use std::mem::uninitialized;
use crate::bit_set::*;
use crate::capabilities_and_privileges::Capability;
use crate::cpu::*;
use crate::file_descriptors::CreationError;
use crate::file_descriptors::process_identifier::ProcessIdentifierFileDescriptor;
use crate::file_descriptors::standard::StandardFileDescriptor;
use crate::memory::numa::*;
use crate::paths::*;
use crate::scheduling::RoundRobinInterval;
use crate::signals::Signal;
use crate::strings::*;
use crate::strings::into_line_feed_terminated_byte_string::*;
use crate::strings::parse_number::*;
use crate::thread::ThreadIdentifier;
use crate::vectors::*;
use arrayvec::ArrayVec;
use errno::errno;
use indexmap::set::IndexSet;
use libc::EFAULT;
use libc::EINVAL;
use libc::ENOMEM;
use libc::EPERM;
use libc::ESRCH;
use libc::fork;
use libc::getpgid;
use libc::getpid;
use libc::getsid;
use libc::iovec;
use libc::mode_t;
use libc::pid_t;
use libc::process_vm_readv;
use libc::process_vm_writev;
use libc::setsid;
use libc::strnlen;
use libc_extra::linux::errno::program_invocation_short_name;
use likely::*;
use likely::unlikely;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;
use serde::de::Visitor;
use serde::Serializer;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error;
use std::ffi::CStr;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::io::IoSliceMut;
use std::io::Initializer;
use std::io::IoSlice;
use std::io::Read;
use std::io::stdin;
use std::io::stdout;
use std::io::stderr;
use std::io::Write;
use std::mem::transmute;
use std::num::NonZeroI32;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::process;
use std::ptr::write;
use std::slice::from_raw_parts;


/// `/proc/<N>/stat`.
pub mod stat;


/// `/proc/<N>/statm`.
pub mod statm;


/// `/proc/<N>/status`.
pub mod status;


include!("CommandName.rs");
include!("CommandNameFromBytesError.rs");
include!("daemonize.rs");
include!("get_program_invocation_short_name.rs");
include!("ProcessGroupIdentifier.rs");
include!("ProcessGroupIdentifierChoice.rs");
include!("ProcessIdentifier.rs");
include!("ProcessIdentifierChoice.rs");
include!("ProcessIdentifierVectoredRead.rs");
include!("ProcessIdentifierVectoredWrite.rs");
include!("ProcessName.rs");
include!("ProcessState.rs");
