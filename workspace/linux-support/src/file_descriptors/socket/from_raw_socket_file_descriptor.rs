// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn from_raw_socket_file_descriptor<R>(socket_file_descriptor: RawFd, internet_protocol_version_4: impl FnOnce(SocketFileDescriptor<sockaddr_in>) -> R, internet_protocol_version_6: impl FnOnce(SocketFileDescriptor<sockaddr_in6>) -> R, unix_domain_socket: impl FnOnce(SocketFileDescriptor<sockaddr_un>) -> R) -> R
{
	let domain = SocketFileDescriptor::<sockaddr_in>(socket_file_descriptor, PhantomData).get_socket_option(SOL_SOCKET, SO_DOMAIN);
	match domain
	{
		AF_INET => internet_protocol_version_4(SocketFileDescriptor::<sockaddr_in>(socket_file_descriptor, PhantomData)),
		AF_INET6 => internet_protocol_version_6(SocketFileDescriptor::<sockaddr_in6>(socket_file_descriptor, PhantomData)),
		AF_UNIX => unix_domain_socket(SocketFileDescriptor::<sockaddr_un>(socket_file_descriptor, PhantomData)),

		_ => panic!("Not an AF_INET, AF_INET6 or AF_UNIX socket but {}", domain)
	}
}
