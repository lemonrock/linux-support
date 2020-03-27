// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::cpu::HyperThread;
use crate::memory::huge_pages::*;
use crate::memory::information::*;
use crate::user_and_groups::assert_effective_user_id_is_root;
use crate::paths::*;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;
use std::num::TryFromIntError;


include!("NumaNode.rs");
include!("NumaNodeBitmask.rs");

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

//mod syscall
//{
//	use libc::*;
//
//	#[inline(always)]
//	fn get_mempolicy(policy: *mut c_int, nmask: *c_ulong, maxnode: c_ulong, add: *mut c_void, flags: uint) -> c_long
//	{
//		unsafe { syscall(__NR_get_mempolicy, policy, nmask, maxnode, addr, flags) }
//	}
//
//	#[inline(always)]
//	fn set_mempolicy(mode: c_int, nmask: *const c_ulong, maxnode: c_ulong) -> c_long
//	{
//		unsafe { syscall(__NR_set_mempolicy, mode, nmask, maxnode) }
//	}
//
//	#[inline(always)]
//	fn mbind(start: *mut c_void, len: c_ulong, mode: c_int, nmask: *const c_ulong, maxnode: c_ulong, flags: c_uint) -> c_long
//	{
//		unsafe { syscall(__NR_mbind, (long) start, len, mode, (long) nmask, maxnode, flags) }
//	}
//
//	#[inline(always)]
//	fn migrate_pages(pid: c_int, maxnode: c_ulong, frommask: *const c_ulong, tomask: *const c_ulong) -> c_long
//	{
//		if defined(__NR_migrate_pages)
//		{
//			unsafe { syscall(__NR_migrate_pages, pid, maxnode, frommask, tomask) }
//		}
//		else
//		{
//			use errno::Errno;
//			use errno::set_errno;
//			set_errno(Errno(ENOSYS));
//			return -1
//		}
//	}
//
//	#[inline(always)]
//	fn move_pages(pid: c_int, count: c_ulong, pages: *mut *mut c_void, nodes: *const c_int, status: *mut c_int, flags: c_int) -> c_long
//	{
//		unsafe { syscall(__NR_move_pages, pid, count, pages, nodes, status, flags) }
//	}
//}
