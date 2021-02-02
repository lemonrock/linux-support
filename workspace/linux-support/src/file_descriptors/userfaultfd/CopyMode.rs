// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Copy mode.
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	pub struct CopyMode: u64
	{
		/// Do not wake up.
		const DoNotWakeUp = UFFDIO_COPY_MODE_DONTWAKE;
		
		/// Write Protect (WP).
		///
		/// Only for mapped memory that does not use huge pages and was registered with `RegisterMode::AllowWriteProtectedCopying`.
		const WriteProtect = UFFDIO_COPY_MODE_WP;
	}
}
