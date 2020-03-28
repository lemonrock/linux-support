// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::syscall::*;
use super::page_size;
use super::huge_pages::*;
use super::information::*;
use crate::current_numa_node_and_hyper_thread;
use crate::bit_set::*;
use crate::cpu::HyperThread;
use crate::paths::*;
use crate::user_and_groups::assert_effective_user_id_is_root;
use bitflags::bitflags;
use errno::errno;
use libc::c_void;
use libc::EFAULT;
use libc::EINVAL;
use libc::EIO;
use libc::ENOMEM;
use libc::EPERM;
use likely::*;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;
use std::mem::transmute;
#[allow(deprecated)] use std::mem::uninitialized;
use std::ptr::NonNull;
use std::ptr::null;
use std::ptr::null_mut;


include!("GetMemoryPolicyFlags.rs");
include!("NumaNode.rs");
include!("MemoryPolicy.rs");
include!("MemoryPolicyDynamism.rs");
include!("SetMemoryPolicy.rs");


mod syscall;

/*
	xxx;
	// TODO: Explore hugepages global mempolicy

	xxx;
	// TODO: Explore hugepages per NUMA node, and check if all files are psent - only:-
		&nr_hugepages_attr.attr,
		&free_hugepages_attr.attr,
		&surplus_hugepages_attr.attr,

	// TODO: Use mmap with the new flags to mmap huge pages
*/
