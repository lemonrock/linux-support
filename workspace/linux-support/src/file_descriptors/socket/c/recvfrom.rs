// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// Receives data from a socket.
	///
	/// Pass `NULL` for `addr` and `addrlen` for connected sockets (eg TCP connections).
	///
	/// On success, returns the number of bytes sent.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EAGAIN`: The socket is marked nonblocking and the receive operation would block, or a receive timeout had been set and the timeout expired before data was received.
	/// * `EBADF`: The argument `sockfd` is an invalid descriptor.
	/// * `ECONNREFUSED`: A remote host refused to allow the network connection (typically because it is not running the requested service).
	/// * `EFAULT`: The receive buffer pointer(s) point outside the process's address space.
	/// * `EINTR`: The receive was interrupted by delivery of a signal before any data were available.
	/// * `EINVAL`: Invalid argument passed.
	/// * `ENOMEM`: Could not allocate memory.
	/// * `ENOTCONN`: The socket is associated with a connection-oriented protocol and has not been connected.
	/// * `ENOTSOCK`: The argument `sockfd` does not refer to a socket.
	///
	/// Whilst not documented, it seems possible that `EOPNOTSUPP` could occur (eg because some flags in the `flags` argument are inappropriate for the socket type).
	///
	/// Additionally, [this stack overflow question](https://stackoverflow.com/questions/10387082/unix-ipc-socket-closing-one-end-without-reading-from-it) seems to imply that `ECONNRESET` can occur for Unix domain sockets.
	pub(crate) fn recvfrom(sockfd: RawFd, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr_storage, addrlen: *mut socklen_t) -> ssize_t;
}
