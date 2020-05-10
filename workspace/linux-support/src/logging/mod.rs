// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::cpu::HyperThread;
use crate::process::ProcessName;
use libc::*;
use libc_extra::android_linux::stdio::cookie_io_functions_t;
use libc_extra::android_linux::stdio::cookie_write_function_t;
use libc_extra::android_linux::stdio::fopencookie;
use libc_extra::unix::stdio::stderr;
use libc_extra::unix::stdio::stdout;
use serde::Deserialize;
use serde::Serialize;
use std::env::set_var;
use std::ffi::CString;
use std::mem::forget;
use std::panic::catch_unwind;
use std::ptr::null_mut;
use std::slice::from_raw_parts;
use terminate::*;


include!("ParsedPanicErrorLoggerProcessLoggingConfiguration.rs");
include!("ProcessLoggingConfiguration.rs");
include!("SyslogFacility.rs");
include!("SyslogPriority.rs");
