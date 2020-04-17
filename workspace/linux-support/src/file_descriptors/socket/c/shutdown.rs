// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `shutdown()` is most useful for initiating TCP's `FIN` state machine, and so freeing up resources inside Linux.
	///
	/// There is very little advantages in half-close, especially when sockets are used with upper layer protocols such as TLS, as these can do reads when a logical write is required, and vice-versa.
	///
	/// `how` is one of `SHUT_RD` (shutdown read), `SHUT_WR` (shutdown write) or `SHUT_RDWR` (shutdown read and write).
	///
	/// On success, zero is returned.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EBADF`: `sockfd` is not a valid file descriptor.
	/// * `EINVAL`: An invalid value was specified in `how`.
	/// * `ENOTCONN`: The specified socket is not connected.
	/// * `ENOTSOCK`: The file descriptor `sockfd` does not refer to a socket.
	pub(crate) fn shutdown(sockfd: RawFd, how: c_int) -> c_int;
}

/// Half-close read.
#[allow(dead_code)]
pub(crate) const SHUT_RD: c_int = 0;

/// Half-close write.
#[allow(dead_code)]
pub(crate) const SHUT_WR: c_int = 1;

/// Close read and write.
pub(crate) const SHUT_RDWR: c_int = 2;
