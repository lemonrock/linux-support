// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::paths::ProcPath;
use crate::process::ProcessIdentifierChoice;


/// Huge Pages.
///
/// Functionality in here replaces the need to use `libhugetlbfs`.
pub mod huge_pages;


/// Memory information.
pub mod information;


/// Memory mapped files (`mmap()`).
pub mod mapping;


/// Memory maps from `mmap()` and the like.
///
/// Use by looking at `MemoryMaps`.
pub mod memory_maps;


/// Non-Uniform Memory Architecture (NUMA).
///
/// Functionality in here replaces the need to use `libnuma`.
pub mod numa;


/// Out-of-Memory.
pub mod out_of_memory;


/// Page map, for mapping physical to virtual addresses.
pub mod page_map;


/// Legacy System V (SysV) shared memory.
pub mod system_v_shared_memory;


include!("HasVirtualAddress.rs");
include!("Kilobyte.rs");
include!("NonZeroKilobyte.rs");
include!("NonZeroNumberOfPages.rs");
include!("NumberOfPages.rs");
include!("PageMap.rs");
include!("PageMapEntry.rs");
include!("PageSize.rs");
include!("PhysicalAddress.rs");
include!("PhysicalPageFrameNumber.rs");
include!("VirtualAddress.rs");
include!("VirtualPageFrameNumber.rs");
