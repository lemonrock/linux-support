// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(alloc_layout_extra)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(const_transmute)]


//! #linux-support-common
//! 
//! This library provides wrappers and additional functionality to make use of a panoply of miscellaneous Linux (and, sometimes, POSIX) features.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_arch = "x86_64");
assert_cfg!(target_pointer_width = "64");


use crate::bit_set::BitSetAware;
use crate::cpu::HyperThread;
use crate::logging::LoggingConfiguration;
use crate::memory::numa::NumaNode;
use libc_extra::linux::errno::program_invocation_short_name;
use raw_cpuid::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::ffi::CStr;
use std::fmt::Debug;
use std::mem::transmute;


/// A set of types to support the use of bit sets in Linux APIs and files.
pub mod bit_set;


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


/// Memory
pub mod memory;


/// Mounts.
pub mod mounts;


/// Namespaces.
pub mod namespaces;


/// Niceness.
pub mod niceness;


/// Paths.
pub mod paths;


/// Linux personality.
///
/// A mostly broken and discarded concept, but we should check we're running as a standard Linux process.
pub mod personality;


/// PCI Express (PCIe).
pub mod pci_express;


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


/// Support for raw syscalls.
pub mod syscall;


/// User and groups.
pub mod user_and_groups;


include!("current_numa_node_and_hyper_thread.rs");
include!("get_program_name.rs");
include!("Kilobyte.rs");
include!("move_to_front_of_vec.rs");
include!("WarningsToSuppress.rs");
