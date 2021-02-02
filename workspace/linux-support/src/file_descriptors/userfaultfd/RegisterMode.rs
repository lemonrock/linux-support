// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Register mode.
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	pub struct RegisterMode: u64
	{
		/// When registering memory, request tracking of missing pages (missing pages are those that have not yet been faulted in).
		///
		/// Useful for allocated-on-demand anonymous mapped memory.
		///
		/// If used, then either `copy()` or `zero_page_copy()` __MUST__ be used before the thread with the page fault wakes up.
		const Missing = UFFDIO_REGISTER_MODE_MISSING;
		
		/// Track page faults on write-protected pages (WP).
		///
		/// Only for memory that does not use huge pages.
		const AllowWriteProtectedCopying = UFFDIO_REGISTER_MODE_WP;
	}
}
