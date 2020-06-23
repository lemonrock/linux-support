// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Can also be `SocketData` but not necessarily.
pub trait SocketAddress
{
	/// Socket data associated with this address.
	type SD: 'static + SocketData;
	
	/// Default.
	const DefaultReceiveBufferSizeInBytes: ReceiveBufferSizeInBytes = ReceiveBufferSizeInBytes::UsualGlobalDefaultForTcp;
	
	/// Default.
	const DefaultSendBufferSizeInBytes: SendBufferSizeInBytes = SendBufferSizeInBytes::UsualGlobalDefaultForTcp;
	
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket server listener.
	fn new_transmission_control_protocol_server_listener(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds, keep_alive_interval_seconds: KeepAliveIntervalSeconds, maximum_keep_alive_probes: MaximumKeepAliveProbes, socket_linger_seconds: SocketLingerSeconds, finish_timeout_seconds: FinishTimeoutSeconds, maximum_syn_retransmits: MaximumSynRetransmits, not_sent_low_water_in_bytes: NotSentLowWaterInBytes, back_log: BackLog, non_blocking: bool, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>;
	
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket client.
	///
	/// `writes_before_reading` is only appropriate for client sockets that send the first packet (ie write() before they read()).
	fn new_transmission_control_protocol_client(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds, keep_alive_interval_seconds: KeepAliveIntervalSeconds, maximum_keep_alive_probes: MaximumKeepAliveProbes, socket_linger_seconds: SocketLingerSeconds, finish_timeout_seconds: FinishTimeoutSeconds, maximum_syn_retransmits: MaximumSynRetransmits, writes_before_reading: bool, not_sent_low_water_in_bytes: NotSentLowWaterInBytes, non_blocking: bool) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>;
	
	/// Creates a new instance of a User Datagram Protocol (UDP) socket server listener.
	fn new_user_datagram_protocol_server_listener(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, non_blocking: bool) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>;
	
	/// Creates a new instance of a User Datagram Protocol (UDP) socket client.
	fn new_user_datagram_protocol_client(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, non_blocking: bool) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>;
}

impl SocketAddress for SocketAddrV4
{
	type SD = sockaddr_in;
	
	#[inline(always)]
	fn new_transmission_control_protocol_server_listener(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds, keep_alive_interval_seconds: KeepAliveIntervalSeconds, maximum_keep_alive_probes: MaximumKeepAliveProbes, socket_linger_seconds: SocketLingerSeconds, finish_timeout_seconds: FinishTimeoutSeconds, maximum_syn_retransmits: MaximumSynRetransmits, not_sent_low_water_in_bytes: NotSentLowWaterInBytes, back_log: BackLog, non_blocking: bool, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.new_transmission_control_protocol_server_listener(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, socket_linger_seconds, finish_timeout_seconds, maximum_syn_retransmits, not_sent_low_water_in_bytes, back_log, non_blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_transmission_control_protocol_client(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds, keep_alive_interval_seconds: KeepAliveIntervalSeconds, maximum_keep_alive_probes: MaximumKeepAliveProbes, socket_linger_seconds: SocketLingerSeconds, finish_timeout_seconds: FinishTimeoutSeconds, maximum_syn_retransmits: MaximumSynRetransmits, writes_before_reading: bool, not_sent_low_water_in_bytes: NotSentLowWaterInBytes, non_blocking: bool) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.new_transmission_control_protocol_client(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, socket_linger_seconds, finish_timeout_seconds, maximum_syn_retransmits, writes_before_reading, not_sent_low_water_in_bytes, non_blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_server_listener(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, non_blocking: bool) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.new_user_datagram_protocol_server_listener(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, non_blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_client(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, non_blocking: bool) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.new_user_datagram_protocol_client(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, non_blocking)
	}
}

impl SocketAddress for SocketAddrV6
{
	type SD = sockaddr_in6;
	
	#[inline(always)]
	fn new_transmission_control_protocol_server_listener(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds, keep_alive_interval_seconds: KeepAliveIntervalSeconds, maximum_keep_alive_probes: MaximumKeepAliveProbes, socket_linger_seconds: SocketLingerSeconds, finish_timeout_seconds: FinishTimeoutSeconds, maximum_syn_retransmits: MaximumSynRetransmits, not_sent_low_water_in_bytes: NotSentLowWaterInBytes, back_log: BackLog, non_blocking: bool, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.new_transmission_control_protocol_server_listener(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, socket_linger_seconds, finish_timeout_seconds, maximum_syn_retransmits, not_sent_low_water_in_bytes, back_log, non_blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_transmission_control_protocol_client(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds, keep_alive_interval_seconds: KeepAliveIntervalSeconds, maximum_keep_alive_probes: MaximumKeepAliveProbes, socket_linger_seconds: SocketLingerSeconds, finish_timeout_seconds: FinishTimeoutSeconds, maximum_syn_retransmits: MaximumSynRetransmits, writes_before_reading: bool, not_sent_low_water_in_bytes: NotSentLowWaterInBytes, non_blocking: bool) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.new_transmission_control_protocol_client(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, socket_linger_seconds, finish_timeout_seconds, maximum_syn_retransmits, writes_before_reading, not_sent_low_water_in_bytes, non_blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_server_listener(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, non_blocking: bool) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.new_user_datagram_protocol_server_listener(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, non_blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_client(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, non_blocking: bool) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.new_user_datagram_protocol_client(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, non_blocking)
	}
}
