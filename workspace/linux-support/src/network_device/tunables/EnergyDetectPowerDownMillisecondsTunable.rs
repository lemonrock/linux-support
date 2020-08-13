// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Energy Detect Power Down (EDPD).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct EnergyDetectPowerDownMillisecondsTunable(u16);

impl Tunable for EnergyDetectPowerDownMillisecondsTunable
{
	const Identifier: TunableIdentifier = TunableIdentifier::phy(phy_tunable_id::ETHTOOL_PHY_EDPD);
	
	const TypeIdentifier: tunable_type_id = tunable_type_id::ETHTOOL_TUNABLE_U16;
	
	const Commands: Commands = Commands::Physical;
}

impl TryFrom<NonZeroU16> for EnergyDetectPowerDownMillisecondsTunable
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, Self::Error>
	{
		let value = value.get();
		if unlikely!(value >= Self::ETHTOOL_PHY_EDPD_NO_TX.0)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl EnergyDetectPowerDownMillisecondsTunable
{
	/// Automatic.
	pub const NoTransmit: Self = Self(ETHTOOL_PHY_EDPD_NO_TX);
	
	/// Automatic.
	pub const On: Self = Self(ETHTOOL_PHY_EDPD_DFLT_TX_MSECS);
	
	/// Disabled.
	pub const Off: Self = Self(ETHTOOL_PHY_EDPD_DISABLE);
	
	/// Time in milliseconds; driver may choose a nearest value.
	#[inline(always)]
	pub fn from_milliseconds(milliseconds_to_delay: NonZeroU16) -> Result<Self, ParseNumberError>
	{
		Self::try_from(milliseconds_to_delay)
	}
}
