// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Message data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetLinkMessageData
{
	/// Link flags.
	pub link_flags: net_device_flags,
	
	/// Device ARP type.
	pub device_arp_type: ARPHRD,
	
	/// Network interface index.
	pub network_interface_index: NetworkInterfaceIndex,
	
	/// Network interface name.
	pub network_interface_name: NetworkInterfaceName,
	
	/// Transmission queue length.
	pub transmission_queue_length: u32,
	
	/// RFC 2863 operational status.
	pub operational_status: IF_OPER,
	
	/// Link mode.
	pub link_mode: IF_LINK_MODE,
	
	/// Maximum Transmission Unit (MTU).
	pub maximum_transmission_unit: MaximumTransmissionUnit,
	
	/// Maximum Transmission Unit (MTU), minimum.
	pub maximum_transmission_unit_minimum: MaximumTransmissionUnit,
	
	/// Maximum Transmission Unit (MTU), maximum.
	pub maximum_transmission_unit_maximum: MaximumTransmissionUnit,
	
	/// Group this link belongs to.
	pub group: u32,
	
	/// Promiscuity count.
	///
	/// A non-zero value means the interface is acting promiscuously (seems to overlap with `link_flags` containing `net_device_flags::IFF_PROMISC`).
	pub promiscuity_count: u32,
	
	#[allow(missing_docs)]
	pub number_of_transmission_queues: QueueCount,
	
	/// Usually present unless the Linux kernel has not been configured with `CONFIG_RPS`.
	pub number_of_receive_queues: Option<QueueCount>,
	
	#[allow(missing_docs)]
	pub generic_segmentation_offload_maximum_segments: u32,
	
	#[allow(missing_docs)]
	pub generic_segmentation_offload_maximum_size: u32,
	
	/// Master if this is a slave network interface, eg in a bonded device.
	pub master_network_interface_index: Option<NetworkInterfaceIndex>,
	
	/// Is the 'carrier wave' present, ie is there a line (for a modem) or a network cable (for an ethernet device)?
	pub carrier_ok: bool,
	
	/// How many times the 'carrier wave' has gone from not present to present.
	pub carrier_up_count: u32,
	
	/// How many times the 'carrier wave' has gone from present to not present.
	pub carrier_down_count: u32,
	
	/// Is usually the sum of `carrier_up_count` and `carrier_down_count` but because these are read atomically and more than once internally by Linux (tsk, tsk) it may not be.
	pub carrier_up_and_down_count: u32,
	
	/// Queuing discipline.
	pub queueing_discipline: Option<CString>,
	
	/// Alias.
	pub network_interface_alias: Option<NetworkInterfaceAlias>,
	
	/// ?
	pub proto_down: bool,
	
	/// Target net namespace identifier.
	pub target_net_namespace_identifier: Option<i32>,
	
	/// Linked net namespace identifier.
	pub linked_net_namespace_identifier: Option<i32>,
	
	/// Linked network interface index.
	pub linked_network_interface_index: Option<NetworkInterfaceIndex>,
	
	/// New net namespace identifier.
	pub new_net_namespace_identifier: Option<i32>,
	
	/// New network interface index.
	pub new_network_interface_index: Option<NetworkInterfaceIndex>,
	
	/// Was this link data sent because of an event? If so, what was it?
	pub event: IFLA_EVENT,
	
	/// Map.
	pub map: rtnl_link_ifmap,
	
	/// Hardware unicast address.
	///
	/// Nearly always an Ethernet MAC address (length of 6).
	///
	/// If `Some` then all `HardwareAddress` will have the same length.
	/// The permanent address is probably the 'burned into ROM' Ethernet Media Access Control address.
	pub address_and_broadcast_and_permanent_address: Option<(HardwareAddress, HardwareAddress, Option<HardwareAddress>)>,
	
	#[allow(missing_docs)]
	pub physical_port_identifier: Option<PhysicalIdentifier>,
	
	#[allow(missing_docs)]
	pub physical_port_name: Option<CString>,
	
	#[allow(missing_docs)]
	pub physical_switch_identifier: Option<PhysicalIdentifier>,
	
	/// Statistics.
	pub(crate) statistics: Option<rtnl_link_stats64>,
	
	/// Attached eXpress Data Path (XDP) program identifiers.
	pub(crate) attached_express_data_path_program_identifiers: Option<MultipleProgramIdentifiers>,
}

#[allow(missing_docs)]
impl GetLinkMessageData
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_for(&self, network_interface_name: &NetworkInterfaceName) -> bool
	{
		Some(&self.network_interface_name) == Some(network_interface_name)
	}
}
