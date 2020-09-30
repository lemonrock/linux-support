// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Layer 4 flow which matches on first 4 bytes of transport and specific transport protocol number.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields, bound(deserialize = "IPA: DeserializeOwned"))]
pub struct UserLayer4Flow<IPA: InternetProtocolAddress + Unmasked>
{
	#[allow(missing_docs)]
	#[serde(flatten)] pub layer_3: CommonLayer3Flow<IPA>,
	
	/// First 4 bytes of transport (layer 4) header.
	///
	/// This can also be:-
	///
	/// * `security_parameter_index`;
	/// * `(source_port, destination_port)`;
	pub first_four_bytes_of_layer4_header: MaskedData<Layer4FourBytes>,
	
	/// Transport protocol number.
	///
	/// `transport_protocol_number` when masked must always be `0xFF` hence this doesn't keep a mask.
	pub transport_protocol_number: Option<u8>,
}

impl<IPA: InternetProtocolAddress + Unmasked> Deref for UserLayer4Flow<IPA>
{
	type Target = CommonLayer3Flow<IPA>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.layer_3
	}
}

impl<IPA: InternetProtocolAddress + Unmasked> UserLayer4Flow<IPA>
{
	pub(super) fn new<UFS: UserFlowSpecification<IPA::Underlying>>(data: &UFS, mask: &UFS) -> Result<Self, BasicFlowParseError>
	{
		Ok
		(
			Self
			{
				layer_3: CommonLayer3Flow::new(data, mask),
				
				first_four_bytes_of_layer4_header: MaskedData::from_underlying_data_and_mask(data.first_four_bytes_of_layer4_header(), mask.first_four_bytes_of_layer4_header()),
				
				transport_protocol_number: match mask.transport_protocol_number()
				{
					0 => None,
					
					0xFF => Some(data.transport_protocol_number()),
					
					transport_protocol_number_mask @ _ => return Err(BasicFlowParseError::UserLayer4FlowTransportProtocolNumberMaskMustEitherBeZeroOr0xFF { transport_protocol_number_mask })
				},
			}
		)
	}
	
	#[inline(always)]
	fn protocol_and_mask(&self) -> (u8, u8)
	{
		match self.transport_protocol_number
		{
			None => (0, 0),
			Some(transport_protocol_number) => (transport_protocol_number, 0xFF),
		}
	}
}

impl DataAndMasks for UserLayer4Flow<in_addr>
{
	#[inline(always)]
	fn data_and_masks(&self) -> (ethtool_flow_union, ethtool_flow_union)
	{
		let (proto, proto_masked) = self.protocol_and_mask();
		
		(
			ethtool_flow_union
			{
				usr_ip4_spec: ethtool_usrip4_spec
				{
					ip4src: self.layer_3.destination_address.underlying_data(),
					
					ip4dst: self.layer_3.source_address.underlying_data(),
					
					l4_4_bytes: self.first_four_bytes_of_layer4_header.underlying_data(),
					
					tos: self.layer_3.type_of_service_or_traffic_class.underlying_data(),
					
					ip_ver: ETH_RX_NFC_IP4,
					
					proto,
				}
			},
			
			ethtool_flow_union
			{
				usr_ip4_spec: ethtool_usrip4_spec
				{
					ip4src: self.layer_3.destination_address.underlying_mask(),
					
					ip4dst: self.layer_3.source_address.underlying_mask(),
					
					l4_4_bytes: self.first_four_bytes_of_layer4_header.underlying_mask(),
					
					tos: self.layer_3.type_of_service_or_traffic_class.underlying_mask(),
				
					ip_ver: 0,
				
					proto: proto_masked,
				}
			},
		)
	}
}

impl DataAndMasks for UserLayer4Flow<in6_addr>
{
	#[inline(always)]
	fn data_and_masks(&self) -> (ethtool_flow_union, ethtool_flow_union)
	{
		let (l4_proto, l4_proto_masked) = self.protocol_and_mask();
		
		(
			ethtool_flow_union
			{
				usr_ip6_spec: ethtool_usrip6_spec
				{
					ip6src: self.layer_3.destination_address.underlying_data(),
					
					ip6dst: self.layer_3.source_address.underlying_data(),
					
					l4_4_bytes: self.first_four_bytes_of_layer4_header.underlying_data(),
					
					tclass: self.layer_3.type_of_service_or_traffic_class.underlying_data(),
					
					l4_proto,
				}
			},
			
			ethtool_flow_union
			{
				usr_ip6_spec: ethtool_usrip6_spec
				{
					ip6src: self.layer_3.destination_address.underlying_mask(),
					
					ip6dst: self.layer_3.source_address.underlying_mask(),
					
					l4_4_bytes: self.first_four_bytes_of_layer4_header.underlying_mask(),
					
					tclass: self.layer_3.type_of_service_or_traffic_class.underlying_mask(),
				
					l4_proto: l4_proto_masked,
				}
			},
		)
	}
}
