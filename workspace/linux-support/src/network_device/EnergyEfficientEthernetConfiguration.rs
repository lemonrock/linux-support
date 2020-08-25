// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Energy Efficient Ethernet (EEE) configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum EnergyEfficientEthernetConfiguration
{
	/// Disable.
	Disable,
	
	/// Enable.
	Enable
	{
		/// Link speeds to advertise with Energy Efficient Ethernet.
		advertise: HashSet<LegacySpeed>,
		
		/// Assert transmit low power idle (LPI) for this number of microseconds (including 0), if `Some`.
		transmit_low_power_idle_microseconds: Option<u32>,
	}
}

impl Default for EnergyEfficientEthernetConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		EnergyEfficientEthernetConfiguration::Disable
	}
}
