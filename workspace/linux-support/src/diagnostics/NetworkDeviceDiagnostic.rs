// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Network device diagnostic.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkDeviceDiagnostic
{
	/// From netlink.
	#[serde(flatten)] pub link: GetLinkMessageData,

	/// From `ioctl()`.
	#[serde(flatten)] pub input_output_control: DiagnosticUnobtainableResult<NetworkDeviceInputOutputControlDiagnostic>,

	#[allow(missing_docs)]
	pub generic_receive_offload_flush_timeout_in_nanoseconds: DiagnosticUnobtainableResult<u32>,

	/// See detail in `Documentation/ABI/testing/sysfs-class-net` in Linux source.
	pub device_identifier: DiagnosticUnobtainableResult<u16>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net` in Linux source.
	pub device_port: DiagnosticUnobtainableResult<u16>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net` in Linux source.
	pub is_dormant: DiagnosticUnobtainableResult<bool>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net` in Linux source.
	pub is_testing: DiagnosticUnobtainableResult<bool>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net` in Linux source.
	pub assigned_hardware_address_type: DiagnosticUnobtainableResult<NET_ADDR>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net` in Linux source.
	pub assigned_hardware_name: DiagnosticUnobtainableResult<NET_NAME>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub receive_queues: DiagnosticUnobtainableResult<HashMap<QueueIdentifier, NetworkDeviceReceiveQueueDiagnostic>>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub transmit_queues: DiagnosticUnobtainableResult<HashMap<QueueIdentifier, NetworkDeviceTransmitQueueDiagnostic>>,
}

impl NetworkDeviceDiagnostic
{
	#[inline(always)]
	fn gather(sys_path: &SysPath, link: GetLinkMessageData) -> Self
	{
		let network_interface_name = &link.network_interface_name;
		
		Self
		{
			input_output_control: NetworkDeviceInputOutputControlDiagnostic::gather(&link.network_interface_name),
			
			generic_receive_offload_flush_timeout_in_nanoseconds: network_interface_name.generic_receive_offload_flush_timeout_in_nanoseconds(sys_path).map_err(DiagnosticUnobtainable::from),
			
			device_identifier: network_interface_name.device_identifier(sys_path).map_err(DiagnosticUnobtainable::from),
			
			device_port: network_interface_name.device_port(sys_path).map_err(DiagnosticUnobtainable::from),
			
			is_dormant: network_interface_name.is_dormant(sys_path).map_err(DiagnosticUnobtainable::from),
			
			is_testing: network_interface_name.is_testing(sys_path).map_err(DiagnosticUnobtainable::from),
			
			assigned_hardware_address_type: network_interface_name.assigned_hardware_address_type(sys_path).map_err(DiagnosticUnobtainable::from),
			
			assigned_hardware_name: network_interface_name.assigned_hardware_name(sys_path).map_err(DiagnosticUnobtainable::from),
			
			receive_queues: match network_interface_name.receive_queues(sys_path)
			{
				Err(error) => Err(DiagnosticUnobtainable::from(error)),
				
				Ok(receive_sysfs_queues) => Ok(receive_queues.iter(|receive_sysfs_queue| NetworkDeviceReceiveQueueDiagnostic::gather(sys_path, receive_sysfs_queue)).map().collect()),
			},
			
			transmit_queues: match network_interface_name.receive_queues(sys_path)
			{
				Err(error) => Err(DiagnosticUnobtainable::from(error)),
				
				Ok(transmit_sysfs_queues) => Ok(transmit_queues.iter().map(|transmit_sysfs_queue| NetworkDeviceTransmitQueueDiagnostic::gather(sys_path, transmit_sysfs_queue)).collect()),
			},
			
			link,
		}
	}
}
