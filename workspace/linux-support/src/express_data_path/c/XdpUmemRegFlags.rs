// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for `struct xdp_umem_reg`.
	pub(crate) struct XdpUmemRegFlags: u32
	{
		/// Unaligned chunks.
		///
		/// In this case, `xdp_desc.addr` have an offset in the topmost 12 bits.
		const UnalignedChunks = XDP_UMEM_UNALIGNED_CHUNK_FLAG;
		
		// This flag is ***not*** part of the public API.
		#[doc(hidden)]
		const UsesNeedWakeUp = XDP_UMEM_USES_NEED_WAKEUP;
	}
}

impl Default for XdpUmemRegFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}
