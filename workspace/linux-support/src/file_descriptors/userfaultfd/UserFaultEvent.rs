// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User fault event.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum UserFaultEvent
{
	/// A page fault occurred on a registered memory range.
	PageFault = UFFD_EVENT_PAGEFAULT,

	/// The process has forked.
	Fork = UFFD_EVENT_FORK,

	/// The process used `mremap()` on a registered memory range.
	Remap = UFFD_EVENT_REMAP,

	/// The process used `madvise(MADV_DONT_NEED)` or `madvise(MADV_REMOVE)` on a registered memory range.
	Remove = UFFD_EVENT_REMOVE,
	
	/// The process used `munmap()` on a registered memory range.
	Unmap = UFFD_EVENT_UNMAP,
}
