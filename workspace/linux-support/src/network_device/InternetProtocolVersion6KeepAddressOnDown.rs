// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// How or whether to keep address on an interface down event.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum InternetProtocolVersion6KeepAddressOnDown
{
	#[allow(missing_docs)]
	Disabled = -1,
	
	#[allow(missing_docs)]
	SystemDefault = 0,
	
	#[allow(missing_docs)]
	Enabled = 1,
}

impl Default for InternetProtocolVersion6KeepAddressOnDown
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6KeepAddressOnDown::SystemDefault
	}
}

impl InternetProtocolVersion6KeepAddressOnDown
{
	#[inline(always)]
	pub(crate) fn parse(value: i32) -> Self
	{
		use self::InternetProtocolVersion6KeepAddressOnDown::*;
		
		if value < 0
		{
			Disabled
		}
		else if value == 0
		{
			SystemDefault
		}
		else
		{
			Enabled
		}
	}
}
