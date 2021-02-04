// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Page fault event notification setting when registering a memory range.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u64)]
pub enum PageFaultEventNotificationSetting
{
	/// Never raise page fault events.
	Off = 0,
	
	/// When registering memory, request tracking of missing pages (missing pages are those that have not yet been faulted in).
	///
	/// Useful for allocated-on-demand anonymous mapped memory.
	///
	/// If used, then either `copy_memory_range()` or `zero_memory_range()` __MUST__ be used before the thread with the page fault wakes up.
	IfMissing = UFFDIO_REGISTER_MODE_MISSING,
	
	/// Track page faults on write-protected pages (WP).
	///
	/// Only for memory that does not use hugetlbfs huge pages or shmem memory pages (anonymous pages are fine).
	///
	/// Write protection must be explicitly enabled using `UserFaultFileDescriptor.enable_write_protection_of_registered_memory_subrange_and_wake_up_registered_memory_subrange()`.
	IfWriteProtectedPageAccess = UFFDIO_REGISTER_MODE_WP,

	#[allow(missing_docs)]
	BothIfMissingAndIfWriteProtectedPageAccess
}

impl Default for PageFaultEventNotificationSetting
{
	#[inline(always)]
	fn default() -> Self
	{
		PageFaultEventNotificationSetting::Off
	}
}
