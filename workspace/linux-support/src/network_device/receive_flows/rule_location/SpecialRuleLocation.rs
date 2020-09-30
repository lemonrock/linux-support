// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Special rule location.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum SpecialRuleLocation
{
	Any = RX_CLS_LOC_ANY,
	
	First = RX_CLS_LOC_FIRST,
	
	Last = RX_CLS_LOC_LAST,
}

impl PartialOrd for SpecialRuleLocation
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		use self::Ordering::*;
		use self::SpecialRuleLocation::*;
		
		match (self, rhs)
		{
			(First, First) => Some(Equal),
			
			(First, Last) => Some(Less),
			
			(First, Any) => None,
			
			(Last, Last) => Some(Equal),
			
			(Last, First) => Some(Greater),
			
			(Last, Any) => None,
			
			(Any, First) => None,
			
			(Any, Last) => None,
			
			(Any, Any) => None,
		}
	}
}

impl Default for SpecialRuleLocation
{
	#[inline(always)]
	fn default() -> Self
	{
		SpecialRuleLocation::Any
	}
}

impl Into<u32> for SpecialRuleLocation
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self as u32
	}
}
