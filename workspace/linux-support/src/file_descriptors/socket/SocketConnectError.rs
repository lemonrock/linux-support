// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during binding of a socket instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SocketConnectError
{
	/// Permission denied.
	///
	/// For UNIX domain sockets, which are identified by pathname: Write permission is denied on the socket file, or search permission is denied for one of the directories in the path prefix.
	/// For others: The user tried to connect to a broadcast address without having the socket broadcast flag enabled or the connection request failed because of a local firewall rule.
	PermissionDenied,

	/// The local address is already in use.
	AddressInUse,

	/// No more free local ports or insufficient entries in the routing cache (eg try adjusting `/proc/sys/net/ipv4/ip_local_port_range` for `AF_INET`).
	NoMoreFreeLocalPorts,

	/// No-one listening on the remote address.
	ConnectionRefused,

	/// Network is unreachable.
	NetworkUnreachable,

	/// Timed out whilst attempting connection.
	TimedOut,

	/// The socket is nonblocking and the connection cannot be completed immediately. After `epoll_wait()` indicates the socket is writable use `getsockopt()` with `SOL_SOCKET` and `SO_ERROR` to determine if the connection succeeded (returns `0`) or failed (returns one of the errors as listed here).
	InProgress,

	/// The system call was interrupted by a signal that was caught.
	Interrupted,
}

impl Display for SocketConnectError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<SocketConnectError as Debug>::fmt(self, f)
	}
}

impl error::Error for SocketConnectError
{
}
