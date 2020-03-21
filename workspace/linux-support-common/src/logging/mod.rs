// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::get_program_name;
use super::cpu::HyperThread;
use super::signals::SignalNumber;
use super::strings::ConstCStr;
use super::strings::to_c_string_robustly;
use libc::*;
use libc_extra::android_linux::stdio::cookie_io_functions_t;
use libc_extra::android_linux::stdio::cookie_write_function_t;
use libc_extra::android_linux::stdio::fopencookie;
use libc_extra::linux::errno::program_invocation_short_name;
use libc_extra::unix::stdio::stderr;
use libc_extra::unix::stdio::stdout;
use libc_extra::unix::string::strsignal;
use serde::Deserialize;
use serde::Serialize;
use std::any::Any;
use std::env::set_var;
use std::ffi::CString;
use std::panic::set_hook;
use std::panic::take_hook;
use std::ptr::null_mut;


include!("caught_unwind_and_log_it_to_syslog.rs");
include!("log_exit_signalled_to_syslog.rs");
include!("LoggingConfiguration.rs");
include!("panic_payload_to_cause.rs");
include!("redirect_to_syslog.rs");
include!("SyslogFacility.rs");
include!("SyslogPriority.rs");
include!("redirect_standard_out_and_standard_error_to_syslog.rs");
include!("write_standard_error_to_syslog.rs");
include!("write_standard_out_to_syslog.rs");
include!("write_to_syslog.rs");
