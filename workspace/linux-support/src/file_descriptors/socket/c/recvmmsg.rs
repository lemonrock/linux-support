// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// The `recvmmsg()` system call is an extension of `recvmsg()` that allows the caller to receive multiple messages from a socket using a single system call.
	///
	/// A further extension over `recvmsg()` is support for a timeout on the receive operation.
	///
	/// The `sockfd` argument is the file descriptor of the socket to receive data from.
	///
	/// The `msgvec` argument is a pointer to an array of `mmsghdr` structures.
	///
	/// The size of this array is specified in `vlen`.
	///
	/// The `flags` argument contains flags ORed together.
	/// The flags are the same as documented for `recvmsg()`, with the addition of `MSG_WAITFORONE`.
	///
	/// The `timeout` argument points to a struct timespec defining a timeout (seconds plus nanoseconds) for the receive operation.
	/// (This interval will be rounded up to the system clock granularity, and kernel scheduling delays mean that the blocking interval may overrun by a small amount).
	/// If `timeout` is `NULL`, then the operation blocks indefinitely.
	///
	/// A blocking `recvmmsg()` call blocks until `vlen` messages have been received or until the `timeout` expires.
	/// A nonblocking call reads as many messages as are available (up to the limit specified by `vlen`) and returns immediately.
	///
	/// On return from `recvmmsg()`, successive elements of `msgvec` are updated to contain information about each received message:-
	///
	/// * `msgvec.msg_len` contains the size of the received message
	/// * the subfields of `msg_hdr` are updated as described in `recvmsg()`.
	///
	/// The return value of the call indicates the number of elements of `msgvec` that have been updated.
	/// On success, `recvmmsg()` returns the number of messages received in `msgvec`.
	/// On error, `-1` is returned, and `errno` is set to indicate the error.
	///
	/// Errors are as for `recvmsg()`.
	/// If the `timeout` is invalid, `EINVAL` is set in `errno`.
	pub(crate) fn recvmmsg(sockfd: RawFd, msgvec: *mut mmsghdr, vlen: c_uint, flags: c_int, timeout: *mut timespec) -> c_int;
}
