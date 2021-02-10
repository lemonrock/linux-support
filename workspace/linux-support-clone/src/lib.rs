// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![feature(slice_ptr_get)]


//! #linux-support-clone
//! 
//! This library provides support for cloned processes in Linux.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use bitflags::bitflags;
use errno::errno;
use libc::*;
use likely::*;
use linux_support::file_descriptors::CreationError;
use linux_support::file_descriptors::FileDescriptor;
use linux_support::file_descriptors::pipes_and_fifos::PipeFileDescriptor;
use linux_support::file_descriptors::pipes_and_fifos::ReceivePipeFileDescriptor;
use linux_support::file_descriptors::pipes_and_fifos::SendPipeFileDescriptor;
use linux_support::file_systems::FileSystemType;
use linux_support::mounts::*;
use linux_support::mounts::mount_wrapper;
use linux_support::namespaces::setup_uts_namespace;
use linux_support::namespaces::write_uid_and_gid_maps;
use linux_support::paths::ProcPath;
use linux_support::process::ProcessIdentifier;
use linux_support::signals::Signal;
use serde::Deserialize;
use serde::Serialize;
use std::alloc::Layout;
use std::alloc::Allocator;
use std::alloc::AllocError;
use std::env::set_current_dir;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::remove_dir_all;
use std::io;
use std::io::Read;
use std::io::Write;
use std::mem::transmute;
use std::num::NonZeroI32;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::ptr::NonNull;
use std::ptr::null_mut;
use std::str::FromStr;
use swiss_army_knife::hash_map_and_hash_set::FastSecureHashMap as HashMap;
use swiss_army_knife::hash_map_and_hash_set::FastSecureHashSet as HashSet;
use swiss_army_knife::non_zero::new_non_null;
use swiss_army_knife::non_zero::new_non_zero_i32;
use swiss_army_knife::strings::ConstCStr;
use swiss_army_knife::strings::path_to_cstring;


include!("clone_child_process_in_new_namespace.rs");
include!("ChildProcessFunction.rs");
include!("ChildStackAlignment.rs");
include!("ChildTerminationSignal.rs");
include!("clone.rs");
include!("clone_wrapper.rs");
include!("CloneError.rs");
include!("CloneFlags.rs");
include!("CouldNotStartChildProcessError.rs");
include!("HiddenChildProcessArguments.rs");
include!("kill_wrapper.rs");
include!("mkdir.rs");
include!("mkdir_wrapper.rs");
include!("pivot_root.rs");
include!("pivot_root_wrapper.rs");
include!("replace_inheritated_mounts.rs");
include!("wait_until_parent_process_has_written_uid_and_gid_maps.rs");
include!("WaitByte.rs");
include!("write_uid_and_gid_maps_and_inform_child_process.rs");
include!("allocate_child_process_stack.rs");
include!("deallocate_child_process_stack.rs");
include!("top_of_child_stack_pointer.rs");
include!("anonymous_pipe_between_parent_and_child.rs");
