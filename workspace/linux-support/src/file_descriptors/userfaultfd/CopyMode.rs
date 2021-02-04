// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Copy mode.
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	struct CopyMode: u64
	{
		/// Do not wake up.
		const DoNotWakeUp = UFFDIO_COPY_MODE_DONTWAKE;
		
		/// Write Protect (WP).
		///
		/// Only for mapped memory registered with `PageFaultEventNotificationSetting::IfWriteProtectedPageAccess` or similar.
		const WriteProtect = UFFDIO_COPY_MODE_WP;
	}
}

impl Default for CopyMode
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}

impl CopyMode
{
	const fn new(wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange: bool, write_protect: bool) -> Self
	{
		unsafe { CopyMode::from_bits_unchecked(((!wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange) as u64) | ((write_protect as u64) << 1)) }
	}
}
