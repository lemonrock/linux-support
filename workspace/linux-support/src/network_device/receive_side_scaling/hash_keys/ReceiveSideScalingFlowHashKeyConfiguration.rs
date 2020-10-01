// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Configuration.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiveSideScalingFlowHashKeyConfiguration
{
	pub flow_hash_key: ReceiveSideScalingFlowHashKey,
	
	#[serde(default)] pub discard: bool,
}

impl ReceiveSideScalingFlowHashKeyConfiguration
{
	#[inline(always)]
	pub(crate) fn to_ethtool_rxnfc(&self, receive_side_scaling_context: Option<Option<ContextIdentifier>>) -> ethtool_rxnfc
	{
		self.flow_hash_key.to_ethtool_rxnfc(receive_side_scaling_context, self.discard)
	}
	
	#[inline(always)]
	pub(crate) fn parse(command: ethtool_rxnfc, expected_flow_type: u32) -> Self
	{
		debug_assert_eq!(command.flow_type, expected_flow_type);
		
		let rxh = RXH::from_bits_truncate(command.data);
		
		let actual_flow_type = expected_flow_type & !FLOW_RSS;
		
		use self::ReceiveSideScalingFlowHashKey::*;
		
		Self
		{
			flow_hash_key:  match actual_flow_type
			{
				TCP_V4_FLOW => TransmissionControlProtocolOverInternetProtocolVersion4(TransmissionControlProtocolReceiveSideScalingFlowHashKey(Layer4ReceiveSideScalingFlowHashKey::from(rxh))),
				
				UDP_V4_FLOW => UserDatagramProtocolOverInternetProtocolVersion4(UserDatagramProtocolReceiveSideScalingFlowHashKey(Layer4ReceiveSideScalingFlowHashKey::from(rxh))),
				
				SCTP_V4_FLOW => StreamTransmissionControlProtocolOverInternetProtocolVersion4(StreamTransmissionControlProtocolReceiveSideScalingFlowHashKey(Layer4ReceiveSideScalingFlowHashKey::from(rxh))),
				
				AH_ESP_V4_FLOW => IpsecOverInternetProtocolVersion4(IpsecReceiveSideScalingFlowHashKey::from(rxh)),
				
				TCP_V6_FLOW => TransmissionControlProtocolOverInternetProtocolVersion6(TransmissionControlProtocolReceiveSideScalingFlowHashKey(Layer4ReceiveSideScalingFlowHashKey::from(rxh))),
				
				UDP_V6_FLOW => UserDatagramProtocolOverInternetProtocolVersion6(UserDatagramProtocolReceiveSideScalingFlowHashKey(Layer4ReceiveSideScalingFlowHashKey::from(rxh))),
				
				SCTP_V6_FLOW => StreamTransmissionControlProtocolOverInternetProtocolVersion6(StreamTransmissionControlProtocolReceiveSideScalingFlowHashKey(Layer4ReceiveSideScalingFlowHashKey::from(rxh))),
				
				AH_ESP_V6_FLOW => IpsecOverInternetProtocolVersion6(IpsecReceiveSideScalingFlowHashKey::from(rxh)),
				
				ETHER_FLOW => Ethernet(EthernetReceiveSideScalingFlowHashKey::from(rxh)),
				
				_ => unreachable!("Invalid flow type"),
			},
		
			discard: rxh.contains(RXH::Discard),
		}
	}
}
