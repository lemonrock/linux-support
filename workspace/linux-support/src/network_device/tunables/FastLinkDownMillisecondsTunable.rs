// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Results in ERANGE if the value is out-of-range.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct FastLinkDownMillisecondsTunable(u8);

impl Tunable for FastLinkDownMillisecondsTunable
{
	const Identifier: TunableIdentifier = TunableIdentifier::phy(phy_tunable_id::ETHTOOL_PHY_FAST_LINK_DOWN);
	
	const TypeIdentifier: tunable_type_id = tunable_type_id::ETHTOOL_TUNABLE_U8;
	
	const Commands: Commands = Commands::Physical;
}

impl TryFrom<NonZeroU8> for FastLinkDownMillisecondsTunable
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU8) -> Result<Self, Self::Error>
	{
		let value = value.get();
		if unlikely!(value == Self::Off.0)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl FastLinkDownMillisecondsTunable
{
	/// Automatic.
	pub const On: Self = Self(ETHTOOL_PHY_FAST_LINK_DOWN_ON);
	
	/// Disabled.
	pub const Off: Self = Self(ETHTOOL_PHY_FAST_LINK_DOWN_OFF);
	
	/// Time in milliseconds; driver may choose a nearest value.
	#[inline(always)]
	pub fn from_milliseconds(milliseconds_to_delay_reporting_link_is_down: NonZeroU8) -> Result<Self, Self::Error>
	{
		Self::try_from(milliseconds_to_delay_reporting_link_is_down)
	}
}
