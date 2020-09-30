// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Rule location.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum RuleLocation
{
	/// Fixed.
	Fixed(FixedRuleLocation),
	
	/// Special.
	Special(SpecialRuleLocation),
}

impl PartialOrd for RuleLocation
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		use self::Ordering::*;
		use self::RuleLocation::*;
		use self::SpecialRuleLocation::*;
		
		match (self, rhs)
		{
			(Fixed(lhs), Fixed(rhs)) => lhs.partial_cmp(rhs),
			
			(Special(lhs), Special(rhs)) => lhs.partial_cmp(rhs),
			
			(Fixed(_), Special(rhs)) => match rhs
			{
				First => Some(Greater),
				
				Last => Some(Less),
				
				Any => None,
			}
			
			(Special(lhs), Fixed(_)) => match lhs
			{
				First => Some(Less),
				
				Last => Some(Greater),
				
				Any => None,
			}
		}
	}
}

impl Default for RuleLocation
{
	#[inline(always)]
	fn default() -> Self
	{
		RuleLocation::Special(SpecialRuleLocation::default())
	}
}

impl Into<u32> for RuleLocation
{
	#[inline(always)]
	fn into(self) -> u32
	{
		use self::RuleLocation::*;
		
		match self
		{
			Fixed(rule_location) => rule_location.into(),
			
			Special(rule_location) => rule_location.into(),
		}
	}
}
