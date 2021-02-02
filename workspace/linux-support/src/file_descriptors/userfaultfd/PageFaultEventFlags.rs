// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Page fault event flags.
	///
	/// Note that `all()` is equivalent to `UFFD_API_FEATURES` (which has not been defined in this module).
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	pub struct PageFaultEventFlags: u64
	{
		/// Write fault.
		///
		/// If not set then a read fault.
		const WriteFault = UFFD_PAGEFAULT_FLAG_WRITE;
		
		/// Write protect fault (if supported and requested
		///
		/// Only set if `Features::ReceivePageFaultWriteProtectEvents` was requested and supported..
		const WriteProtectFault = UFFD_PAGEFAULT_FLAG_WP;
	}
}

impl PageFaultEventFlags
{
	/// Was this a read fault?
	#[inline(always)]
	pub fn was_read_fault(self) -> bool
	{
		!self.was_write_fault()
	}
	
	/// Was this a write fault?
	#[inline(always)]
	pub fn was_write_fault(self) -> bool
	{
		self.contains(Self::WriteFault)
	}
}
