// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::c::flow_specifications::*;
use crate::file_descriptors::socket::c::{in6_addr, in_addr};


/// Masked data.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct MaskedData<T: Default + Debug + Copy + Clone + PartialEq + Eq + Partialord + Ord + Hash>
{
	data: T,

	mask: T,
}

/// Masked internet protocol address.
///
/// Similar to `InternetProtocolAddressWithMask` but differs because the mask is not necessarily a prefix mask.
pub type MaskedInternetProtocolAddress<IPA: InternetProtocolAddress> = MaskedData<IPA>;

/// Masked port.
pub type MaskedPort = MaskedData<BigEndianU16>;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CommonLayer4Flow<IPA: InternetProtocolAddress>
{
	source_address: MaskedInternetProtocolAddress<IPA>,
	
	destination_address: MaskedInternetProtocolAddress<IPA>,
	
	type_of_service_or_traffic_class: MaskedData<u8>,

	source_port: MaskedPort,
	
	destination_port: MaskedPort,
}

impl CommonLayer4Flow<in_addr>
{
	fn for_internet_protocol_version_4(data: &ethtool_tcpip4_spec, mask: &ethtool_tcpip4_spec) -> Self
	{
		Self
		{
			source_address: MaskedInternetProtocolAddress
			{
				data: data.ip4src,
				
				mask: mask.ip4src,
			},
			
			destination_address: MaskedInternetProtocolAddress
			{
				data: data.ip4dst,
				
				mask: mask.ip4dst,
			},
			
			type_of_service_or_traffic_class: MaskedData
			{
				data: data.tos,
				
				mask: mask.tos,
			},
			
			source_port: MaskedPort
			{
				data: data.psrc,
				
				mask: mask.psrc,
			},
			
			destination_port: MaskedPort
			{
				data: data.pdst,
				
				mask: mask.pdst,
			},
		}
	}
}

impl CommonLayer4Flow<in6_addr>
{
	fn for_internet_protocol_version_6(data: &ethtool_tcpip6_spec, mask: &ethtool_tcpip6_spec) -> Self
	{
		Self
		{
			source_address: MaskedInternetProtocolAddress
			{
				data: data.ip6src,
				
				mask: mask.ip6src,
			},
			
			destination_address: MaskedInternetProtocolAddress
			{
				data: data.ip6dst,
				
				mask: mask.ip6dst,
			},
			
			type_of_service_or_traffic_class: MaskedData
			{
				data: data.tclass,
				
				mask: mask.tclass,
			},
			
			source_port: MaskedPort
			{
				data: data.psrc,
				
				mask: mask.psrc,
			},
			
			destination_port: MaskedPort
			{
				data: data.pdst,
				
				mask: mask.pdst,
			},
		}
	}
}

/// Also called a 'filter' or 'filter rule'.
fn from_ethtool_rx_flow_spec(fsp: ethtool_rx_flow_spec)
{
	let filter = value.location;
	
	match fsp.actual_flow_type()
	{
		TCP_V4_FLOW => TransmissionControlProtocolOverInternetProtocolVersion4Flow
		{
			common: CommonLayer4Flow::for_internet_protocol_version_4(&fsp.h_u.tcp_ip4_spec, &fsp.m_u.tcp_ip4_spec),
		},
		
		UDP_V4_FLOW => UserDatagramProtocolOverInternetProtocolVersion4Flow
		{
			common: CommonLayer4Flow::for_internet_protocol_version_4(&fsp.h_u.udp_ip4_spec, &fsp.m_u.udp_ip4_spec),
		},
		
		SCTP_V4_FLOW => StreamControlTransmissionProtocolOverInternetProtocolVersion4Flow
		{
			common: CommonLayer4Flow::for_internet_protocol_version_4(&fsp.h_u.sctp_ip4_spec, &fsp.m_u.sctp_ip4_spec),
		},
		
		
		
		
		TCP_V6_FLOW => TransmissionControlProtocolOverInternetProtocolVersion4Flow
		{
			common: CommonLayer4Flow::for_internet_protocol_version_6(&fsp.h_u.tcp_ip6_spec, &fsp.m_u.tcp_ip6_spec),
		},
		
		UDP_V6_FLOW => UserDatagramProtocolOverInternetProtocolVersion4Flow
		{
			common: CommonLayer4Flow::for_internet_protocol_version_6(&fsp.h_u.udp_ip6_spec, &fsp.m_u.udp_ip6_spec),
		},
		
		SCTP_V6_FLOW => StreamControlTransmissionProtocolOverInternetProtocolVersion4Flow
		{
			common: CommonLayer4Flow::for_internet_protocol_version_6(&fsp.h_u.sctp_ip6_spec, &fsp.m_u.sctp_ip6_spec),
		},
		
		_ => panic!(),
	};
}
