// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// Opens a socket.
	///
	/// Whilst powerful, the socket interface is showing its age and isn't particularly pleasant to work with.
	///
	/// The `domain` argument specifies a communication domain; this selects the protocol family which will be used for communication.
	/// It is a constant starting `AF_*`; not all of these constants are defined in `socket.h` (for example, those for Infiniband).
	///
	/// `type_` is one of `SOCK_STREAM`, `SOCK_DGRAM`, `SOCK_SEQPACKET`, `SOCK_RAW` or `SOCK_RDM` (`SOCK_PACKET` is obsolete).
	///
	/// Most combinations of `domain` and `type_` are invalid.
	///
	/// Some good combinations:-
	///
	/// * For TCP on IPv4: `domain` is `AF_INET` and `type_` is `SOCK_STREAM`.
	/// * For TCP on IPv6: `domain` is `AF_INET6` and `type_` is `SOCK_STREAM`.
	/// * For UDP on IPv4: `domain` is `AF_INET` and `type_` is `SOCK_DGRAM`.
	/// * For UDP on IPv6: `domain` is `AF_INET6` and `type_` is `SOCK_DGRAM`.
	/// * For a stream on an Unix Domain Socket: `domain` is `AF_UNIX` and `type_` is `SOCK_STREAM`.
	///
	/// On success, a file descriptor for the new socket is returned.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EACCES`: Permission to create a socket of the specified type and/or protocol is denied.
	/// * `EAFNOSUPPORT`: The implementation does not support the specified address family.
	/// * `EINVAL`: Unknown protocol, or protocol family not available.
	/// * `EINVAL`: Invalid flags in type.
	/// * `EMFILE`: Process file table overflow.
	/// * `ENFILE`: The system limit on the total number of open files has been reached.
	/// * `ENOBUFS` or `ENOMEM`: Insufficient memory is available. The socket cannot be created until sufficient resources are freed.
	/// * `EPROTONOSUPPORT`: The protocol type or the specified protocol is not supported within this domain.
	pub(crate) fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
}

/// Sets the `O_NONBLOCK` file status flag on the newly opened socket file description.
///
/// Since Linux 2.6.27.
pub(crate) const SOCK_NONBLOCK: c_int = O_NONBLOCK;

/// Set the close-on-exec (`FD_CLOEXEC`) flag on the newly opened socket file description.
///
/// Since Linux 2.6.27.
pub(crate) const SOCK_CLOEXEC: c_int = O_CLOEXEC;
