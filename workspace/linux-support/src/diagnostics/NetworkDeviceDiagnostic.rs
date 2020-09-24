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
}

/*
TODO: Diagnostic for ethX queues.
TODO: Setting values for ethX queues.
/sys/class/net/eth0

    queues/
        tx-0/
            traffic_class
                couldn't be read
            tx_maxrate
                0
                read-write
            tx_timeout
                0 (without line feed)
            xps_cpus
                couldn't be read
                read-write
            xps_rxqs
                0
                read-write
            byte_queue_limits/
                hold_time
                    1000
                    read-write
                inflight
                    0
                    read-only
                limit
                    0
                    read-write
                limit_max
                    1879048192
                    read-write
                limit_min
                    0
                    read-write
    power/
        autosuspend_delay_ms
            couldn't be read
            read-write
        control
            "auto"
            read-write
        runtime_active_time
        runtime_status
            "unsupported"
        runtime_suspended_time
 */


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
			
			link,
		}
	}
}
