// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global socket network configuration.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalSocketConfiguration
{
	/// Equivalent to socket option `SO_BUSY_POLL`.
	///
	/// If set, the Linux documentation recommends a value of `50` microseconds.
	///
	/// Only exists if the Linux kernel has been configured with `CONFIG_NET_RX_BUSY_POLL`.
	///
	/// Requires root.
	pub socket_busy_read: Option<BusyPollMicroseconds>,
	
	/// Only exists if the Linux kernel has been configured with `CONFIG_NET_RX_BUSY_POLL`.
	///
	/// Requires root.
	pub socket_busy_select_and_poll: Option<BusyPollMicroseconds>,
	
	/// Default is 128.
	///
	/// Requires root.
	pub socket_maximum_back_log: Option<BackLog>,
	
	/// Default is 4_294_967_295.
	///
	/// Set this to 16KB for HTTP/2 prioritization to work reliably.
	///
	/// Requires root.
	pub socket_not_sent_low_water_in_bytes: Option<NotSentLowWaterInBytes>,
	
	/// By default, Linux sets auto corking to on (true).
	///
	/// Requires root.
	pub socket_adjust_auto_corking: Option<bool>,
	
	/// Global maximum send buffer size.
	///
	/// Requires root.
	pub global_maximum_send_buffer_size_in_bytes: Option<SendBufferSizeInBytes>,
	
	/// Global default send buffer size.
	///
	/// Requires root.
	pub global_default_send_buffer_size_in_bytes: Option<SendBufferSizeInBytes>,
	
	/// Global maximum receive buffer size.
	///
	/// Requires root.
	pub global_maximum_receive_buffer_size_in_bytes: Option<ReceiveBufferSizeInBytes>,
	
	/// Global default receive buffer size.
	///
	/// Requires root.
	pub global_default_receive_buffer_size_in_bytes: Option<ReceiveBufferSizeInBytes>,
	
	/// Global maximum control (ancillary) message buffer size.
	///
	/// Requires root.
	pub global_maximum_control_message_buffer_size_in_bytes: Option<ControlMessageBufferSizeInBytes>,
}

impl GlobalSocketConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalSocketConfigurationError>
	{
		use self::GlobalSocketConfigurationError::*;
		
		instance_set_value(proc_path, BusyPollMicroseconds::set_global_default, self.socket_busy_read, CouldNotChangeGlobalDefaultSocketBusyRead)?;
		instance_set_value(proc_path, BusyPollMicroseconds::set_global_select_and_poll_default, self.socket_busy_select_and_poll, CouldNotChangeGlobalDefaultSocketBusySelectAndPoll)?;
		instance_set_value(proc_path, BackLog::set_global_maximum, self.socket_maximum_back_log, CouldNotChangeGlobalMaximumBackLog)?;
		instance_set_value(proc_path, NotSentLowWaterInBytes::set_global_default, self.socket_not_sent_low_water_in_bytes, CouldNotChangeGlobalDefaultNotSentLowWater)?;
		
		instance_set_value(proc_path, SendBufferSizeInBytes::set_global_maximum, self.global_maximum_send_buffer_size_in_bytes, CouldNotChangeGlobalMaximumSendBufferSize)?;
		instance_set_value(proc_path, SendBufferSizeInBytes::set_global_default, self.global_default_send_buffer_size_in_bytes, CouldNotChangeGlobalDefaultSendBufferSize)?;
		
		instance_set_value(proc_path, ReceiveBufferSizeInBytes::set_global_maximum, self.global_maximum_receive_buffer_size_in_bytes, CouldNotChangeGlobalMaximumReceiveBufferSize)?;
		instance_set_value(proc_path, ReceiveBufferSizeInBytes::set_global_default, self.global_default_receive_buffer_size_in_bytes, CouldNotChangeGlobalDefaultReceiveBufferSize)?;
		
		instance_set_value(proc_path, ControlMessageBufferSizeInBytes::set_global_maximum, self.global_maximum_control_message_buffer_size_in_bytes, CouldNotChangeGlobalMaximumControlMessageBufferSize)?;
		
		set_value(proc_path, set_auto_corking, self.socket_adjust_auto_corking, CouldNotChangeAutoCorking)?;
		
		if let Some(ref tcp_configuration) = self.tcp_configuration
		{
			tcp_configuration.configure(proc_path)?;
		}
		
		Ok(())
	}
}
