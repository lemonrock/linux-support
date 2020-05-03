// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![feature(alloc_layout_extra)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(backtrace)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(const_ptr_offset_from)]
#![feature(const_raw_ptr_deref)]
#![feature(const_transmute)]
#![feature(never_type)]
#![feature(ptr_offset_from)]
#![feature(read_initializer)]


//! #linux-support
//! 
//! This library provides wrappers and additional functionality to make use of a panoply of miscellaneous Linux (and, sometimes, POSIX) features.
//!
//! See <https://github.com/lemonrock/linux-support> for far more detail.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use crate::bit_set::BitSetAware;
use crate::cpu::HyperThread;
use crate::memory::numa::NumaNode;
use crate::strings::Radix;
use crate::strings::parse_number::*;
use libc::clock_t;
use std::fmt::Debug;
use std::mem::transmute;


/// A set of types to support the use of bit sets in Linux APIs and files.
#[macro_use]
pub mod bit_set;


/// Vectored reads and writes.
#[macro_use]
pub mod vectors;


/// Berkeley Packet Filter (BPF) and Extended Berkeley Packet Filter (eBPF).
pub mod bpf;


/// Capabilities and privileges.
///
/// * Manage capability sets for security.
/// * Disable the 'dumpable' flag for security.
/// * Lock down a process to remove privileges.
pub mod capabilities_and_privileges;


/// Cgroups (containers).
pub mod cgroups;


/// Configuration.
pub mod configuration;


/// CPU.
///
/// * Cpu features wrapper.
/// * A proper CPU count that takes into account NUMA nodes, hotplugs, etc.
/// * Hyper thread (SMT) insight, status, usage, etc.
/// 	* Turn off and on
/// 	* Mappings to NUMA nodes
/// 	* And lots more
pub mod cpu;


/// Block and character device abstractions.
pub mod devices;


/// Environment variables.
///
/// * Find the original environment of a process.
/// * Find the command line of a process.
/// * Create a clean environment for a process with just essential variables set (a security and reproducibility protection).
pub mod environment;


/// Extended file attributes.
pub mod extended_attributes;


pub mod file_descriptors;


/// File handles.
pub mod file_handles;


/// File systems.
pub mod file_systems;


/// Inode.
///
/// A wrapper type for Inodes.
pub mod inode;


/// Basic (for security) access io I/O ports on mip64, powerpc64 and x86_64.
#[cfg(any(target_arch = "mips64", target_arch = "powerpc64", target_arch = "x86_64"))] pub mod ioports;


/// Linux kernel command line.
pub mod linux_kernel_command_line;


/// Also known as `KAIO`.
///
/// Support for functions such as `io_submit()` in `linuxaio.h`.
///
/// This is *NOT* POSIX AIO.
pub mod linux_kernel_asynchronous_io;


/// Linux kernel modules.
pub mod linux_kernel_modules;


/// Linux kernel panic.
pub mod linux_kernel_panic;


/// Linux kernel version.
pub mod linux_kernel_version;


/// Logging.
///
/// Miscellany support for using syslog with a Rust process, including:-
///
/// * Redirecting standard out and standard error to syslog;
/// * Logging process terminating signals to syslog.
/// * Logging panics to syslog.
/// * Configuring syslog.
pub mod logging;


/// Memory.
///
/// * Detailed, comprehensive and insightful NUMA node level information.
/// * Proper, modern Linux support for huge pages and mapping huge pages into memory.
/// * Memory usage and insight.
/// * A Linux-specific wrapper for mmap and related functionality that makes it *much* harder to misconfigure.
/// * Wrapper types for virtual and physical addreses.
/// * Wrapper types for number of pages.
/// * Efficient enums for page size and huge page sizes.
/// * Insight into memory maps
/// 	* For finding physical addresses from virtual memory addresses
pub mod memory;


/// Mounts.
pub mod mounts;


/// Namespaces.
pub mod namespaces;


/// Nice.
pub mod scheduling;


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


/// Process.
pub mod process;


/// Swap.
pub mod swap;


/// Strings.
pub mod strings;


/// Support for raw syscalls.
pub mod syscall;


/// Support for terminals.
pub mod terminal;


/// Support for threads.
pub mod thread;


/// User and groups.
pub mod user_and_groups;


include!("ClockTicks.rs");
include!("current_numa_node_and_hyper_thread.rs");
include!("move_to_front_of_vec.rs");
