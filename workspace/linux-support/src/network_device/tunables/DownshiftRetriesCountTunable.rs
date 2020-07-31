// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Usually quite small eg 9 or less.
///
/// Results in ERANGE if the value is out-of-range.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct DownshiftRetriesCountTunable(u8);

impl Tunable for DownshiftRetriesCountTunable
{
	const Identifier: TunableIdentifier = TunableIdentifier::phy(phy_tunable_id::DOWNSHIFT);
	
	const TypeIdentifier: tunable_type_id = tunable_type_id::ETHTOOL_TUNABLE_U8;
	
	const Commands: Commands = Commands::Physical;
}

impl TryFrom<NonZeroU8> for DownshiftRetriesCountTunable
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU8) -> Result<Self, Self::Error>
	{
		let value = value.get();
		if unlikely!(value == Self::Disabled.0)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl DownshiftRetriesCountTunable
{
	/// Automatic.
	pub const Automatic: Self = Self(DOWNSHIFT_DEV_DEFAULT_COUNT);
	
	/// Disabled.
	pub const Disabled: Self = Self(DOWNSHIFT_DEV_DISABLE);
	
	/// A retries count; usually quite small eg 9 or less.
	#[inline(always)]
	pub fn from_retries_count(retries_count: NonZeroU8) -> Result<Self, Self::Error>
	{
		Self::try_from(retries_count)
	}
}
