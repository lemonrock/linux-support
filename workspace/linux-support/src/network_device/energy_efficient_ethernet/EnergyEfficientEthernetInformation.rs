// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Energy Efficient Ethernet (EEE) information.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnergyEfficientEthernetInformation
{
	/// Our supported speeds.
	pub speeds_we_could_advertise: HashSet<LegacySpeed>,
	
	/// The speeds we're advertising to our link partner.
	pub speeds_advertising_to_our_link_partner: HashSet<LegacySpeed>,
	
	/// The speeds our link parner is advertising to us.
	pub speeds_link_partner_advertising_to_us: HashSet<LegacySpeed>,
	
	/// Is active?
	pub is_active: bool,
	
	/// Is enabled?
	pub is_enabled: bool,
	
	/// Transmit low-power idle microseconds, if enabled (`Some`).
	pub transmit_low_power_idle_microseconds: Option<u32>,
}

impl EnergyEfficientEthernetInformation
{
	#[inline(always)]
	pub(crate) fn is_this_a_speed_we_could_advertise(&self, speed: LegacySpeed) -> bool
	{
		self.speeds_we_could_advertise.contains(&speed)
	}
}
