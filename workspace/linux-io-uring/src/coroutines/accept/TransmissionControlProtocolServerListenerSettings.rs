// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct TransmissionControlProtocolServerListenerSettings<SA: SocketAddress>
{
	pub socket_address: SA,
	pub send_buffer_size_in_bytes: SendBufferSizeInBytes,
	pub receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes,
	pub idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds,
	pub keep_alive_interval_seconds: KeepAliveIntervalSeconds,
	pub maximum_keep_alive_probes: MaximumKeepAliveProbes,
	pub socket_linger_seconds: SocketLingerSeconds,
	pub finish_timeout_seconds: FinishTimeoutSeconds,
	pub maximum_syn_retransmits: MaximumSynRetransmits,
	pub back_log: BackLog,
}

impl<SA: SocketAddress> TransmissionControlProtocolServerListenerSettings<SA>
{
	/// Must be run on the thread the socket is to be created on.
	///
	// This logic NEEDS TO happen before the coroutine starts.
	// This allows us to drop capabilities on the thread for binding to ports below 1024.
	#[inline(always)]
	pub(crate) fn new_socket(self) -> Result<StreamingServerListenerSocketFileDescriptor<SA::SD>, NewSocketServerListenerError>
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
			self.back_log,
			false,
			HyperThread::current_hyper_thread(),
		)
	}
}
