// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// Transmits data.
	///
	/// The flags argument is either zero or a bitwise or of the flags `MSG_CONFIRM`, `MSG_DONTROUTE`, `MSG_DONTWAIT`, `MSG_EOR`, `MSG_MORE`, `MSG_NOSIGNAL` or `MSG_OOB`.
	///
	/// On success, returns the number of bytes sent.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EACCES`: Write permission is denied on the destination socket file, or search permission is denied for one of the directories the path prefix (only for Unix domain sockets).
	/// * `EAGAIN`: The socket is marked nonblocking and the receive operation would block, or a receive timeout had been set and the timeout expired before data was received.
	/// * `EBADF`: The argument `sockfd` is an invalid descriptor.
	/// * `ECONNRESET`: Connection reset by peer.
	/// * `EDESTADDRREQ`: The socket is not connection-mode, and no peer address is set.
	/// * `EFAULT`: An invalid user space address was specified for an argument.
	/// * `EINTR`: The receive was interrupted by delivery of a signal before any data were available.
	/// * `EINVAL`: Invalid argument passed.
	/// * `EISCONN`: The connection-mode socket was connected already but a recipient was specified.
	/// * `EMSGSIZE`: The socket type requires that message be sent atomically, and the size of the message to be sent made this impossible.
	/// * `ENOBUFS`: The output queue for a network interface was full. This generally indicates that the interface has stopped sending, but may be caused by transient congestion. (Normally, this does not occur in Linux. Packets are just silently dropped when a device queue overflows).
	/// * `ENOMEM`: Could not allocate memory.
	/// * `ENOTCONN`: The socket is associated with a connection-oriented protocol and has not been connected.
	/// * `ENOTSOCK`: The argument `sockfd` does not refer to a socket.
	/// * `EOPNOTSUPP`: Some flags in the `flags` argument are inappropriate for the socket type.
	/// * `EPIPE`: The local end has been shut down on a connection oriented socket (eg a half-close). In this case the process will also receive a `SIGPIPE` unless `MSG_NOSIGNAL` is set in `flags`.
	pub(crate) fn sendto(sockfd: RawFd, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr_storage, addrlen: socklen_t) -> ssize_t;
}
