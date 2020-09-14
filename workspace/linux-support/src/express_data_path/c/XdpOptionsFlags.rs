// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for `struct xdp_options`.
	pub struct XdpOptionsFlags: u32
	{
		/// Zero-copy.
		const ZeroCopy = XDP_OPTIONS_ZEROCOPY;
	}
}

impl Default for XdpOptionsFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}
