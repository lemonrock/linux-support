// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// Binds a socket as a server (listener).
	///
	/// On success, zero is returned.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EACCES`: The address is protected, and the user is not the superuser.
	/// * `EADDRINUSE`: The given address is already in use.
	/// * `EBADF`: `sockfd` is not a valid descriptor.
	/// * `EINVAL`: The socket is already bound to an address.
	/// * `ENOTSOCK`: `sockfd` is not a socket file descriptor.
	///
	/// Additionally, for a Unix domain socket, the following errors are documented:-
	///
	/// * `EACCES`: Search permission is denied on a component of the path prefix.
	/// * `EADDRNOTAVAIL`: A nonexistent interface was requested or the requested address was not local.
	/// * `EFAULT`: `addr` points outside the user's accessible address space.
	/// * `EINVAL`: The addrlen is wrong, or the socket was not in the AF_UNIX family.
	/// * `ELOOP`: Too many symbolic links were encountered in resolving `addr`.
	/// * `ENAMETOOLONG`: `addr` is too long.
	/// * `ENOENT`: The file does not exist.
	/// * `ENOMEM`: Insufficient kernel memory was available.
	/// * `ENOTDIR`: A component of the path prefix is not a directory.
	/// * `EROFS`: The socket inode would reside on a read-only file system.
	///
	/// Whilst not documenting, it seems reasonable that `EAFNOSUPPORT` could occur as it can occur for `connect()`.
	pub(crate) fn bind(sockfd: RawFd, addr: *const sockaddr_storage, len: socklen_t) -> c_int;
}
