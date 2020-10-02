// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Fields from incoming packet to use in the hash function.
///
/// Totally unsupported by Intel ixgbevf.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(HashFunctionFieldsName))]
#[strum_discriminants(derive(PartialOrd))]
#[strum_discriminants(derive(Ord))]
#[strum_discriminants(derive(Hash))]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(derive(EnumCount))]
#[strum_discriminants(derive(Deserialize))]
#[strum_discriminants(derive(Serialize))]
pub enum HashFunctionFields
{
	/// * Partly supported by Amazon ENA.
	Ethernet(EthernetHashFunctionFields),
	
	/// * Partly supported by Amazon ENA.
	InternetProtocolVersion4(InternetProtocolHashFunctionFields),
	
	/// * Partly supported by Amazon ENA.
	TransmissionControlProtocolOverInternetProtocolVersion4(Layer4HashFunctionFields),
	
	/// * Partly supported by Amazon ENA.
	UserDatagramProtocolOverInternetProtocolVersion4(Layer4HashFunctionFields),
	
	/// * Totally unsupported by Amazon ENA.
	StreamTransmissionControlProtocolOverInternetProtocolVersion4(Layer4HashFunctionFields),
	
	/// * Totally unsupported by Amazon ENA.
	IpsecOverInternetProtocolVersion4(IpsecHashFunctionFields),
	
	/// * Partly supported by Amazon ENA.
	InternetProtocolVersion6(InternetProtocolHashFunctionFields),
	
	/// * Partly supported by Amazon ENA.
	TransmissionControlProtocolOverInternetProtocolVersion6(Layer4HashFunctionFields),
	
	/// * Partly supported by Amazon ENA.
	UserDatagramProtocolOverInternetProtocolVersion6(Layer4HashFunctionFields),
	
	/// * Totally unsupported by Amazon ENA.
	StreamTransmissionControlProtocolOverInternetProtocolVersion6(Layer4HashFunctionFields),
	
	/// * Totally unsupported by Amazon ENA.
	IpsecOverInternetProtocolVersion6(IpsecHashFunctionFields),
}

impl HashFunctionFields
{
	#[allow(deprecated)]
	pub(crate) fn to_ethtool_rxnfc(&self, receive_side_scaling_context: Option<ContextIdentifier>, discard: bool) -> ethtool_rxnfc
	{
		let (actual_flow_type, mut data_field) = self.to_actual_flow_type_and_data_field();
		
		if discard
		{
			data_field |= RXH::Discard
		}
		
		ethtool_rxnfc
		{
			cmd: ETHTOOL_SRXFH,
			
			flow_type: if receive_side_scaling_context.is_some()
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
			
			fs: unsafe { uninitialized() },
			
			rule_locs: unsafe { uninitialized() },
		}
	}
	
	#[inline(always)]
	fn to_actual_flow_type_and_data_field(&self) -> (u32, RXH)
	{
		use self::HashFunctionFields::*;
		
		match self
		{
			Ethernet(hash_key) => (ETHER_FLOW, hash_key.to_data_field()),
			
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
		}
	}
}
