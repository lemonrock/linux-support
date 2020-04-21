// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags to control behaviour.
	///
	/// Can be empty.
	///
	/// Not all flags can be supported by all file systems.
	pub struct RetrieveFileExtentsFlags: u32
	{
		/// Synchronize file before returning information.
		///
		/// Potentially an expensive operation.
		///
		/// All known file systems support this.
		const Synchronize = FIEMAP_FLAG_SYNC;

		/// Cache extent data for future calls.
		///
		/// Only supported by `ext2` and `f2fs`, and, it is believed, not supported outside of the Linux kernel because of the implementation of `fiemap_check_flags()`.
		#[deprecated(since = "0.0.0", note = "believed to not be supported outside of the Linux kernel because of the implementation of `fiemap_check_flags()`")]
		const Cache = FIEMAP_FLAG_CACHE;
	}
}
