// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::cpu::HyperThread;
use crate::file_descriptors::directory::AccessPermissions;
use crate::file_descriptors::socket::*;
use crate::linux_kernel_version::*;
use crate::logging::rfc3164::Rfc3164MessageTemplate;
use crate::paths::DevPath;
use crate::process::ProcessName;
use crate::strings::*;
use crate::strings::to_number::NumberAsBytes;
use arrayvec::*;
use chrono::DateTime;
use chrono::Utc;
use const_fn_assert::cfn_debug_assert;
use errno::errno;
use libc::*;
use libc_extra::android_linux::stdio::cookie_io_functions_t;
use libc_extra::android_linux::stdio::fopencookie;
use libc_extra::unix::stdio::stderr;
use libc_extra::unix::stdio::stdout;
use likely::likely;
use serde::Deserialize;
use serde::Serialize;
use std::cell::Cell;
use std::convert::TryInto;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::ErrorKind;
use std::mem::forget;
use std::mem::transmute;
#[allow(deprecated)] use std::mem::uninitialized;
use std::mem::zeroed;
use std::panic::{catch_unwind, RefUnwindSafe};
use std::path::*;
use std::ptr::null_mut;
use std::slice::from_raw_parts;
use terminate::*;
use std::rc::Rc;
use std::ops::Deref;
use crate::logging::rfc5424::{Rfc5424MessageTemplate, HostName, ApplicationName, MessageIdentifier, PrintableAsciiCharacterPushError};
use std::net::IpAddr;


/// RFC 5424 syslog.
pub mod rfc3164;


/// RFC 5424 syslog.
pub mod rfc5424;


include!("Facility.rs");
include!("KnownFacility.rs");
include!("LocalSyslogSocket.rs");
include!("MessageTemplate.rs");
include!("ParsedPanicErrorLoggerProcessLoggingConfiguration.rs");
include!("PriorityValue.rs");
include!("PrivateEnterpriseNumber.rs");
include!("ProcessLoggingConfiguration.rs");
include!("Severity.rs");
include!("StaticLoggingConfiguration.rs");
include!("system_information.rs");
include!("UnknownFacility.rs");
include!("write_slice_truncated.rs");
include!("write_slice_unchecked.rs");
