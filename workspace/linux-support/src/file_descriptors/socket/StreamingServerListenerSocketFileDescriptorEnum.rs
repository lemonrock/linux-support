// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// One of three possible server listener socket file descriptors.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamingServerListenerSocketFileDescriptorEnum
{
	/// An Internet Protocol (IP) version 4 listening socket.
	InternetProtocolVersion4(StreamingServerListenerSocketInternetProtocolVersion4FileDescriptor),

	/// An Internet Protocol (IP) version 6 listening socket.
	InternetProtocolVersion6(StreamingServerListenerSocketInternetProtocolVersion6FileDescriptor),

	/// An Unix Domain listening socket.
	UnixDomain(StreamingServerListenerSocketUnixDomainFileDescriptor),
}

impl AsRawFd for StreamingServerListenerSocketFileDescriptorEnum
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		use self::StreamingServerListenerSocketFileDescriptorEnum::*;

		match self
		{
			&InternetProtocolVersion4(ref streaming_server_listener_socket_file_descriptor) => streaming_server_listener_socket_file_descriptor.as_raw_fd(),
			&InternetProtocolVersion6(ref streaming_server_listener_socket_file_descriptor) => streaming_server_listener_socket_file_descriptor.as_raw_fd(),
			&UnixDomain(ref streaming_server_listener_socket_file_descriptor) => streaming_server_listener_socket_file_descriptor.as_raw_fd(),
		}
	}
}

impl AsRawFdExt for StreamingServerListenerSocketFileDescriptorEnum
{
}

impl IntoRawFd for StreamingServerListenerSocketFileDescriptorEnum
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		use self::StreamingServerListenerSocketFileDescriptorEnum::*;

		match self
		{
			InternetProtocolVersion4(streaming_server_listener_socket_file_descriptor) => streaming_server_listener_socket_file_descriptor.into_raw_fd(),
			InternetProtocolVersion6(streaming_server_listener_socket_file_descriptor) => streaming_server_listener_socket_file_descriptor.into_raw_fd(),
			UnixDomain(streaming_server_listener_socket_file_descriptor) => streaming_server_listener_socket_file_descriptor.into_raw_fd(),
		}
	}
}

impl StreamingServerListenerSocketFileDescriptorEnum
{
	/// Accepts a pending connections.
	#[inline(always)]
	pub fn accept(&self) -> Result<AcceptedConnectionEnum, SocketAcceptError>
	{
		use self::StreamingServerListenerSocketFileDescriptorEnum::*;

		Ok
		(
			match self
			{
				InternetProtocolVersion4(streaming_server_listener_socket_file_descriptor) => AcceptedConnectionEnum::InternetProtocolVersion4(streaming_server_listener_socket_file_descriptor.accept()?),
				InternetProtocolVersion6(streaming_server_listener_socket_file_descriptor) => AcceptedConnectionEnum::InternetProtocolVersion6(streaming_server_listener_socket_file_descriptor.accept()?),
				UnixDomain(streaming_server_listener_socket_file_descriptor) => AcceptedConnectionEnum::UnixDomain(streaming_server_listener_socket_file_descriptor.accept()?),
			}
		)
	}
}
