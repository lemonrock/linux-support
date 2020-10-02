// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `arp_ignore`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum InternetProtocolVersion4AddressResolutionProtocolIgnore
{
	#[allow(missing_docs)]
	UseAnyLocalAddressOnAnyInterface = 0,
	
	/// Reply only if the target IP address is local address configured on the incoming interface.
	Level1 = 1,
	
	/// Reply only if the target IP address is local address configured on the incoming interface and both with the sender’s IP address are part from same subnet on this interface.
	Level2 = 2,
	
	/// Do not reply for local addresses configured with scope host, only resolutions for global and link addresses are replied.
	Level3 = 3,

	/// Do not reply for all local addresses.
	Level8 = 8,
}

impl Default for InternetProtocolVersion4AddressResolutionProtocolIgnore
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion4AddressResolutionProtocolIgnore::UseAnyLocalAddressOnAnyInterface
	}
}

impl InternetProtocolVersion4AddressResolutionProtocolIgnore
{
	#[inline(always)]
	pub(crate) fn parse(value: u32) -> Result<Self, String>
	{
		use self::InternetProtocolVersion4AddressResolutionProtocolIgnore::*;
		
		match value
		{
			0 => Ok(UseAnyLocalAddressOnAnyInterface),
			
			1 => Ok(Level1),
			
			2 => Ok(Level2),
			
			3 => Ok(Level3),
			
			4 ..= 7 => Err(format!("Reserved value for arp_ignore of {}", value)),
			
			8 => Ok(Level8),
			
			_ => Err(format!("Unexpected value for arp_ignore of {}", value))
		}
	}
}
