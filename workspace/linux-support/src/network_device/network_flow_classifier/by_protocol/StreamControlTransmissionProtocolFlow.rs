// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Stream Control Transmission Protocol (SCTP) flow over either Internet Protocol version 4 or Internet Protocol version 6.
///
/// `ethtool` calls this either `sctp4` or `sctp6`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(bound(deserialize = "IPA: DeserializeOwned"))]
#[repr(transparent)]
pub struct StreamControlTransmissionProtocolFlow<IPA: InternetProtocolAddress + Unmasked>(CommonLayer4Flow<IPA>);

impl<IPA: InternetProtocolAddress + Unmasked> Deref for StreamControlTransmissionProtocolFlow<IPA>
{
	type Target = CommonLayer4Flow<IPA>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<IPA: InternetProtocolAddress + Unmasked> From<CommonLayer4Flow<IPA>> for StreamControlTransmissionProtocolFlow<IPA>
{
	#[inline(always)]
	fn from(common: CommonLayer4Flow<IPA>) -> Self
	{
		Self(common)
	}
}

impl DataAndMasks for StreamControlTransmissionProtocolFlow<in_addr>
{
	#[inline(always)]
	fn data_and_masks(&self) -> (ethtool_flow_union, ethtool_flow_union)
	{
		(
			ethtool_flow_union
			{
				sctp_ip4_spec: ethtool_tcpip4_spec
				{
					ip4src: self.layer_3.destination_address.underlying_data(),
					
					ip4dst: self.layer_3.source_address.underlying_data(),
					
					psrc: self.source_port.underlying_data(),
					
					pdst: self.destination_port.underlying_data(),
					
					tos: self.layer_3.type_of_service_or_traffic_class.underlying_data(),
				}
			},
			
			ethtool_flow_union
			{
				sctp_ip4_spec: ethtool_tcpip4_spec
				{
					ip4src: self.layer_3.destination_address.underlying_mask(),
					
					ip4dst: self.layer_3.source_address.underlying_mask(),
					
					psrc: self.source_port.underlying_mask(),
					
					pdst: self.destination_port.underlying_mask(),
					
					tos: self.layer_3.type_of_service_or_traffic_class.underlying_mask(),
				}
			},
		)
	}
}

impl DataAndMasks for StreamControlTransmissionProtocolFlow<in6_addr>
{
	#[inline(always)]
	fn data_and_masks(&self) -> (ethtool_flow_union, ethtool_flow_union)
	{
		(
			ethtool_flow_union
			{
				sctp_ip6_spec: ethtool_tcpip6_spec
				{
					ip6src: self.layer_3.destination_address.underlying_data(),
					
					ip6dst: self.layer_3.source_address.underlying_data(),
					
					psrc: self.source_port.underlying_data(),
					
					pdst: self.destination_port.underlying_data(),
					
					tclass: self.layer_3.type_of_service_or_traffic_class.underlying_data(),
				}
			},
			
			ethtool_flow_union
			{
				sctp_ip6_spec: ethtool_tcpip6_spec
				{
					ip6src: self.layer_3.destination_address.underlying_mask(),
					
					ip6dst: self.layer_3.source_address.underlying_mask(),
					
					psrc: self.source_port.underlying_mask(),
					
					pdst: self.destination_port.underlying_mask(),
					
					tclass: self.layer_3.type_of_service_or_traffic_class.underlying_mask(),
				}
			},
		)
	}
}
