// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global network configuration.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalNetworkConfiguration
{
	/// Default is `pfifo_fast`.
	///
	/// However `fq` is better for HTTP/2 prioritization.
	///
	/// Requires root.
	pub queuing_discipline_algorithm: Option<QueuingDisciplineAlgorithm>,
	
	/// Requires root.
	pub receive_packet_steering: Option<GlobalReceivePacketSteeringAndReceiveFlowSteeringConfiguration>,
	
	/// Requires root.
	pub all_network_devices_configuration: Option<GlobalAllNetworkDevicesConfiguration>,
	
	/// Requires root.
	pub network_devices_configuration: HashMap<NetworkInterfaceName, GlobalNetworkDeviceConfiguration>,
	
	/// Requires root.
	pub socket_configuration: Option<GlobalSocketConfiguration>,
	
	/// Global TCP network configuration.
	///
	/// Requires root.
	pub tcp_network_configuration: Option<GlobalTransmissionControlProtocolConfiguration>,
	
	/// Requires root.
	pub unix_domain_socket_maximum_datagram_queue_length: Option<MaximumUnixDomainSocketDatagramQueueLength>,
}

impl GlobalNetworkConfiguration
{
	/// Configures.
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath) -> Result<(), GlobalNetworkConfigurationError>
	{
		use self::GlobalNetworkConfigurationError::*;
		
		instance_set_value(proc_path, QueuingDisciplineAlgorithm::set_global_default, self.queuing_discipline_algorithm, CouldNotChangeGlobalDefaultQueuingDisciplineAlgorithm)?;
		
		if let Some(ref receive_packet_steering) = self.receive_packet_steering
		{
			receive_packet_steering.configure(proc_path)?;
		}
		
		if let Some(ref all_network_devices_configuration) = self.all_network_devices_configuration
		{
			all_network_devices_configuration.configure(proc_path)?;
		}
		
		for (network_interface_name, network_device_configuration) in self.network_devices_configuration.iter()
		{
			network_device_configuration.configure(sys_path, network_interface_name)?;
		}
		
		if let Some(ref socket_configuration) = self.socket_configuration
		{
			socket_configuration.configure(proc_path)?;
		}
		
		if let Some(ref tcp_network_configuration) = self.tcp_network_configuration
		{
			tcp_network_configuration.configure(proc_path)?;
		}
		
		instance_set_value(proc_path, MaximumUnixDomainSocketDatagramQueueLength::set_global_default, self.unix_domain_socket_maximum_datagram_queue_length, CouldNotChangeMaximumUnixDomainSocketDatagramQueueLength)?;
		
		Ok(())
	}
}
