// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// Sets a socket option.
	///
	/// On success, zero is returned.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EBADF`: The argument `sockfd` is not a valid descriptor.
	/// * `EFAULT`: The address pointed to by `optval` is not in a valid part of the process address space.
	/// * `EINVAL`: `optlen` is invalid, or there is an invalid value in `optval`.
	/// * `ENOPROTOOPT`: The option is unknown at the level indicated.
	/// * `ENOTSOCK`: The argument `sockfd` is a file, not a socket.
	pub(crate) fn setsockopt(sockfd: RawFd, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
}
