// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An Unix socket address.
#[derive(Debug)]
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
	
	const DefaultReceiveBufferSizeInBytes: ReceiveBufferSizeInBytes = ReceiveBufferSizeInBytes::UsualGlobalDefault;
	
	const DefaultSendBufferSizeInBytes: SendBufferSizeInBytes = SendBufferSizeInBytes::UsualGlobalDefault;
	
	#[inline(always)]
	fn new_transmission_control_protocol_server_listener(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, _receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, _idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds, _keep_alive_interval_seconds: KeepAliveIntervalSeconds, _maximum_keep_alive_probes: MaximumKeepAliveProbes, _socket_linger_seconds: SocketLingerSeconds, _finish_timeout_seconds: FinishTimeoutSeconds, _maximum_syn_retransmits: MaximumSynRetransmits, _not_sent_low_water_in_bytes: NotSentLowWaterInBytes, back_log: BackLog, non_blocking: bool, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<Self::SD>::new_streaming_unix_domain_socket_server_listener(self, send_buffer_size_in_bytes, back_log, non_blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_transmission_control_protocol_client(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, _receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, _idles_before_keep_alive_seconds: IdlesBeforeKeepAliveSeconds, _keep_alive_interval_seconds: KeepAliveIntervalSeconds, _maximum_keep_alive_probes: MaximumKeepAliveProbes, _socket_linger_seconds: SocketLingerSeconds, _finish_timeout_seconds: FinishTimeoutSeconds, _maximum_syn_retransmits: MaximumSynRetransmits, _writes_before_reading: bool, _not_sent_low_water_in_bytes: NotSentLowWaterInBytes, non_blocking: bool) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		SocketFileDescriptor::<Self::SD>::new_streaming_unix_domain_socket_client(self, send_buffer_size_in_bytes, non_blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_server_listener(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, _receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, non_blocking: bool) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<Self::SD>::new_datagram_unix_domain_socket_server_listener(self, send_buffer_size_in_bytes, non_blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_client(&self, send_buffer_size_in_bytes: SendBufferSizeInBytes, _receive_buffer_size_in_bytes: ReceiveBufferSizeInBytes, non_blocking: bool) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		SocketFileDescriptor::<Self::SD>::new_datagram_unix_domain_socket_client(self, send_buffer_size_in_bytes, non_blocking)
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
