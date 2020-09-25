// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Message data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetLinkMessageData
{
	/// Link flags.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/flags`.
	/// Also available via the ioctl `SIOCGIFFLAGS`.
	/// Also settable via the ioctl `SIOCSIFFLAGS`.
	/// Read-write via sysfs.
	///
	/// See also `NetworkDeviceInputOutputControl::link_flags()`.
	pub link_flags: net_device_flags,
	
	/// Device ARP type.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/type`.
	pub device_arp_type: ARPHRD,
	
	/// Network interface index.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/ifindex`.
	/// Also available via the ioctl `SIOCGIFINDEX`.
	pub network_interface_index: NetworkInterfaceIndex,
	
	/// Network interface name.
	///
	/// Also settable via the ioctl `SIOCSIFNAME`.
	pub network_interface_name: NetworkInterfaceName,
	
	/// Transmission queue length.
	///
	/// Defaults to `1000` for Ethernet devices.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/tx_queue_len`.
	/// Also available via the ioctl `SIOCGIFTXQLEN`.
	/// Also settable via the ioctl `SIOCSIFTXQLEN`.
	/// Read-write via sysfs.
	///
	/// Seems to be different to `PendingQueueDepths.transmit_pending_queue_depth` and hence `ethtool_ringparam.tx_pending`.
	pub transmission_queue_length: u32,
	
	/// RFC 2863 operational status.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/oper_state`, where it is turned into a string:-
	///
	/// * `unknown`.
	/// * `notpresent` (apparently unused according to a Linux kernel source comment).
	/// * `down`.
	/// * `lowerlayerdown`.
	/// * `testing` (apparently unused according to a Linux kernel source comment).
	/// * `dormant`.
	/// * `up`.
	///
	/// Also reported as a boolean in `/sys/class/net/<network_interface_name>/dormant` which should be checked before relying on `IF_OPER::IF_OPER_UP` or `IF_OPER::IF_OPER_DORMANT`. Value of `dormant` which:-
	///
	/// * if true, reflects that a driver has been placed in the `dormant` state but the operational status won't yet have been updated to `IF_OPER_DORMANT`.
	/// * if false, reflects that a driver has never been placed in the `dormant` state, or, rarely, is exiting from a `dormant` state but the operational status won't yet have been updated to `IF_OPER_UP`.
	///
	/// Rarely-used state by a very small number of Linux drivers, none of which are common.
	///
	/// See also `NetworkInterfaceName.dormant()`.
	pub operational_status: IF_OPER,
	
	/// Link mode.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/link_mode`.
	pub link_mode: IF_LINK_MODE,
	
	/// Maximum Transmission Unit (MTU).
	///
	/// Also available at `/sys/class/net/<network_interface_name>/mtu`.
	/// Also available via the ioctl `SIOCGIFMTU`.
	/// Also settable via the ioctl `SIOCSIFMTU`.
	/// Read-write via sysfs.
	pub maximum_transmission_unit: MaximumTransmissionUnitPayloadSize,
	
	/// Maximum Transmission Unit (MTU), minimum.
	pub maximum_transmission_unit_minimum: MaximumTransmissionUnitPayloadSize,
	
	/// Maximum Transmission Unit (MTU), maximum.
	pub maximum_transmission_unit_maximum: MaximumTransmissionUnitPayloadSize,
	
	/// Group this link belongs to.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/netdev_group`.
	/// Read-write via sysfs.
	///
	/// The initial net device group is `INIT_NETDEV_GROUP` (`0`).
	/// All devices belong to group 0 by default.
	pub group: NetworkDeviceGroup,
	
	/// Promiscuity count.
	///
	/// A non-zero value means the interface is acting promiscuously (seems to overlap with `link_flags` containing `net_device_flags::IFF_PROMISC`).
	pub promiscuity_count: u32,
	
	#[allow(missing_docs)]
	pub number_of_transmission_queues: QueueCount,
	
	/// Usually present unless the Linux kernel has not been configured with `CONFIG_RPS`.
	pub number_of_receive_queues: Option<QueueCount>,
	
	/// Can not exceed `GSO_MAX_SIZE`, 65536.
	pub generic_segmentation_offload_maximum_segments: u32,
	
	/// Can not exceed `GSO_MAX_SEGS`, 65535.
	pub generic_segmentation_offload_maximum_size: u32,
	
	/// Master if this is a slave network interface, eg in a bonded device.
	pub master_network_interface_index: Option<NetworkInterfaceIndex>,
	
	/// Is the 'carrier wave' present, ie is there a line (for a modem) or a network cable (for an ethernet device)?
	///
	/// Also available at `/sys/class/net/<network_interface_name>/carrier`.
	/// Read-write via sysfs.
	pub carrier_ok: bool,
	
	/// How many times the 'carrier wave' has gone from not present to present.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/carrier_up_count`.
	pub carrier_up_count: u32,
	
	/// How many times the 'carrier wave' has gone from present to not present.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/carrier_down_count`.
	pub carrier_down_count: u32,
	
	/// Is usually the sum of `carrier_up_count` and `carrier_down_count` but because these are read atomically and more than once internally by Linux (tsk, tsk) it may not be.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/carrier_changes`.
	pub carrier_up_and_down_count: u32,
	
	/// Queuing discipline.
	pub queueing_discipline: Option<QueuingDisciplineAlgorithm>,
	
	/// Alias.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/ifalias` (empty if unset).
	/// Read-write via sysfs.
	pub network_interface_alias: Option<NetworkInterfaceAlias>,
	
	/// Alternative names.
	pub network_interface_alternative_names: Vec<NetworkInterfaceAlternativeName>,
	
	/// Internet Protocol version 4 details.
	pub internet_version_4_protocol_details: Option<InternetProtocolVersion4Details>,
	
	/// Internet Protocol version 6 details.
	pub internet_version_6_protocol_details: Option<InternetProtocolVersion6Details>,
	
	/// `None` is `false`.
	/// `Some(..)` is `true`.
	/// `Some(None)` is `true` and Linux does not provide a reason code (older versions) or the reason code was `0` ( which may not be a valid possibility; Linux does not have clarity here).
	/// `Some(Some(reason))` is `true` and the reason code is provided by Linux.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/proto_down`.
	/// Read-write via sysfs as boolean.
	pub protocol_down_and_reason_code: Option<Option<NonZeroU32>>,
	
	/// Target net namespace identifier.
	pub target_net_namespace_identifier: Option<i32>,
	
	/// Linked net namespace identifier.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/iflink`.
	pub linked_net_namespace_identifier: Option<i32>,
	
	/// Linked network interface index.
	///
	/// May be `Some(None)` which means the network interface is linked to `NONE`.
	pub linked_network_interface_index: Option<Option<NetworkInterfaceIndex>>,
	
	/// New net namespace identifier.
	pub new_net_namespace_identifier: Option<i32>,
	
	/// New network interface index.
	pub new_network_interface_index: Option<NetworkInterfaceIndex>,
	
	/// Map.
	/// Also available via the ioctl `SIOCGIFMAP`.
	/// Also settable via the ioctl `SIOCSIFMAP`.
	pub map: rtnl_link_ifmap,
	
	/// Hardware unicast address.
	///
	/// Nearly always an Ethernet MAC address (length of 6).
	///
	/// If `Some` then all `HardwareAddress` will have the same length.
	/// The permanent address is probably the 'burned into ROM' Ethernet Media Access Control address.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/address` and `/sys/class/net/<network_interface_name>/broadcast`.
	/// Also available via the ioctl `SIOCGIFHWADDR`.
	/// Also settable via the ioctls `SIOCSIFHWADDR` and `SIOCSIFHWBROADCAST`.
	/// The length of the two addresses is available at `/sys/class/net/<network_interface_name>/addr_len`.
	/// Formatted in lower-case colon-separated hexadecimal, eg `00:1c:42:9e:fc:bb`.
	///
	/// More information about `Some(hardware_addresses.0)` is avaliable at  `/sys/class/net/<network_interface_name>/addr_assign_type` which is a value of type `NET_ADDR`.
	/// See also `NetworkInterfaceName.assigned_hardware_address_type()`.
	pub address_and_broadcast_and_permanent_address: Option<(HardwareAddress, HardwareAddress, Option<HardwareAddress>)>,
	
	///
	/// Also available at `/sys/class/net/<network_interface_name>/phys_port_id`.
	pub physical_port_identifier: Option<PhysicalIdentifier>,
	
	///
	/// Also available at `/sys/class/net/<network_interface_name>/phys_port_name`.
	pub physical_port_name: Option<CString>,
	
	///
	/// Also available at `/sys/class/net/<network_interface_name>/phys_switch_id`.
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
	pub fn is_for_name(&self, network_interface_name: &NetworkInterfaceName) -> bool
	{
		&self.network_interface_name == network_interface_name
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_for_index(&self, network_interface_index: NetworkInterfaceIndex) -> bool
	{
		self.network_interface_index == network_interface_index
	}
}
