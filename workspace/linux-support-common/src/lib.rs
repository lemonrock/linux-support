// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]


//! #linux-support-common
//! 
//! This library provides wrappers and additional functionality to make use of a panoply of miscellaneous Linux (and, sometimes, POSIX) features.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_arch = "x86_64");
assert_cfg!(target_pointer_width = "64");


use crate::logging::LoggingConfiguration;
use libc_extra::linux::errno::program_invocation_short_name;
use raw_cpuid::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::ffi::CStr;
use std::fmt::Debug;


/// Capabilities and privileges.
pub mod capabilities_and_privileges;


/// Cgroups (containers).
pub mod cgroups;


/// CPU.
pub mod cpu;


/// Environment variables.
pub mod environment;


/// File systems.
pub mod file_systems;


/// Inode.
pub mod inode;


/// Linux Kernel command line.
pub mod linux_kernel_command_line;


/// Linux Kernel modules.
pub mod linux_kernel_modules;


/// Logging.
pub mod logging;

/// Mounts.
pub mod mounts;


/// Namespaces.
pub mod namespaces;


/// Niceness.
pub mod niceness;


/// Paths.
pub mod paths;


/// Resource limits.
pub mod resource_limits;


/// Seccomp.
pub mod seccomp;


/// Signals.
pub mod signals;


/// Status.
pub mod status;


/// Strings.
pub mod strings;


/// Transparent Huge Pages.
pub mod transparent_huge_pages;


/// User and groups.
pub mod user_and_groups;


include!("get_program_name.rs");
include!("WarningsToSuppress.rs");
