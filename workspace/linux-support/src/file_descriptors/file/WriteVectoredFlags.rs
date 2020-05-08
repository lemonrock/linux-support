// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags to use with `pwritev2()` and io-uring.
	pub struct WriteVectoredFlags: u32
	{
		/// High priority write.
		///
		/// Allows block-based filesystems to use polling of the device, which provides lower latency, but may use additional resources.
		///
		/// (Currently, this feature is usable only on a file descriptor opened using the `O_DIRECT` flag).
		///
		/// Since Linux 4.6.
		const HighPriority = RWF_HIPRI;

		/// Provide a per-write equivalent of the O_DSYNC open(2) flag.
		///
		/// Since Linux 4.7.
		const SynchronizeDataOnly = RWF_DSYNC;

		/// Provide a per-write equivalent of the O_SYNC open(2) flag.
		///
		/// Since Linux 4.7.
		const SynchronizeDataAndMetadata = RWF_SYNC;

		/// Provide a per-write equivalent of the O_APPEND open(2) flag.
		///
		/// Applies only to the data range written by the system call.
		/// The `offset` argument does not affect the write operation; the data is always appended to the end of the file.
		///
		/// However, if the offset argument is `ReadOrWriteOffset::CurrentFilePosition`, the current file offset is updated.
		///
		/// Since Linux 4.16.
		const Append = RWF_APPEND;
	}
}
