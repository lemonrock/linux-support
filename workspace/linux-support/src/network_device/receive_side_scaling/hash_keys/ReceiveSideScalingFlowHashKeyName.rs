// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl ReceiveSideScalingFlowHashKeyName
{
	#[inline(always)]
	pub(crate) fn to_ethtool_rxnfc(&self, receive_side_scaling_context: Option<Option<ContextIdentifier>>) -> ethtool_rxnfc
	{
		let (has_receive_side_scaling_context, receive_side_scaling_context) = match receive_side_scaling_context
		{
			Some(receive_side_scaling_context) => (true, receive_side_scaling_context),
			
			None => (false, None)
		};
		
		let actual_flow_type = self.to_actual_flow_type();
		
		let nfcmd = ethtool_rxnfc
		{
			cmd: ETHTOOL_GRXFH,
			
			flow_type: if has_receive_side_scaling_context
			{
				actual_flow_type | FLOW_RSS
			}
			else
			{
				actual_flow_type
			},
			
			rule_count_or_rss_context: ethtool_rxnfc_rule_count_or_rss_context
			{
				rss_context: receive_side_scaling_context,
			},
			
			data: data_field.bits,
			
			fs: unsafe { zeroed() },
			
			rule_locs: Default::default(),
		};
	}
	
	#[inline(always)]
	fn to_actual_flow_type(self) -> u32
	{
		use self::ReceiveSideScalingFlowHashKeyName::*;
		
		match self
		{
			InternetProtocolVersion4 => IPV4_FLOW,
			
			TransmissionControlProtocolOverInternetProtocolVersion4 => TCP_V4_FLOW,
			
			UserDatagramProtocolOverInternetProtocolVersion4 => UDP_V4_FLOW,
			
			StreamTransmissionControlProtocolOverInternetProtocolVersion4 => SCTP_V4_FLOW,
			
			IpsecOverInternetProtocolVersion4 => AH_ESP_V4_FLOW,
			
			InternetProtocolVersion6 => IPV6_FLOW,
			
			TransmissionControlProtocolOverInternetProtocolVersion6 => TCP_V6_FLOW,
			
			UserDatagramProtocolOverInternetProtocolVersion6 => UDP_V6_FLOW,
			
			StreamTransmissionControlProtocolOverInternetProtocolVersion6 => SCTP_V6_FLOW,
			
			IpsecOverInternetProtocolVersion6 => AH_ESP_V6_FLOW,
			
			Ethernet => ETHER_FLOW,
		}
	}
}
