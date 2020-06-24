// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Can also be `SocketData` but not necessarily.
pub trait SocketAddress
{
	/// Socket data associated with this address.
	type SD: 'static + SocketData;
	
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket server listener.
	fn new_transmission_control_protocol_server_listener(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, back_log: BackLog, blocking: &Blocking, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>;
	
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket client.
	///
	/// `writes_before_reading` is only appropriate for client sockets that send the first packet (ie write() before they read()).
	fn new_transmission_control_protocol_client(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, writes_before_reading: bool, blocking: &Blocking) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>;
	
	/// Creates a new instance of a User Datagram Protocol (UDP) socket server listener.
	fn new_user_datagram_protocol_server_listener(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking, hyper_thread: HyperThread) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>;
	
	/// Creates a new instance of a User Datagram Protocol (UDP) socket client.
	fn new_user_datagram_protocol_client(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>;
}

impl SocketAddress for SocketAddrV4
{
	type SD = sockaddr_in;
	
	#[inline(always)]
	fn new_transmission_control_protocol_server_listener(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, back_log: BackLog, blocking: &Blocking, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.new_transmission_control_protocol_server_listener(internet_protocol_socket_settings, transmission_control_protocol_socket_settings, back_log, blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_transmission_control_protocol_client(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, writes_before_reading: bool, blocking: &Blocking) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.new_transmission_control_protocol_client(internet_protocol_socket_settings, transmission_control_protocol_socket_settings, writes_before_reading, blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_server_listener(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking, hyper_thread: HyperThread) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.new_user_datagram_protocol_server_listener(internet_protocol_socket_settings, blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_client(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.new_user_datagram_protocol_client(internet_protocol_socket_settings, blocking)
	}
}

impl SocketAddress for SocketAddrV6
{
	type SD = sockaddr_in6;
	
	#[inline(always)]
	fn new_transmission_control_protocol_server_listener(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, back_log: BackLog, blocking: &Blocking, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.new_transmission_control_protocol_server_listener(internet_protocol_socket_settings, transmission_control_protocol_socket_settings, back_log, blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_transmission_control_protocol_client(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, writes_before_reading: bool, blocking: &Blocking) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.new_transmission_control_protocol_client(internet_protocol_socket_settings, transmission_control_protocol_socket_settings, writes_before_reading, blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_server_listener(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking, hyper_thread: HyperThread) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.new_user_datagram_protocol_server_listener(internet_protocol_socket_settings, blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_client(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.new_user_datagram_protocol_client(internet_protocol_socket_settings, blocking)
	}
}
