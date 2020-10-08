// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global receive packet steering (RPS) and Receive Flow Steering (RFS) configuration.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalReceivePacketSteeringAndReceiveFlowSteeringConfiguration
{
	/// Default is 0!
	///
	/// A better value might be 32768.
	///
	/// Rounded up to power of two.
	///
	/// Requires root.
	pub maximum_receive_packet_steering_flows_per_hyper_thread: Option<ReceiveFlowSteeringFlowCount>,
	
	/// Requires root.
	///
	/// Default value is 4096; ?will be rounded up by the Linux kernel to a power-of-two?
	///
	/// Size of the hash table used by Receive Flow Steering (RFS).
	pub flow_limit_table_size: Option<NonZeroU32>,
}

impl GlobalReceivePacketSteeringAndReceiveFlowSteeringConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalReceivePacketSteeringAndReceiveFlowSteeringConfigurationError>
	{
		use self::GlobalReceivePacketSteeringAndReceiveFlowSteeringConfigurationError::*;
		
		instance_set_value(proc_path, ReceiveFlowSteeringFlowCount::set_global_maximum, self.maximum_receive_packet_steering_flows_per_hyper_thread, CouldNotChangeGlobalDefaultReceivePacketSteeringFlowsPerCpu)?;
		
		set_proc_sys_net_core_value(proc_path, "flow_limit_table_len", self.flow_limit_table_size.map(UnpaddedDecimalInteger), CouldNotChangeFlowLimitTableSize)?;
		
		Ok(())
	}
}
