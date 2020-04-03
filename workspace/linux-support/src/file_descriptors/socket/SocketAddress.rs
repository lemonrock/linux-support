// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a socket address that can be used as a server listener or as a client, for streams and data grams.
pub enum SocketAddress<FilePath: AsRef<Path>>
{
	/// An Internet Protocol (IP) version 4 or 6 socket.
	InternetProtocol(SocketAddr),

	/// An Unix Domain Socket, either as a file or as an abstract name.
	Unix(UnixSocketAddress<FilePath>),
}

impl<FilePath: AsRef<Path>> SocketAddress<FilePath>
{
	/// New streaming server listener.
	///
	/// `back_log` can not exceed `std::i32::MAX` and is capped by the Operating System to the value in `/proc/sys/net/core/somaxconn`.
	///
	/// The default value in `/proc/sys/net/core/somaxconn` is `128`.
	///
	/// `logical_core_identifier` for the CPU the current thread is executing on can be obtained by using `unsafe { ::libc::sched_getcpu() }` on Linux.
	#[inline(always)]
	pub fn new_streaming_server_listener(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16, back_log: u32, logical_core_identifier: u16) -> Result<StreamingServerListenerSocketFileDescriptorEnum, NewSocketServerListenerError>
	{
		use self::StreamingServerListenerSocketFileDescriptorEnum::*;
		use self::SocketAddr::*;
		use self::SocketAddress::*;

		Ok
		(
			match self
			{
				&InternetProtocol(V4(socket_address)) => InternetProtocolVersion4(SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_4_server_listener(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits, back_log, logical_core_identifier)?),
				&InternetProtocol(V6(socket_address)) => InternetProtocolVersion6(SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_6_server_listener(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits, back_log, logical_core_identifier)?),
				&Unix(ref unix_socket_address) => UnixDomain(SocketFileDescriptor::new_streaming_unix_domain_socket_server_listener(unix_socket_address, send_buffer_size_in_bytes, back_log, logical_core_identifier)?),
			}
		)
	}

	/// New streaming client.
	#[inline(always)]
	pub fn new_streaming_client(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16) -> Result<StreamingSocketFileDescriptorEnum, NewSocketClientError>
	{
		use self::StreamingSocketFileDescriptorEnum::*;
		use self::SocketAddr::*;
		use self::SocketAddress::*;

		Ok
		(
			match self
			{
				&InternetProtocol(V4(socket_address)) => InternetProtocolVersion4(SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_4_client(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits)?),
				&InternetProtocol(V6(socket_address)) => InternetProtocolVersion6(SocketFileDescriptor::new_transmission_control_protocol_over_internet_protocol_version_6_client(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits)?),
				&Unix(ref unix_socket_address) => UnixDomain(SocketFileDescriptor::new_streaming_unix_domain_socket_client(unix_socket_address, send_buffer_size_in_bytes)?),
			}
		)
	}

	/// New datagram server listener.
	#[inline(always)]
	pub fn new_datagram_server_listener(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize) -> Result<DatagramServerListenerSocketFileDescriptorEnum, NewSocketServerListenerError>
	{
		use self::DatagramServerListenerSocketFileDescriptorEnum::*;
		use self::SocketAddr::*;
		use self::SocketAddress::*;

		Ok
		(
			match self
			{
				&InternetProtocol(V4(socket_address)) => InternetProtocolVersion4(SocketFileDescriptor::new_user_datagram_protocol_over_internet_protocol_version_4_server_listener(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes)?),
				&InternetProtocol(V6(socket_address)) => InternetProtocolVersion6(SocketFileDescriptor::new_user_datagram_protocol_over_internet_protocol_version_6_server_listener(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes)?),
				&Unix(ref unix_socket_address) => UnixDomain(SocketFileDescriptor::new_datagram_unix_domain_socket_server_listener(unix_socket_address, send_buffer_size_in_bytes)?),
			}
		)
	}

	/// New datagram client.
	#[inline(always)]
	pub fn new_datagram_client(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize) -> Result<DatagramClientSocketFileDescriptorEnum, NewSocketClientError>
	{
		use self::DatagramClientSocketFileDescriptorEnum::*;
		use self::SocketAddr::*;
		use self::SocketAddress::*;

		Ok
		(
			match self
			{
				&InternetProtocol(V4(socket_address)) => InternetProtocolVersion4(SocketFileDescriptor::new_user_datagram_protocol_over_internet_protocol_version_4_client(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes)?),
				&InternetProtocol(V6(socket_address)) => InternetProtocolVersion6(SocketFileDescriptor::new_user_datagram_protocol_over_internet_protocol_version_6_client(socket_address, send_buffer_size_in_bytes, receive_buffer_size_in_bytes)?),
				&Unix(ref unix_socket_address) => UnixDomain(SocketFileDescriptor::new_datagram_unix_domain_socket_client(unix_socket_address, send_buffer_size_in_bytes)?),
			}
		)
	}

	/// Creates a new streaming Unix Domain client socket pair.
	///
	/// This is local socket akin to a Transmission Control Protocol (TCP) socket.
	#[inline(always)]
	pub fn new_streaming_unix_domain_socket_pair(lefthand_send_buffer_size_in_bytes: usize, righthand_send_buffer_size_in_bytes: usize) -> Result<(StreamingSocketFileDescriptor<sockaddr_un>, StreamingSocketFileDescriptor<sockaddr_un>), NewSocketClientError>
	{
		SocketFileDescriptor::new_streaming_unix_domain_socket_pair(lefthand_send_buffer_size_in_bytes, righthand_send_buffer_size_in_bytes)
	}

	/// Creates a new datagram Unix Domain client socket pair.
	///
	/// This is local socket akin to an User Datagram Protocol (UDP) socket.
	#[inline(always)]
	pub fn new_datagram_unix_domain_socket_pair(lefthand_send_buffer_size_in_bytes: usize, righthand_send_buffer_size_in_bytes: usize) -> Result<(DatagramClientSocketFileDescriptor<sockaddr_un>, DatagramClientSocketFileDescriptor<sockaddr_un>), NewSocketClientError>
	{
		SocketFileDescriptor::new_datagram_unix_domain_socket_pair(lefthand_send_buffer_size_in_bytes, righthand_send_buffer_size_in_bytes)
	}
}
