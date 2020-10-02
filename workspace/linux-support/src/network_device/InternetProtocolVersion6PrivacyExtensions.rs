// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 3041 privacy extensions.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum InternetProtocolVersion6PrivacyExtensions
{
	#[allow(missing_docs)]
	Disable = 0,
	
	#[allow(missing_docs)]
	EnableAndPreferPublicAddressesOverTemporary = 1,
	
	#[allow(missing_docs)]
	EnableAndPreferTemporaryAddressesOverPublic = 2,
}

impl Default for InternetProtocolVersion6PrivacyExtensions
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6PrivacyExtensions::EnableAndPreferPublicAddressesOverTemporary
	}
}

impl InternetProtocolVersion6PrivacyExtensions
{
	#[inline(always)]
	pub(crate) fn parse(value: i32) -> Self
	{
		use self::InternetProtocolVersion6PrivacyExtensions::*;
		
		if value <= 0
		{
			Disable
		}
		else if value == 1
		{
			EnableAndPreferPublicAddressesOverTemporary
		}
		else
		{
			EnableAndPreferTemporaryAddressesOverPublic
		}
	}
}
