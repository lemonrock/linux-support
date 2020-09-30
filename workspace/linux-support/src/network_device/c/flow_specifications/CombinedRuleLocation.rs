// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Rule location.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct CombinedRuleLocation(u32);

impl From<RuleLocation> for CombinedRuleLocation
{
	#[inline(always)]
	fn from(rule_location: RuleLocation) -> Self
	{
		Self(rule_location.into())
	}
}

impl TryInto<RuleLocation> for CombinedRuleLocation
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_into(self) -> Result<RuleLocation, Self::Error>
	{
		use self::RuleLocation::*;
		use self::SpecialRuleLocation::*;
		
		let value = self.0;
		let is_special = value & RX_CLS_LOC_SPECIAL != 0;
		if is_special
		{
			match value
			{
				RX_CLS_LOC_ANY => Ok(Special(Any)),
				
				RX_CLS_LOC_FIRST => Ok(Special(First)),
				
				RX_CLS_LOC_LAST => Ok(Special(Last)),
				
				_ => Err(ParseNumberError::OutOfRange)
			}
		}
		else
		{
			Ok(Fixed(FixedRuleLocation(value)))
		}
	}
}
