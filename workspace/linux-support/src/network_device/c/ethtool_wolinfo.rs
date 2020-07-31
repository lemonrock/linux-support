// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Wake-On-Lan configuration
#[repr(C)]
pub(crate) struct ethtool_wolinfo
{
	/// Either `ETHTOOL_GWOL` or `ETHTOOL_SWOL`.
	pub(crate) cmd: u32,
	
	/// Bitmask of `WAKE_*` flags for supported Wake-On-Lan modes.
	///
	/// Read-only.
	pub(crate) supported: WAKE,
	
	/// Bitmask of `WAKE_*` flags for enabled Wake-On-Lan modes.
	pub(crate) wolopts: WAKE,
	
	/// SecureOn™ password; meaningful only if `WAKE_MAGICSECURE` is set in `wolopts`.
	///
	/// A 6-byte Ethernet Media Access Control Address.
	pub(crate) sopass: [u8; Self::SOPASS_MAX],
}

impl EthtoolCommand for ethtool_wolinfo
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}
