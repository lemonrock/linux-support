// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::syscall::*;
use super::PageSize;
use super::huge_pages::*;
use super::information::*;
use crate::bit_set::*;
use crate::cpu::HyperThread;
use crate::current_numa_node_and_hyper_thread;
use crate::paths::*;
use crate::process::*;
use crate::process::status::ProcessStatusStatistics;
use crate::strings::FromBytes;
use crate::strings::Radix;
use crate::strings::parse_number::*;
use crate::strings::to_number::NumberAsBytes;
use crate::user_and_groups::assert_effective_user_id_is_root;
use bitflags::bitflags;
use errno::errno;
use libc::c_void;
use libc::E2BIG;
use libc::EACCES;
use libc::EBUSY;
use libc::ENODEV;
use libc::EFAULT;
use libc::EINVAL;
use libc::EIO;
use libc::ENOENT;
use libc::ENOMEM;
use libc::EPERM;
use libc::ESRCH;
use likely::*;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error;
use std::io;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
#[allow(deprecated)] use std::mem::uninitialized;
use std::num::NonZeroU8;
use std::path::Path;
use std::ptr::NonNull;
use std::ptr::null;
use std::ptr::null_mut;
use std::io::ErrorKind;


include!("BitSetNumaNode.rs");
include!("GetMemoryPolicyFlags.rs");
include!("MemoryLatencyRelativeCost.rs");
include!("NumaNode.rs");
include!("PageMoveError.rs");
include!("PageMoveStatus.rs");
include!("PerPageMoveError.rs");
include!("MemoryPolicy.rs");
include!("MemoryPolicyDynamism.rs");
include!("SetMemoryPolicy.rs");


mod syscall;
