// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Force Multicast Listener Detection (MLD) version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum InternetProtocolVersion6ForceMulticastListenerDiscoverVersion
{
	#[allow(missing_docs)]
	NoEnforcementWithFallbackToVersion1 = 0,
	
	#[allow(missing_docs)]
	ForceVersion1 = 1,
	
	#[allow(missing_docs)]
	ForceVersion2 = 2,
}

impl Default for InternetProtocolVersion6ForceMulticastListenerDiscoverVersion
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion6ForceMulticastListenerDiscoverVersion::NoEnforcementWithFallbackToVersion1
	}
}

impl InternetProtocolVersion6ForceMulticastListenerDiscoverVersion
{
	#[inline(always)]
	pub(crate) fn parse(value: u32) -> Result<Self, String>
	{
		use self::InternetProtocolVersion6ForceMulticastListenerDiscoverVersion::*;
		
		match value
		{
			0 => Ok(NoEnforcementWithFallbackToVersion1),
			
			1 => Ok(ForceVersion1),
			
			2 => Ok(ForceVersion2),
			
			_ => Err(format!("Unexpected value for force_mld_version of {}", value))
		}
	}
}
