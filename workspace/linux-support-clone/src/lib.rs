#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]


//! #linux-support-common
//! 
//! This library provides wrappers and additional functionality to make use of a panoply of miscellaneous Linux (and, sometimes, POSIX) features.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_arch = "x86_64");
assert_cfg!(target_pointer_width = "64");


use bitflags::bitflags;
use errno::errno;
use file_descriptors::AsRawFdExt;
use file_descriptors::CreationError;
use file_descriptors::pipes_and_fifos::PipeFileDescriptor;
use file_descriptors::pipes_and_fifos::ReceivePipeFileDescriptor;
use file_descriptors::pipes_and_fifos::SendPipeFileDescriptor;
use libc::*;
use likely::likely;
use likely::unlikely;
use linux_support_common::file_systems::FileSystemType;
use linux_support_common::mounts::Mount;
use linux_support_common::mounts::mount_wrapper;
use linux_support_common::mounts::MountFlags;
use linux_support_common::mounts::UnmountFlags;
use linux_support_common::namespaces::setup_uts_namespace;
use linux_support_common::namespaces::write_uid_and_gid_maps;
use linux_support_common::paths::ProcPath;
use linux_support_common::signals::SignalNumber;
use linux_support_common::strings::ConstCStr;
use linux_support_common::strings::path_to_cstring;
use serde::Deserialize;
use serde::Serialize;
use std::alloc::Layout;
use std::alloc::AllocRef;
use std::alloc::AllocErr;
use std::collections::HashMap;
use std::convert::TryInto;
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
use std::num::NonZeroU32;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::ptr::NonNull;
use std::ptr::null_mut;
use std::str::FromStr;


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
