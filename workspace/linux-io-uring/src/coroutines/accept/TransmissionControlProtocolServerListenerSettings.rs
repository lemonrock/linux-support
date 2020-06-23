// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransmissionControlProtocolServerListenerSettings<SA: SocketAddress>
{
	pub socket_address: SA,
	#[serde(default = "TransmissionControlProtocolServerListenerSettings::<SA>::default_send_buffer_size_in_bytes")] pub send_buffer_size_in_bytes: SendBufferSizeInBytes,
	#[serde(default = "TransmissionControlProtocolServerListenerSettings::<SA>::default_receive_buffer_size_in_bytes")] pub receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes,
	#[serde(default)] pub idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds,
	#[serde(default)] pub keep_alive_interval_seconds: KeepAliveIntervalSeconds,
	#[serde(default)] pub maximum_keep_alive_probes: MaximumKeepAliveProbes,
	#[serde(default)] pub socket_linger_seconds: SocketLingerSeconds,
	#[serde(default)] pub finish_timeout_seconds: FinishTimeoutSeconds,
	#[serde(default)] pub maximum_syn_retransmits: MaximumSynRetransmits,
	#[serde(default)] pub not_sent_low_water_in_bytes: NotSentLowWaterInBytes,
	#[serde(default)] pub back_log: BackLog,
}

impl<SA: SocketAddress> TransmissionControlProtocolServerListenerSettings<SA>
{
	/// Defaulti-ish
	#[inline(always)]
	pub fn defaultish(socket_address: SA) -> Self
	{
		Self
		{
			socket_address,
			send_buffer_size_in_bytes: Self::default_send_buffer_size_in_bytes(),
			receive_buffer_size_in_bytes: Self::default_receive_buffer_size_in_bytes(),
			idles_before_keep_alive_seconds: Default::default(),
			keep_alive_interval_seconds: Default::default(),
			maximum_keep_alive_probes: Default::default(),
			socket_linger_seconds: Default::default(),
			finish_timeout_seconds: Default::default(),
			maximum_syn_retransmits: Default::default(),
			not_sent_low_water_in_bytes: Default::default(),
			back_log: Default::default()
		}
	}
	
	/// Must be run on the thread the socket is to be created on.
	///
	// This logic NEEDS TO happen before the coroutine starts.
	// This allows us to drop capabilities on the thread for binding to ports below 1024.
	#[inline(always)]
	pub(crate) fn new_socket(self, our_hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<SA::SD>, NewSocketServerListenerError>
	{
		self.socket_address.new_transmission_control_protocol_server_listener
		(
			self.send_buffer_size_in_bytes,
			self.receive_buffer_size_in_bytes,
			self.idles_before_keep_alive_seconds,
			self.keep_alive_interval_seconds,
			self.maximum_keep_alive_probes,
			self.socket_linger_seconds,
			self.finish_timeout_seconds,
			self.maximum_syn_retransmits,
			self.not_sent_low_water_in_bytes,
			self.back_log,
			false,
			our_hyper_thread,
		)
	}
	
	#[inline(always)]
	fn default_send_buffer_size_in_bytes() -> SendBufferSizeInBytes
	{
		SA::DefaultSendBufferSizeInBytes
	}
	
	#[inline(always)]
	fn default_receive_buffer_size_in_bytes() -> ReceiveBufferSizeInBytes
	{
		SA::DefaultReceiveBufferSizeInBytes
	}
}
