// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An Unix socket address.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
	fn new_transmission_control_protocol_server_listener(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, _transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, back_log: BackLog, blocking: &Blocking, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<Self::SD>::new_streaming_unix_domain_socket_server_listener(self, internet_protocol_socket_settings.send_buffer_size, back_log, blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_transmission_control_protocol_client(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, _transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, _writes_before_reading: bool, blocking: &Blocking) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		SocketFileDescriptor::<Self::SD>::new_streaming_unix_domain_socket_client(self, internet_protocol_socket_settings.send_buffer_size, blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_server_listener(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<Self::SD>::new_datagram_unix_domain_socket_server_listener(self, internet_protocol_socket_settings.send_buffer_size, blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_client(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		SocketFileDescriptor::<Self::SD>::new_datagram_unix_domain_socket_client(self, internet_protocol_socket_settings.send_buffer_size, blocking)
	}
}

impl<FilePath: AsRef<Path>> UnixSocketAddress<FilePath>
{
	/// From an abstract name.
	#[inline(always)]
	pub fn from_abstract_name(name: &[u8]) -> Result<Self, CapacityError>
	{
		let mut abstract_name = ArrayVec::new();
		abstract_name.try_extend_from_slice(name)?;
		Ok
		(
			UnixSocketAddress::Abstract
			{
				abstract_name
			}
		)
	}
}
