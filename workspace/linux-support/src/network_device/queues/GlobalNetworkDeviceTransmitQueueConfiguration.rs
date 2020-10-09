// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global network device transmit queue configuration
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalNetworkDeviceTransmitQueueConfiguration
{
	/// Common.
	#[serde(default)] pub common: CommonTransmitQueueConfiguration,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	///
	/// Either set Transmit Packet Steering (XPS) affinity to HyperThreads or Receive QueueIdentifiers.
	#[serde(default)] pub transmit_packet_steering: Option<Either<HyperThreads, QueueIdentifiers>>,
}

impl GlobalNetworkDeviceTransmitQueueConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, transmit_sysfs_queue: &TransmitSysfsQueue) -> Result<(), GlobalNetworkDeviceConfigurationError>
	{
		use self::GlobalNetworkDeviceConfigurationError::*;
		
		self.common.configure(sys_path, transmit_sysfs_queue)?;
		
		if let Some(ref transmit_packet_steering) = self.transmit_packet_steering
		{
			match transmit_packet_steering
			{
				&Left(ref hyper_threads) =>
				{
					transmit_sysfs_queue.set_transmit_packet_steering_hyper_thread_affinity(sys_path, hyper_threads).map_err(CouldNotSetPerTransmitQueueTransmitPacketSteeringHyperThreadAffinity)?;
				},
				
				&Right(ref receive_queues_to_map_to) => transmit_sysfs_queue.set_transmit_packet_steering_receive_queue_affinity(sys_path, receive_queues_to_map_to).map_err(CouldNotSetPerTransmitQueueTransmitPacketSteeringReceiveQueueAffinity)?,
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn linux_default_with_one_to_one_receive_to_transmit_packet_steering(receive_queue_identifier: QueueIdentifier) -> Self
	{
		Self
		{
			common: CommonTransmitQueueConfiguration::linux_default(),
			
			transmit_packet_steering: Some(Right(QueueIdentifiers::for_one(receive_queue_identifier))),
		}
	}
}
