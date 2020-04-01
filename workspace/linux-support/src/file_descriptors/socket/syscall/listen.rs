// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// Makes a bound socket a listener.
	///
	/// On success, zero is returned.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EADDRINUSE`: Another socket is already listening on the same port.
	/// * `EBADF`: `sockfd` is not a valid descriptor.
	/// * `ENOTSOCK`: `sockfd` is not a socket file descriptor.
	/// * `EOPNOTSUPP`: The socket is not of a type that supports the `listen()` operation.
	pub(crate) fn listen(sockfd: c_int, backlog: c_int) -> c_int;
}

