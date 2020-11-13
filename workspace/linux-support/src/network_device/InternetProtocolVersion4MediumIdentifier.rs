// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum InternetProtocolVersion4MediumIdentifier
{
	Unknown,

	Sole,

	MoreThanOne
	{
		/// Can not be `0xFFFF_FFFF`.
		identifier: NonZeroU32,
	}
}

impl Default for InternetProtocolVersion4MediumIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion4MediumIdentifier::Unknown
	}
}

impl InternetProtocolVersion4MediumIdentifier
{
	#[inline(always)]
	pub(crate) fn parse(value: i32) -> Self
	{
		use self::InternetProtocolVersion4MediumIdentifier::*;
		
		match value
		{
			-1 => Unknown,
			
			0 => Sole,
			
			identifier @ _ => MoreThanOne { identifier: new_non_zero_u32(identifier as u32)},
		}
	}
}
