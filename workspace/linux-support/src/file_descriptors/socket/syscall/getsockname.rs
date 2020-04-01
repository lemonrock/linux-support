// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `getsockname()` returns the current address to which the socket `sockfd` is bound, in the buffer pointed to by `addr`.
	/// The `addrlen` argument should be initialized to indicate the amount of space (in bytes) pointed to by `addr`.
	/// On return it contains the actual size of the socket address.
	///
	/// The returned address is truncated if the buffer provided is too small; in this case, addrlen will return a value greater than was supplied to the call.
	///
	/// On success, zero is returned.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EBADF`: The argument `sockfd` is not a valid file descriptor.
	/// * `EFAULT`: The `addr` argument points to memory not in a valid part of the process address space.
	/// * `EINVAL`: `addrlen` is invalid.
	/// * `ENOBUFS`: Insufficient resources were available in the system to perform the operation.
	/// * `ENOTSOCK`: The file descriptor `sockfd` does not refer to a socket.
	pub(crate) fn getsockname(sockfd: RawFd, addr: *mut sockaddr_storage, addrlen: *mut socklen_t) -> c_int;
}
