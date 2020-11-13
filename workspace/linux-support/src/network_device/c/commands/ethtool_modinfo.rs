// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Copy, Clone)]
pub(crate) struct ethtool_modinfo
{
	/// Always `ETHTOOL_GMODULEINFO`.`
	cmd: u32,
	
	/// One of `ETH_MODULE_SFF_` or `0`.
	pub(crate) type_: u32,
	
	pub(crate) eeprom_len: u32,

	reserved: [u32; 8],
}

impl EthtoolCommand for ethtool_modinfo
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl ethtool_modinfo
{
	#[inline(always)]
	pub(crate) fn get() -> Self
	{
		Self
		{
			cmd: ETHTOOL_GMODULEINFO,
			type_: 0,
			eeprom_len: 0,
			reserved: unsafe_zeroed(),
		}
	}
}
