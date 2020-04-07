// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(deprecated)] use std::mem::uninitialized;
use crate::ClockTicks;
use crate::bit_set::*;
use crate::memory::VirtualAddress;
use crate::process::ProcessIdentifier;
use crate::process::UserIdentifier;
use crate::process::status::StatusStatisticParseError;
use crate::strings::{FromBytes, Radix};
use crate::strings::parse_number::*;
use crate::syscall::UnconstrainedSystemCallNumber;
use self::codes::*;
use self::syscall::*;
use errno::errno;
use libc::*;
use libc_extra::unix::string::strsignal;
use likely::*;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::error;
use std::ffi::CStr;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::mem::transmute;
use std::num::NonZeroI32;
use std::num::NonZeroU32;
use std::num::NonZeroU8;
use std::os::unix::io::RawFd;
use std::ptr::null_mut;
use strum_macros::EnumIter;


/// `si_code` ranges of values.
pub mod codes;


/// System call and libc wrapping of system call specific details.
pub mod syscall;


include!("AuditArchitecture.rs");
include!("BitSetSignal.rs");
include!("ChildStatus.rs");
include!("ElfMachine.rs");
include!("FaultCode.rs");
include!("one_millisecond_timed_wait_for_signals.rs");
include!("OutOfRangeSignalNumberError.rs");
include!("ParsedSignal.rs");
include!("Signal.rs");
include!("SignalQueueStatus.rs");
include!("TimedSignalWait.rs");
