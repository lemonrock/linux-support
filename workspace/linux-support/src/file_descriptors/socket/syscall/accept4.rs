// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// The `accept4()` system call is used with connection-based socket types.
	///
	/// The argument `sockfd` is a socket that has been created with `socket()`, bound to a local address with `bind()`, and is listening for connections after a `listen()`.
	///
	/// The argument `addr` is a pointer to a `sockaddr` structure.
	/// This structure is filled in with the address of the peer socket, as known to the communications layer.
	/// The exact format of the address returned `addr` is determined by the socket's address family.
	/// When `addr` is `NULL`, nothing is filled in; in this case, `addrlen` is not used, and should also be `NULL`.
	///
	/// The `addrlen` argument is a value-result argument: the caller must initialize it to contain the size (in bytes) of the structure pointed to by `addr`; on return it will contain the actual size of the peer address.
	///
	/// The returned address is truncated if the buffer provided is too small; in this case, `addrlen` will return a value greater than was supplied to the call.
	///
	/// If `flags` is 0, then `accept4()` is the same as `accept()`.
	/// `flags` can be a bitwise or of `SOCK_NONBLOCK` or `SOCK_CLOEXEC`.
	///
	/// On success, returns a nonnegative integer that is a descriptor for the accepted socket.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EAGAIN`: The socket is marked nonblocking and no connections are present to be accepted. POSIX.1-2001 allows either error to be returned for this case, and does not require these constants to have the same value, so a portable application should check for both possibilities.
	/// * `EBADF`: The descriptor is invalid.
	/// * `ECONNABORTED`: A connection has been aborted.
	/// * `EFAULT`: The addr argument is not in a writable part of the user address space.
	/// * `EINTR`: The system call was interrupted by a signal that was caught before a valid connection arrived.
	/// * `EINVAL`: Socket is not listening for connections, or `addrlen` is invalid, or the `flags` are invalid.
	/// * `EMFILE`: The per-process limit of open file descriptors has been reached.
	/// * `ENFILE`: The system limit on the total number of open files has been reached.
	/// * `ENOBUFS` or `ENOMEM`: Not enough free memory. This often means that the memory allocation is limited by the socket buffer limits, not by the system memory.
	/// * `ENOTSOCK`: The descriptor references a file, not a socket.
	/// * `EOPNOTSUPP`: The referenced socket is not of type `SOCK_STREAM`.
	/// * `EPROTO`: Protocol error.
	/// * `EPERM`: Firewall rules forbid connection.
	/// * `ENOSR`: ?
	/// * `ESOCKTNOSUPPORT`: ?
	/// * `EPROTONOSUPPORT`: ?
	/// * `ETIMEDOUT`: Treat as for `ECONNABORTED`?
	/// * `ERESTARTSYS`: ? Should never be seen by user programs, but man page implies it can occur during 'tracing'.
	pub(crate) fn accept4(sockfd: RawFd, addr: *mut sockaddr_storage, addrlen: *mut socklen_t, flags: c_int) -> c_int;
}
