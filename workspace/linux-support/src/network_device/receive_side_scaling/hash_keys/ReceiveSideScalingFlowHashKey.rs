// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(ReceiveSideScalingFlowHashKeyName))]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(derive(EnumCount))]
#[strum_discriminants(derive(Deserialize))]
#[strum_discriminants(derive(Serialize))]
pub enum ReceiveSideScalingFlowHashKey
{
	/// * Partly supported by Amazon ENA.
	/// * Totally unsupported by Intel ixgbevf.
	TransmissionControlProtocolOverInternetProtocolVersion4(TransmissionControlProtocolReceiveSideScalingFlowHashKey),
	
	/// * Partly supported by Amazon ENA.
	/// * Totally unsupported by Intel ixgbevf.
	UserDatagramProtocolOverInternetProtocolVersion4(UserDatagramProtocolReceiveSideScalingFlowHashKey),
	
	/// * Totally unsupported by Amazon ENA.
	/// * Totally unsupported by Intel ixgbevf.
	StreamTransmissionControlProtocolOverInternetProtocolVersion4(StreamTransmissionControlProtocolReceiveSideScalingFlowHashKey),
	
	/// * Totally unsupported by Amazon ENA.
	/// * Totally unsupported by Intel ixgbevf.
	IpsecOverInternetProtocolVersion4(IpsecReceiveSideScalingFlowHashKey),
	
	/// * Partly supported by Amazon ENA.
	/// * Totally unsupported by Intel ixgbevf.
	TransmissionControlProtocolOverInternetProtocolVersion6(TransmissionControlProtocolReceiveSideScalingFlowHashKey),
	
	/// * Partly supported by Amazon ENA.
	/// * Totally unsupported by Intel ixgbevf.
	UserDatagramProtocolOverInternetProtocolVersion6(UserDatagramProtocolReceiveSideScalingFlowHashKey),
	
	/// * Totally unsupported by Amazon ENA.
	/// * Totally unsupported by Intel ixgbevf.
	StreamTransmissionControlProtocolOverInternetProtocolVersion6(StreamTransmissionControlProtocolReceiveSideScalingFlowHashKey),
	
	/// * Totally unsupported by Amazon ENA (and .
	/// * Totally unsupported by Intel ixgbevf.
	IpsecOverInternetProtocolVersion6(IpsecReceiveSideScalingFlowHashKey),
	
	/// * Partly supported by Amazon ENA.
	/// * Totally unsupported by Intel ixgbevf.
	InternetProtocolVersion4(InternetProtocolReceiveSideScalingFlowHashKey),
	
	/// * Partly supported by Amazon ENA.
	/// * Totally unsupported by Intel ixgbevf.
	InternetProtocolVersion6(InternetProtocolReceiveSideScalingFlowHashKey),
	
	/// * Partly supported by Amazon ENA.
	/// * Totally unsupported by Intel ixgbevf.
	Ethernet(EthernetReceiveSideScalingFlowHashKey),
}

impl ReceiveSideScalingFlowHashKey
{
	pub(crate) fn to_ethtool_rxnfc(&self, receive_side_scaling_context: Option<Option<ContextIdentifier>>, discard: bool) -> ethtool_rxnfc
	{
		let (has_receive_side_scaling_context, receive_side_scaling_context) = match receive_side_scaling_context
		{
			Some(receive_side_scaling_context) => (true, receive_side_scaling_context),
			
			None => (false, None)
		};
		
		let (actual_flow_type, data_field) = self.to_actual_flow_type_and_data_field();
		
		let nfcmd = ethtool_rxnfc
		{
			cmd: ETHTOOL_SRXFH,
			
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
	fn to_actual_flow_type_and_data_field(&self) -> (u32, RXH)
	{
		use self::ReceiveSideScalingFlowHashKey::*;
		
		let (actual_flow_type, data) = match self
		{
			InternetProtocolVersion4(hash_key) => (IPV4_FLOW, hash_key.to_data_field()),
			
			TransmissionControlProtocolOverInternetProtocolVersion4(hash_key) => (TCP_V4_FLOW, hash_key.to_data_field()),
			
			UserDatagramProtocolOverInternetProtocolVersion4(hash_key) => (UDP_V4_FLOW, hash_key.to_data_field()),
			
			StreamTransmissionControlProtocolOverInternetProtocolVersion4(hash_key) => (SCTP_V4_FLOW, hash_key.to_data_field()),
			
			IpsecOverInternetProtocolVersion4(hash_key) => (AH_ESP_V4_FLOW, hash_key.to_data_field()),
			
			InternetProtocolVersion6(hash_key) => (IPV6_FLOW, hash_key.to_data_field()),
			
			TransmissionControlProtocolOverInternetProtocolVersion6(hash_key) => (TCP_V6_FLOW, hash_key.to_data_field()),
			
			UserDatagramProtocolOverInternetProtocolVersion6(hash_key) => (UDP_V6_FLOW, hash_key.to_data_field()),
			
			StreamTransmissionControlProtocolOverInternetProtocolVersion6(hash_key) => (SCTP_V6_FLOW, hash_key.to_data_field()),
			
			IpsecOverInternetProtocolVersion6(hash_key) => (AH_ESP_V6_FLOW, hash_key.to_data_field()),
			
			Ethernet(hash_key) => (ETHER_FLOW, hash_key.to_data_field()),
		};
	}
}
