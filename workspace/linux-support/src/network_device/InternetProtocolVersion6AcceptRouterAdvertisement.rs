// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Autoconfiguration using router advertisements.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum InternetProtocolVersion6AcceptRouterAdvertisement
{
	#[allow(missing_docs)]
	DoNotAccept = 0,
	
	#[allow(missing_docs)]
	AcceptIfForwardingDisabled = 1,
	
	#[allow(missing_docs)]
	AcceptRegardless = 2,
}

impl Default for InternetProtocolVersion6AcceptRouterAdvertisement
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6AcceptRouterAdvertisement::AcceptIfForwardingDisabled
	}
}

impl InternetProtocolVersion6AcceptRouterAdvertisement
{
	#[inline(always)]
	pub(crate) fn parse(value: u32) -> Result<Self, String>
	{
		use self::InternetProtocolVersion6AcceptRouterAdvertisement::*;
		
		match value
		{
			0 => Ok(DoNotAccept),
			
			1 => Ok(AcceptIfForwardingDisabled),
			
			2 => Ok(AcceptRegardless),
			
			_ => Err(format!("Unexpected value for accept_ra of {}", value))
		}
	}
}
