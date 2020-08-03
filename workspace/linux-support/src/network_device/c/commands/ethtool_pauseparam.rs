// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(crate) struct ethtool_pauseparam
{
	/// Either `ETHTOOL_GPAUSEPARAM` or `ETHTOOL_SPAUSEPARAM`.
	cmd: u32,
	
	autoneg: u32,
	
	rx_pause: u32,
	
	tx_pause: u32,
}

impl ethtool_pauseparam
{
	#[inline(always)]
	pub(crate) fn set(pause_configuration: PauseConfiguration) -> Self
	{
		let (autoneg, rx_pause, tx_pause) = pause_configuration.to_u32_booleans();
		Self
		{
			cmd: ETHTOOL_SPAUSEPARAM,
			autoneg,
			rx_pause,
			tx_pause
		}
	}
}
