// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Basic flow parse error.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum BasicFlowParseError
{
	/// Only used by `ETHTOOL_SRXFH` (`do_srxclass()`) and `ETHTOOL_GRXFH` (`do_grxclass()`).
	UnexpectedActualFlowType { actual_flow_type: u32 },
	
	/// Unused by ethtool but is supported by Amazon ENA driver.
	UnusedActualFlowType { actual_flow_type: u32 },
	
	/// Unknown and possibly a bug in Linux.
	UnknownActualFlowType { actual_flow_type: u32 },
	
	/// UserOverInternetProtocolVersion4 flow must have the `ip_ver` data as only `ETH_RX_NFC_IP4`.
	UserOverInternetProtocolVersion4FlowHasInvalidVersionData { ip_ver: u8 },
	
	/// UserOverInternetProtocolVersion4 flow must have the `ip_ver` mask as only `0`.
	UserOverInternetProtocolVersion4FlowHasInvalidVersionMask { ip_ver: u8 },
	
	/// UserOverInternetProtocolVersion4 flow must have the `proto` mask as only `0` or `0xFF`.
	UserLayer4FlowTransportProtocolNumberMaskMustEitherBeZeroOr0xFF { transport_protocol_number_mask: u8 },
	
	/// Ethernet flows do not use this extension.
	EthernetFlowIsNotAllowedAnExtendedDestinationMediaAccessControlAddress { destination_media_access_control_address_extended_flow: DestinationMediaAccessControlAddressExtendedFlow },
}

impl Display for BasicFlowParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for BasicFlowParseError
{
}
