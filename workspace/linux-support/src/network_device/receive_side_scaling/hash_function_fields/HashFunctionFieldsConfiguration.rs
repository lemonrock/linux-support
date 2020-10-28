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
	/// Best possible combination without packets being received out-of-order.
	#[inline(always)]
	pub fn amazon_ena_best_possible() -> IndexSet<HashFunctionFieldsConfiguration>
	{
		Self::amazon_ena_valid_combinations_of_hash_function_fields_configuration().clone()
	}
	
	/// Valid combinations of `HashFunctionFieldsConfiguration` for Amazon ENA.
	///
	/// All of these can be independently set.
	#[inline(always)]
	pub fn amazon_ena_valid_combinations_of_hash_function_fields_configuration() -> IndexSet<HashFunctionFieldsConfiguration>
	{
		use self::HashFunctionFields::*;
		
		const EthernetFields: EthernetHashFunctionFields = EthernetHashFunctionFields
		{
			include_ethernet_destination_address: true,
			
			include_virtual_local_area_network_tag: false,
		};
		
		const InternetProtocolFields: InternetProtocolHashFunctionFields = InternetProtocolHashFunctionFields
		{
			ethernet: EthernetFields,
			
			include_layer_3_protocol_number: false,
			
			include_internet_protocol_version_source_address: true,
			
			include_internet_protocol_version_destination_address: true,
		};
		
		const Layer4Fields: Layer4HashFunctionFields = Layer4HashFunctionFields
		{
			internet_protocol: InternetProtocolFields,
			
			include_source_port: true,
			
			include_destination_port: true,
		};
		
		indexset!
		[
			HashFunctionFieldsConfiguration::new_without_discard(Ethernet(EthernetFields)),
			HashFunctionFieldsConfiguration::new_without_discard(InternetProtocolVersion4(InternetProtocolFields)),
			HashFunctionFieldsConfiguration::new_without_discard(TransmissionControlProtocolOverInternetProtocolVersion4(Layer4Fields)),
			HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion4(Layer4Fields)),
			HashFunctionFieldsConfiguration::new_without_discard(InternetProtocolVersion6(InternetProtocolFields)),
			HashFunctionFieldsConfiguration::new_without_discard(TransmissionControlProtocolOverInternetProtocolVersion6(Layer4Fields)),
			HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion6(Layer4Fields)),
		]
	}
	
	/// Best possible combination without packets being received out-of-order.
	#[inline(always)]
	pub fn intel_ixgbevf_intel_fork_for_x550_or_later_valid_combinations_of_hash_function_fields_configuration_best_possible() -> IndexSet<HashFunctionFieldsConfiguration>
	{
		use self::HashFunctionFields::*;
		
		const EthernetFields: EthernetHashFunctionFields = EthernetHashFunctionFields
		{
			include_ethernet_destination_address: false,
			
			include_virtual_local_area_network_tag: false,
		};
		
		const InternetProtocolFields: InternetProtocolHashFunctionFields = InternetProtocolHashFunctionFields
		{
			ethernet: EthernetFields,
			
			include_layer_3_protocol_number: false,
			
			include_internet_protocol_version_source_address: true,
			
			include_internet_protocol_version_destination_address: true,
		};
		
		const Layer4FieldsWithPortNumbers: Layer4HashFunctionFields = Layer4HashFunctionFields
		{
			internet_protocol: InternetProtocolFields,
			
			include_source_port: true,
			
			include_destination_port: true,
		};
		
		const Layer4FieldsWithoutPortNumbers: Layer4HashFunctionFields = Layer4HashFunctionFields
		{
			internet_protocol: InternetProtocolFields,
			
			include_source_port: true,
			
			include_destination_port: true,
		};
		
		indexset!
		[
			HashFunctionFieldsConfiguration::new_without_discard(TransmissionControlProtocolOverInternetProtocolVersion4(Layer4FieldsWithPortNumbers)),
		
			HashFunctionFieldsConfiguration::new_without_discard(TransmissionControlProtocolOverInternetProtocolVersion6(Layer4FieldsWithPortNumbers)),
			
			// This *disables* the internal flag `IXGBE_MRQC_RSS_FIELD_IPV4_UDP`, ie it unsets UDP receive side scaling (RSS) hashing of flows.
			HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion4(Layer4FieldsWithoutPortNumbers)),
			
			// This *disables* the internal flag `IXGBE_MRQC_RSS_FIELD_IPV6_UDP`, ie it unsets UDP receive side scaling (RSS) hashing of flows.
			HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion6(Layer4FieldsWithoutPortNumbers)),
		]
	}
	
	/// Valid combinations of `HashFunctionFieldsConfiguration` for Intel ixgbevf (Intel out-of-tree fork) driver for Intel X550 or later models.
	///
	/// In practice, under the covers in the driver, all of these combinations translate into just two alternatives:-
	///
	/// * Receive side scaling (RSS) hashing with or without the flag `IXGBE_MRQC_RSS_FIELD_IPV4_UDP`.
	/// * Receive side scaling (RSS) hashing with or without the flag `IXGBE_MRQC_RSS_FIELD_IPV6_UDP`.
	///
	/// Otherwise, the following receive side scaling (RSS) hashing is always enabled for:-
	///
	/// * `IXGBE_MRQC_RSS_FIELD_IPV4`.
	/// * `IXGBE_MRQC_RSS_FIELD_IPV4_TCP`.
	/// * `IXGBE_MRQC_RSS_FIELD_IPV6`.
	/// * `IXGBE_MRQC_RSS_FIELD_IPV6_TCP`.
	///
	/// Additionally, it seems that enabling either `IXGBE_MRQC_RSS_FIELD_IPV4_UDP` or `IXGBE_MRQC_RSS_FIELD_IPV6_UDP` can cause fragmented UDP packets to arrive to the application's socket *out-of-order*!
	///
	/// So to enable RSS hashing:-
	///
	/// * Pass `HashFunctionFieldsConfiguration::new_without_discard(TransmissionControlProtocolOverInternetProtocolVersion4(Layer4FieldsWithPortNumbers))` and then `HashFunctionFieldsConfiguration::new_without_discard(TransmissionControlProtocolOverInternetProtocolVersion6(Layer4FieldsWithPortNumbers))`.
	/// * Then pass `HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion4(Layer4FieldsWithoutPortNumbers))` and `HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion6(Layer4FieldsWithoutPortNumbers))`.
	/// * Then, if hashing of UDP packets is desired, pass `HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion4(Layer4FieldsWithPortNumbers))` and `HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion6(Layer4FieldsWithPortNumbers))`.
	#[inline(always)]
	pub fn intel_ixgbevf_intel_fork_for_x550_or_later_valid_combinations_of_hash_function_fields_configuration() -> IndexSet<HashFunctionFieldsConfiguration>
	{
		use self::HashFunctionFields::*;
		
		const EthernetFields: EthernetHashFunctionFields = EthernetHashFunctionFields
		{
			include_ethernet_destination_address: false,
			
			include_virtual_local_area_network_tag: false,
		};
		
		const InternetProtocolFields: InternetProtocolHashFunctionFields = InternetProtocolHashFunctionFields
		{
			ethernet: EthernetFields,
			
			include_layer_3_protocol_number: false,
			
			include_internet_protocol_version_source_address: true,
			
			include_internet_protocol_version_destination_address: true,
		};
		
		const Layer4FieldsWithPortNumbers: Layer4HashFunctionFields = Layer4HashFunctionFields
		{
			internet_protocol: InternetProtocolFields,
			
			include_source_port: true,
			
			include_destination_port: true,
		};
		
		const Layer4FieldsWithoutPortNumbers: Layer4HashFunctionFields = Layer4HashFunctionFields
		{
			internet_protocol: InternetProtocolFields,
			
			include_source_port: true,
			
			include_destination_port: true,
		};
		
		const IpsecFields: IpsecHashFunctionFields = IpsecHashFunctionFields
		{
			internet_protocol: InternetProtocolFields,
			
			include_security_parameter_index: false,
		};
		
		indexset!
		[
			HashFunctionFieldsConfiguration::new_without_discard(TransmissionControlProtocolOverInternetProtocolVersion4(Layer4FieldsWithPortNumbers)),
		
			HashFunctionFieldsConfiguration::new_without_discard(TransmissionControlProtocolOverInternetProtocolVersion6(Layer4FieldsWithPortNumbers)),
			
			HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion4(Layer4FieldsWithPortNumbers)),
			
			// This *disables* the internal flag `IXGBE_MRQC_RSS_FIELD_IPV4_UDP`, ie it unsets UDP receive side scaling (RSS) hashing of flows.
			HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion4(Layer4FieldsWithoutPortNumbers)),
			
			HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion6(Layer4FieldsWithPortNumbers)),
			
			// This *disables* the internal flag `IXGBE_MRQC_RSS_FIELD_IPV6_UDP`, ie it unsets UDP receive side scaling (RSS) hashing of flows.
			HashFunctionFieldsConfiguration::new_without_discard(UserDatagramProtocolOverInternetProtocolVersion6(Layer4FieldsWithoutPortNumbers)),
			
			HashFunctionFieldsConfiguration::new_without_discard(StreamTransmissionControlProtocolOverInternetProtocolVersion4(Layer4FieldsWithoutPortNumbers)),
			
			HashFunctionFieldsConfiguration::new_without_discard(StreamTransmissionControlProtocolOverInternetProtocolVersion6(Layer4FieldsWithoutPortNumbers)),
			
			HashFunctionFieldsConfiguration::new_without_discard(IpsecOverInternetProtocolVersion4(IpsecFields)),
			
			HashFunctionFieldsConfiguration::new_without_discard(IpsecOverInternetProtocolVersion6(IpsecFields)),
		]
	}
	
	/// New.
	#[inline(always)]
	pub const fn new_without_discard(hash_function_fields: HashFunctionFields) -> Self
	{
		Self
		{
			hash_function_fields,
		
			discard: false,
		}
	}
	
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
				
				AH_V4_FLOW | AH_V6_FLOW | ESP_V4_FLOW | ESP_V6_FLOW => unreachable_code(format_args!("Never specified in original ethtool syscall")),
				
				IPV4_USER_FLOW | IPV6_USER_FLOW => unreachable_code(format_args!("Never specified in original ethtool syscall and invalid as a hash key")),
				
				_ => unreachable_code(format_args!("Invalid flow type")),
			},
		
			discard: rxh.contains(RXH::Discard),
		}
	}
}
