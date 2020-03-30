// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::paths::ProcPath;
use libc_extra::unix::unistd::getpagesize;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::mem::size_of;
#[allow(deprecated)] use std::mem::uninitialized;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::path::Path;
use std::ptr::NonNull;
use crate::process::ProcessIdentifier;


/// Huge Pages.
///
/// Functionality in here replaces the need to use `libhugetlbfs`.
pub mod huge_pages;

/// Memory information.
pub mod information;

/// NUMA.
///
/// Functionality in here replaces the need to use `libnuma`.
pub mod numa;


include!("HasVirtualAddress.rs");
include!("page_size.rs");
include!("PageMap.rs");
include!("PageMapEntry.rs");
include!("PhysicalAddress.rs");
include!("PhysicalPageFrameNumber.rs");
include!("VirtualAddress.rs");
include!("VirtualPageFrameNumber.rs");


const DirectMemoyAccessMemoryAlignment: usize = 128;
/*
	// TODO: Wrap SignalNumber and realtime SignalNumber in their own types a la Capability with BitSet.

	// TODO: NumaNode / HyperThread BTree to BitSet and vice versa; prefer use of BitSet for operations as it is faster.

	xxx;
	// TODO: Explore hugepages per NUMA node, and check if all files are psent - only:-
		&nr_hugepages_attr.attr,
		&free_hugepages_attr.attr,
		&surplus_hugepages_attr.attr,

	// TODO: Use mmap with the new flags to mmap huge pages
*/
