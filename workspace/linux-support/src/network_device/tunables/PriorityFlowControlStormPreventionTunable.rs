// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Time, in milliseconds, after which the link is reported as down.
///
/// Defaults to `Automatic`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct PriorityFlowControlStormPreventionTunable(u16);

impl Tunable for PriorityFlowControlStormPreventionTunable
{
	const Identifier: TunableIdentifier = TunableIdentifier::normal(tunable_id::ETHTOOL_PFC_PREVENTION_TOUT);
	
	const TypeIdentifier: tunable_type_id = tunable_type_id::ETHTOOL_TUNABLE_U16;
	
	const Commands: Commands = Commands::Normal;
}

impl TryFrom<NonZeroU16> for PriorityFlowControlStormPreventionTunable
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, Self::Error>
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

impl PriorityFlowControlStormPreventionTunable
{
	/// Automatic.
	pub const Automatic: Self = Self(PFC_STORM_PREVENTION_AUTO);
	
	/// Disabled.
	pub const Disabled: Self = Self(PFC_STORM_PREVENTION_DISABLE);
	
	/// From milliseconds to delay.
	#[inline(always)]
	pub fn from_milliseconds(milliseconds_to_delay_reporting_link_is_down: NonZeroU16) -> Result<Self, Self::Error>
	{
		Self::try_from(milliseconds_to_delay_reporting_link_is_down)
	}
}
