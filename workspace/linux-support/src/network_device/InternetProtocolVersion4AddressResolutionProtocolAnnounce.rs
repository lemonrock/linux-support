// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `arp_announce`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum InternetProtocolVersion4AddressResolutionProtocolAnnounce
{
	#[allow(missing_docs)]
	UseAnyLocalAddressOnAnyInterface = 0,
	
	/// Try to avoid local addresses that are not in the target’s subnet for this interface.
	///
	/// This mode is useful when target hosts reachable via this interface require the source IP address in ARP requests to be part of their logical network configured on the receiving interface.
	/// When we generate the request we will check all our subnets that include the target IP and will preserve the source address if it is from such a subnet.
	/// If there is no such subnet we select a source address according to the rules for `Level2`.
	Level1 = 1,
	
	/// Always use the best local address for this target.
	///
	/// In this mode we ignore the source address in the IP packet and try to select a local address that we prefer for talks with the target host.
	/// Such a local address is selected by looking for primary IP addresses on all our subnets on the outgoing interface that include the target IP address.
	/// If no suitable local address is found we select the first local address we have on the outgoing interface or on all other interfaces, with the hope we will receive a reply for our request and even sometimes no matter the source IP address we announce.
	Level2 = 2,
}

impl Default for InternetProtocolVersion4AddressResolutionProtocolAnnounce
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetProtocolVersion4AddressResolutionProtocolAnnounce::UseAnyLocalAddressOnAnyInterface
	}
}

impl InternetProtocolVersion4AddressResolutionProtocolAnnounce
{
	#[inline(always)]
	pub(crate) fn parse(value: u32) -> Result<Self, String>
	{
		use self::InternetProtocolVersion4AddressResolutionProtocolAnnounce::*;
		
		match value
		{
			0 => Ok(UseAnyLocalAddressOnAnyInterface),
			
			1 => Ok(Level1),
			
			2 => Ok(Level2),
			
			_ => Err(format!("Unexpected value for arp_announce of {}", value))
		}
	}
}
