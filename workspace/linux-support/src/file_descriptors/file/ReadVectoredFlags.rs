// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags to use with `preadv2()` and io-uring.
	pub struct ReadVectoredFlags: u32
	{
		/// High priority read.
		///
		/// Allows block-based filesystems to use polling of the device, which provides lower latency, but may use additional resources.
		///
		/// (Currently, this feature is usable only on a file descriptor opened using the `O_DIRECT` flag).
		///
		/// Since Linux 4.6.
		const HighPriority = RWF_HIPRI;

		/// Do not wait for data which is not immediately available.
		///
		/// If this flag is specified, the preadv2() system call will return instantly if it would have to read data from the backing storage or wait for a lock.
		/// If some data was successfully read,  it will return the number of bytes read.
		/// If no bytes were read, it will return -1 and set errno to `EAGAIN`.
		///
		/// Since Linux 4.14.
		const NoWait = RWF_NOWAIT;
	}
}
