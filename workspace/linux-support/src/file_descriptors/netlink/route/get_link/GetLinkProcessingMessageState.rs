// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Processing state.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetLinkProcessingMessageState
{
	link_flags: net_device_flags,
	
	device_arp_type: ARPHRD,
	
	network_interface_index: NetworkInterfaceIndex,
	
	pub(crate) network_interface_name: Option<NetworkInterfaceName>,
	
	pub(crate) transmission_queue_length: Option<u32>,
	
	pub(crate) operational_status: Option<IF_OPER>,
	
	pub(crate) link_mode: Option<IF_LINK_MODE>,
	
	pub(crate) maximum_transmission_unit: Option<MaximumTransmissionUnitPayloadSize>,
	
	pub(crate) maximum_transmission_unit_minimum: Option<MaximumTransmissionUnitPayloadSize>,
	
	pub(crate) maximum_transmission_unit_maximum: Option<MaximumTransmissionUnitPayloadSize>,
	
	pub(crate) group: Option<NetworkDeviceGroup>,
	
	pub(crate) promiscuity: Option<u32>,
	
	pub(crate) number_of_transmission_queues: Option<QueueCount>,
	
	pub(crate) generic_segmentation_offload_maximum_segments: Option<u32>,
	
	pub(crate) generic_segmentation_offload_maximum_size: Option<u32>,
	
	pub(crate) number_of_receive_queues: Option<QueueCount>,
	
	/// Optional.
	pub(crate) master_network_interface_index: Option<NetworkInterfaceIndex>,
	
	pub(crate) carrier_ok: Option<bool>,
	
	/// Optional.
	pub(crate) queueing_discipline: Option<QueuingDisciplineAlgorithm>,
	
	/// Optional.
	pub(crate) network_interface_alias: Option<NetworkInterfaceAlias>,
	
	/// Optional.
	pub(crate) alternative_network_interface_names: Option<Vec<NetworkInterfaceAlternativeName>>,
	
	pub(crate) address_family_specific: Option<(Option<InternetProtocolVersion4Details>, Option<InternetProtocolVersion6Details>)>,
	
	pub(crate) carrier_up_and_down_count: Option<u32>,
	
	pub(crate) proto_down: Option<bool>,
	
	pub(crate) protocol_down_reason_value: Option<NonZeroU32>,
	
	pub(crate) carrier_up_count: Option<u32>,
	
	pub(crate) carrier_down_count: Option<u32>,
	
	pub(crate) target_net_namespace_identifier: Option<NetNamespaceIdentifer>,
	
	pub(crate) linked_net_namespace_identifier: Option<NetNamespaceIdentifer>,
	
	pub(crate) linked_network_interface_index: Option<Option<NetworkInterfaceIndex>>,
	
	pub(crate) new_net_namespace_identifier: Option<NetNamespaceIdentifer>,
	
	pub(crate) new_network_interface_index: Option<NetworkInterfaceIndex>,
	
	pub(crate) map: Option<rtnl_link_ifmap>,
	
	/// If `Some` then `broadcast_address` is `Some`.
	pub(crate) address: Option<HardwareAddress>,
	
	/// If `Some` then `address` is `Some`.
	pub(crate) broadcast_address: Option<HardwareAddress>,
	
	/// May be `Some` only if `address` and `broadcast_address` are `Some`.
	pub(crate) permanent_address: Option<HardwareAddress>,
	
	pub(crate) physical_port_identifier: Option<PhysicalIdentifier>,
	
	pub(crate) physical_port_name: Option<CString>,
	
	pub(crate) physical_switch_identifier: Option<PhysicalIdentifier>,
	
	pub(crate) number_of_virtual_functions: Option<u32>,
	
	pub(crate) statistics: Option<rtnl_link_stats64>,
	
	pub(crate) express_data_path: Option<ExpressDataPathGetLinkMessageData>,
}

#[allow(missing_docs)]
impl GetLinkProcessingMessageState
{
	#[inline(always)]
	pub(crate) fn new(message_header: &ifinfomsg) -> Result<Self, String>
	{
		if unlikely!(message_header.ifi_family != AF_UNSPEC as u8)
		{
			return Err(format!("Linux kernel bug - ifi_family is not AF_UNSPEC"))
		}
		Ok
		(
			Self
			{
				link_flags: message_header.ifi_flags,
				device_arp_type: message_header.ifi_type,
				network_interface_index: message_header.ifi_index.ok_or(format!("Linux kernel bug - missing network interface index"))?,
				network_interface_name: None,
				transmission_queue_length: None,
				operational_status: None,
				link_mode: None,
				maximum_transmission_unit: None,
				maximum_transmission_unit_minimum: None,
				maximum_transmission_unit_maximum: None,
				group: None,
				promiscuity: None,
				number_of_transmission_queues: None,
				generic_segmentation_offload_maximum_segments: None,
				generic_segmentation_offload_maximum_size: None,
				number_of_receive_queues: None,
				master_network_interface_index: None,
				carrier_ok: None,
				queueing_discipline: None,
				network_interface_alias: None,
				alternative_network_interface_names: None,
				address_family_specific: None,
				carrier_up_and_down_count: None,
				proto_down: None,
				protocol_down_reason_value: None,
				carrier_up_count: None,
				carrier_down_count: None,
				target_net_namespace_identifier: None,
				linked_net_namespace_identifier: None,
				linked_network_interface_index: None,
				new_net_namespace_identifier: None,
				new_network_interface_index: None,
				map: None,
				address: None,
				broadcast_address: None,
				permanent_address: None,
				physical_port_identifier: None,
				physical_port_name: None,
				physical_switch_identifier: None,
				number_of_virtual_functions: None,
				statistics: None,
				express_data_path: None,
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn to_processed_message(self) -> Result<GetLinkMessageData, String>
	{
		Ok
		(
			GetLinkMessageData
			{
				link_flags: self.link_flags,
				
				device_arp_type: self.device_arp_type,
				
				network_interface_index: self.network_interface_index,
				
				network_interface_name: self.network_interface_name.ok_or(format!("Linux kernel bug - missing network_interface_name"))?,
				
				transmission_queue_length: self.transmission_queue_length.ok_or(format!("Linux kernel bug - missing transmission_queue_length"))?,
				
				operational_status: self.operational_status.ok_or(format!("Linux kernel bug - missing operational_status"))?,
				
				link_mode: self.link_mode.ok_or(format!("Linux kernel bug - missing link_mode"))?,
				
				maximum_transmission_unit: self.maximum_transmission_unit.ok_or(format!("Linux kernel bug - missing maximum_transmission_unit"))?,
				
				maximum_transmission_unit_minimum: self.maximum_transmission_unit_minimum.ok_or(format!("Linux kernel bug - missing maximum_transmission_unit_minimum"))?,
				
				maximum_transmission_unit_maximum: self.maximum_transmission_unit_maximum.ok_or(format!("Linux kernel bug - missing maximum_transmission_unit_maximum"))?,
				
				group: self.group.ok_or(format!("Linux kernel bug - missing group"))?,
				
				promiscuity_count: self.promiscuity.ok_or(format!("Linux kernel bug - missing promiscuity"))?,
				
				number_of_transmission_queues: self.number_of_transmission_queues.ok_or(format!("Linux kernel bug - missing number_of_transmission_queues"))?,
				
				number_of_receive_queues: self.number_of_receive_queues,
				
				generic_segmentation_offload_maximum_segments: self.generic_segmentation_offload_maximum_segments.ok_or(format!("Linux kernel bug - missing generic_segmentation_offload_maximum_segments"))?,
				
				generic_segmentation_offload_maximum_size: self.generic_segmentation_offload_maximum_size.ok_or(format!("Linux kernel bug - missing generic_segmentation_offload_maximum_size"))?,
				
				master_network_interface_index: self.master_network_interface_index,
				
				carrier_ok: self.carrier_ok.ok_or(format!("Linux kernel bug - missing generic_segmentation_offload_maximum_size"))?,
				
				queueing_discipline: self.queueing_discipline,
				
				network_interface_alias: self.network_interface_alias,
				
				alternative_network_interface_names: self.alternative_network_interface_names.unwrap_or(Vec::new()),
				
				internet_version_4_protocol_details: if let Some((internet_version_4_protocol_details, _)) = self.address_family_specific
				{
					internet_version_4_protocol_details
				}
				else
				{
					None
				},
				
				internet_version_6_protocol_details: if let Some((_, internet_version_6_protocol_details)) = self.address_family_specific
				{
					internet_version_6_protocol_details
				}
				else
				{
					None
				},
				
				carrier_up_and_down_count: self.carrier_up_and_down_count.ok_or(format!("Linux kernel bug - missing carrier_up_and_down_count"))?,
				
				protocol_down_and_reason_code:
				{
					let is_protocol_down = self.proto_down.ok_or(format!("Linux kernel bug - missing proto_down"))?;
					if is_protocol_down
					{
						// Only present in Linux ?5.8 onwards.
						Some(self.protocol_down_reason_value)
					}
					else
					{
						if self.protocol_down_reason_value.is_some()
						{
							return Err(format!("Linux kernel bug - missing proto_down_reason when proto_down present"))
						}
						None
					}
				},
				
				carrier_up_count: self.carrier_up_count.ok_or(format!("Linux kernel bug - missing carrier_up_count"))?,
				
				carrier_down_count: self.carrier_down_count.ok_or(format!("Linux kernel bug - missing carrier_down_count"))?,
				
				target_net_namespace_identifier: self.target_net_namespace_identifier,
				
				linked_net_namespace_identifier: self.linked_net_namespace_identifier,
				
				linked_network_interface_index: self.linked_network_interface_index,
				
				new_net_namespace_identifier: self.new_net_namespace_identifier,
				
				new_network_interface_index: self.new_network_interface_index,
				
				map: self.map.ok_or(format!("Linux kernel bug - missing map"))?,
				
				address_and_broadcast_and_permanent_address: match (self.address, self.broadcast_address)
				{
					(None, None) => None,
					
					(Some(address), Some(broadcast_address)) =>
					{
						if unlikely!(address.len() != broadcast_address.len())
						{
							return Err(format!("Linux kernel bug - address and broadcast_address have different lengths"))
						}
						
						if let Some(ref permanent_address) = self.permanent_address
						{
							if unlikely!(address.len() != permanent_address.len())
							{
								return Err(format!("Linux kernel bug - address and permanent_address have different lengths"))
							}
						}
						
						Some((address, broadcast_address, self.permanent_address))
					}
					
					(Some(_), None) => return Err(format!("Linux kernel bug - both address and broadcast_address should be provided not just address")),
					
					(None, Some(_)) => return Err(format!("Linux kernel bug - both address and broadcast_address should be provided not just broadcast_address")),
				},
				
				physical_port_identifier: self.physical_port_identifier,
				
				physical_port_name: self.physical_port_name,
				
				physical_switch_identifier: self.physical_switch_identifier,
				
				number_of_virtual_functions: match self.number_of_virtual_functions
				{
					None => None,
					
					Some(value) =>
					{
						let maximum = VirtualFunctionIndex::ExclusiveMaximum.0;
						if value > (maximum as u32)
						{
							return Err(format!("Linux kernel bug - number_of_virtual_functions `{:?}` exceeds maximum `{:?}`", value, maximum))
						}
						else
						{
							Some(value as u8)
						}
					}
				},
				
				statistics: self.statistics,
				
				attached_express_data_path_program_identifiers: self.express_data_path.ok_or(format!("Linux kernel bug - missing express_data_path"))?.program_identifier(),
			}
		)
	}
}
