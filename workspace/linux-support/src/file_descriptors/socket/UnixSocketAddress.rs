// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An Unix socket address.
#[derive(Debug)]
pub enum UnixSocketAddress<FilePath: AsRef<Path>>
{
	/// A file in a file system.
	File
	{
		/// An Unix Domain Socket file path of up to 107 bytes.
		socket_file_path: FilePath,

		/// Only used by listeners.
		parent_folder_mode: AccessPermissions,
	},

	/// A Linux-specific abstractly named socket.
	Abstract
	{
		/// An abstract name of zero or more bytes.
		abstract_name: ArrayVec<[u8; sockaddr_un::PathLength - 1]>,
	}
}

impl<FilePath: AsRef<Path>> SocketAddress for UnixSocketAddress<FilePath>
{
	type SD = sockaddr_un;
	
	#[inline(always)]
	fn new_transmission_control_protocol_server_listener(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16, back_log: u32, non_blocking: bool, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<Self::SD>::new_streaming_unix_domain_socket_server_listener(self, send_buffer_size_in_bytes, back_log, non_blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_transmission_control_protocol_client(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16, non_blocking: bool) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		SocketFileDescriptor::<Self::SD>::new_streaming_unix_domain_socket_client(self, send_buffer_size_in_bytes, non_blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_server_listener(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, non_blocking: bool) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<Self::SD>::new_datagram_unix_domain_socket_server_listener(self, send_buffer_size_in_bytes, non_blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_client(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, non_blocking: bool) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		SocketFileDescriptor::<Self::SD>::new_datagram_unix_domain_socket_client(self, send_buffer_size_in_bytes, non_blocking)
	}
}
