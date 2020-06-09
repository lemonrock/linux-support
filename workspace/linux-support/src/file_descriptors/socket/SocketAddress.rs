// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Can also be `SocketData` but not necessarily.
pub trait SocketAddress
{
	/// Socket data associated with this address.
	type SD: SocketData;
	
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket server listener.
	///
	/// `back_log` can not exceed `i32::MAX` and is capped by the Operating System to the value in `/proc/sys/net/core/somaxconn`.
	///
	/// The default value in `/proc/sys/net/core/somaxconn` is `128`.
	fn new_transmission_control_protocol_server_listener(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16, back_log: u32, non_blocking: bool, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>;
	
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket client.
	fn new_transmission_control_protocol_client(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16, non_blocking: bool) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>;
	
	/// Creates a new instance of a User Datagram Protocol (UDP) socket server listener.
	fn new_user_datagram_protocol_server_listener(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, non_blocking: bool) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>;
	
	/// Creates a new instance of a User Datagram Protocol (UDP) socket client.
	fn new_user_datagram_protocol_client(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, non_blocking: bool) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>;
}
