// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]


//! #linux-support
//! 
//! This is a rust library.


use static_assertions::assert_cfg;


assert_cfg!(target_os = "linux");
assert_cfg!(target_arch = "x86_64");
assert_cfg!(target_pointer_width = "64");


use self::daemonize::Daemonize;
use libc::*;
use likely::unlikely;
use linux_support_common::*;
use linux_support_common::capabilities_and_privileges::*;
use linux_support_common::cpu::*;
use linux_support_common::environment::*;
use linux_support_common::linux_kernel_command_line::*;
use linux_support_common::logging::*;
use linux_support_common::niceness::*;
use linux_support_common::paths::*;
use linux_support_common::resource_limits::*;
use linux_support_common::seccomp::*;
use linux_support_common::signals::*;
use linux_support_common::transparent_huge_pages::*;
use linux_support_common::user_and_groups::*;
use maplit::btreeset;
use maplit::hashmap;
use serde::Deserialize;
use serde::Serialize;
use std::any::Any;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env::set_current_dir;
use std::error;
use std::ffi::CString;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;
use std::panic::resume_unwind;
use std::path::PathBuf;
use std::thread;


/// Daemon support.
pub mod daemonize;


/// Master loop support (incomplete).
#[allow(dead_code)]
pub(crate) mod master_loop;


/// Master process that has child clones.
#[cfg(feature = "linux-support-clone")]
pub mod master_process;


include!("DetailedProcessConfiguration.rs");
include!("DetailedProcessConfigurationError.rs");
include!("Process.rs");
include!("ProcessConfiguration.rs");
include!("ProcessConfigurationExecutionError.rs");
