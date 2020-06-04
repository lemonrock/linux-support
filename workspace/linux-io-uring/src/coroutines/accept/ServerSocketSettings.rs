// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
struct ServerSocketSettings<SA: SocketAddress>
{
	pub socket_address: SA,
	pub send_buffer_size_in_bytes: usize,
	pub receive_buffer_size_in_bytes: usize,
	pub idles_before_keep_alive_seconds: u16,
	pub keep_alive_interval_seconds: u16,
	pub maximum_keep_alive_probes: u16,
	pub linger_seconds: u16,
	pub linger_in_FIN_WAIT2_seconds: u16,
	pub maximum_SYN_transmits: u16,
	pub back_log: u32,
}

impl<SA: SocketAddress> ServerSocketSettings<SA>
{
	/// Must be run on the thread the socket is to be created on.
	///
	// TODO: This logic NEEDS TO happen before the coroutine starts.
	// This allows us to drop capabilities on the thread for binding to ports below 1024.
	#[inline(always)]
	fn new_socket(self) -> Result<StreamingServerListenerSocketFileDescriptor<SA::SD>, NewSocketServerListenerError>
	{
		self.socket_address.new_transmission_control_protocol_server_listener
		(
			self.send_buffer_size_in_bytes,
			self.receive_buffer_size_in_bytes,
			self.idles_before_keep_alive_seconds,
			self.keep_alive_interval_seconds,
			self.maximum_keep_alive_probes,
			self.linger_seconds,
			self.linger_in_FIN_WAIT2_seconds,
			self.maximum_SYN_transmits,
			self.back_log,
			false,
			HyperThread::current_hyper_thread(),
		)
	}
}
