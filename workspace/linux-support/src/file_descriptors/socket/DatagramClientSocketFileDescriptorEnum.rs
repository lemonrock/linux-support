// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a datagram client socket instance between two peers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatagramClientSocketFileDescriptorEnum
{
	/// An Internet Protocol (IP) version 4 streaming socket.
	InternetProtocolVersion4(DatagramClientSocketInternetProtocolVersion4FileDescriptor),

	/// An Internet Protocol (IP) version 6 streaming socket.
	InternetProtocolVersion6(DatagramClientSocketInternetProtocolVersion6FileDescriptor),

	/// An Unix Domain streaming socket.
	UnixDomain(DatagramClientSocketUnixDomainFileDescriptor),
}

impl AsRawFd for DatagramClientSocketFileDescriptorEnum
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		use self::DatagramClientSocketFileDescriptorEnum::*;

		match self
		{
			&InternetProtocolVersion4(ref datagram_client_socket_file_descriptor) => datagram_client_socket_file_descriptor.as_raw_fd(),
			&InternetProtocolVersion6(ref datagram_client_socket_file_descriptor) => datagram_client_socket_file_descriptor.as_raw_fd(),
			&UnixDomain(ref datagram_client_socket_file_descriptor) => datagram_client_socket_file_descriptor.as_raw_fd(),
		}
	}
}

impl AsRawFdExt for DatagramClientSocketFileDescriptorEnum
{
}
