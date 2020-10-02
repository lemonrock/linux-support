// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Fields from incoming packet to use in the hash function, and whether to drop matching packets.
///
/// Partly supported by Amazon ENA.
/// Totally unsupported by Intel ixgbevf.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HashFunctionFieldsConfiguration
{
	/// Fields from incoming packet to use in the hash function.
	pub hash_function_fields: HashFunctionFields,
	
	/// Unsupported by Amazon ENA.
	#[serde(default)] pub discard: bool,
}

impl HashFunctionFieldsConfiguration
{
	#[inline(always)]
	pub(crate) fn to_ethtool_rxnfc(&self, receive_side_scaling_context: Option<ContextIdentifier>) -> ethtool_rxnfc
	{
		self.hash_function_fields.to_ethtool_rxnfc(receive_side_scaling_context, self.discard)
	}
	
	#[inline(always)]
	pub(crate) fn parse(command: ethtool_rxnfc, expected_flow_type: u32) -> Self
	{
		debug_assert_eq!(command.flow_type, expected_flow_type, "ethtool syscall mutated flow_type");
		
		let rxh = RXH::from_bits_truncate(command.data);
		
		let actual_flow_type = expected_flow_type & !FLOW_RSS;
		
		use self::HashFunctionFields::*;
		
		Self
		{
			hash_function_fields:  match actual_flow_type
			{
				ETHER_FLOW => Ethernet(EthernetHashFunctionFields::from(rxh)),
				
				IPV4_FLOW => InternetProtocolVersion4(InternetProtocolHashFunctionFields::from(rxh)),
				
				TCP_V4_FLOW => TransmissionControlProtocolOverInternetProtocolVersion4(Layer4HashFunctionFields::from(rxh)),
				
				UDP_V4_FLOW => UserDatagramProtocolOverInternetProtocolVersion4(Layer4HashFunctionFields::from(rxh)),
				
				SCTP_V4_FLOW => StreamTransmissionControlProtocolOverInternetProtocolVersion4(Layer4HashFunctionFields::from(rxh)),
				
				AH_ESP_V4_FLOW => IpsecOverInternetProtocolVersion4(IpsecHashFunctionFields::from(rxh)),
				
				IPV6_FLOW => InternetProtocolVersion6(InternetProtocolHashFunctionFields::from(rxh)),
				
				TCP_V6_FLOW => TransmissionControlProtocolOverInternetProtocolVersion6(Layer4HashFunctionFields::from(rxh)),
				
				UDP_V6_FLOW => UserDatagramProtocolOverInternetProtocolVersion6(Layer4HashFunctionFields::from(rxh)),
				
				SCTP_V6_FLOW => StreamTransmissionControlProtocolOverInternetProtocolVersion6(Layer4HashFunctionFields::from(rxh)),
				
				AH_ESP_V6_FLOW => IpsecOverInternetProtocolVersion6(IpsecHashFunctionFields::from(rxh)),
				
				AH_V4_FLOW | AH_V6_FLOW | ESP_V4_FLOW | ESP_V6_FLOW => unreachable!("Never specified in original ethtool syscall"),
				
				IPV4_USER_FLOW | IPV6_USER_FLOW => unreachable!("Never specified in original ethtool syscall and invalid as a hash key"),
				
				_ => unreachable!("Invalid flow type"),
			},
		
			discard: rxh.contains(RXH::Discard),
		}
	}
}
