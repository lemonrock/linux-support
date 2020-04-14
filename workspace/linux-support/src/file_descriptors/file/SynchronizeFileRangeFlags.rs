// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags.
	///
	/// See <https://linux.die.net/man/2/sync_file_range> for explanation of what combinations of these flags achieve.
	pub struct SynchronizeFileRangeFlags: u32
	{
		/// Wait upon write-out of all pages in the specified range that have already been submitted to the device driver for write-out before performing any write.
		///
		/// Equivalent to `SYNC_FILE_RANGE_WAIT_BEFORE`.
		const WaitBefore = SYNC_FILE_RANGE_WAIT_BEFORE;

		/// Initiate write-out of all dirty pages in the specified range which are not presently submitted write-out.
		///
		/// Note that even this may block if you attempt to write more than request queue size.
		///
		/// Equivalent to `SYNC_FILE_RANGE_WRITE`.
		const Write = SYNC_FILE_RANGE_WRITE;

		/// Equivalent to `SYNC_FILE_RANGE_WAIT_AFTER`.
		///
		/// Wait upon write-out of all pages in the range after performing any write.
		const WaitAfter = SYNC_FILE_RANGE_WAIT_AFTER;
	}
}
