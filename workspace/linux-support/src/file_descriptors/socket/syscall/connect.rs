// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// Connect a socket as a client.
	///
	/// On success, zero is returned.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EACCES`: For UNIX domain sockets, which are identified by pathname: Write permission is denied on the socket file, or search permission is denied for one of the directories in the path prefix.
	/// * `EACCES` or `EPERM`: The user tried to connect to a broadcast address without having the socket broadcast flag enabled or the connection request failed because of a local firewall rule.
	/// * `EADDRINUSE`: Local address is already in use.
	/// * `EAFNOSUPPORT`: The passed address did not have the correct address family in its `sa_family` field.
	/// * `EAGAIN`: No more free local ports or insufficient entries in the routing cache (eg try adjusting `/proc/sys/net/ipv4/ip_local_port_range` for `AF_INET`).
	/// * `EALREADY`: The socket is nonblocking and a previous connection attempt has not yet been completed.
	/// * `EBADF`: The file descriptor is not a valid index in the descriptor table.
	/// * `ECONNREFUSED`: No-one listening on the remote address.
	/// * `EFAULT`: The socket structure address is outside the user's address space.
	/// * `EINPROGRESS`: The socket is nonblocking and the connection cannot be completed immediately. After `epoll_wait()` indicates the socket is writable use `getsockopt()` with `SOL_SOCKET` and `SO_ERROR` to determine if the connection succeeded (returns `0`) or failed (returns a `E*` code as listed here).
	/// * `EINTR`: The system call was interrupted by a signal that was caught.
	/// * `EISCONN`: The socket is already connected.
	/// * `ENETUNREACH`: Network is unreachable.
	/// * `ENOTSOCK`: The file descriptor is not associated with a socket.
	/// * `ETIMEDOUT`: Timeout while attempting connection.
	pub(crate) fn connect(sockfd: RawFd, addr: *const sockaddr_storage, len: socklen_t) -> c_int;
}
