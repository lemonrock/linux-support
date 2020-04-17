// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// The `socketpair()` call creates an unnamed pair of connected sockets in the specified `domain`, of the specified `type_`, and using the optionally specified `protocol`.
	///
	/// The values of `domain`, `type_` and `protocol` are the same as for `socket()`.
	///
	/// The file descriptors used in referencing the new sockets are returned in `sv[0]` and `sv[1]`.
	///
	/// The two sockets are indistinguishable.
	///
	/// On Linux, the only supported domain for this call is `AF_UNIX` (or synonymously, `AF_LOCAL`).
	///
	/// Since Linux 2.6.27, `socketpair()` supports the `SOCK_NONBLOCK` and `SOCK_CLOEXEC` flags.
	///
	/// On success, zero is returned.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EAFNOSUPPORT`: The specified address family is not supported on this machine.
	/// * `EFAULT`: The address `sv` does not specify a valid part of the process address space.
	/// * `EMFILE`: Too many descriptors are in use by this process.
	/// * `ENFILE`: The system limit on the total number of open files has been reached.
	/// * `EOPNOTSUPP`: The specified `protocol` does not support creation of socket pairs.
	/// * `EPROTONOSUPPORT`: The specified `protocol` is not supported on this machine.
	pub(crate) fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut [c_int; 2]) -> c_int;
}
