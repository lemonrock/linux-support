// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `mq_timedsend()` adds the message pointed to by `msg_ptr` to the message queue referred to by the message queue descriptor `mqdes`.
	///
	/// The `msg_len` argument specifies the length of the message pointed to by `msg_ptr`; this length must be less than or equal to the queue's `mq_msgsize` attribute.
	/// Zero-length messages are allowed.
	///
	/// The `msg_prio` argument is a non-negative integer that specifies the priority of this message.
	/// PosixMessages are placed on the queue in decreasing order of priority, with newer messages of the same priority being placed after older messages with the same priority.
	///
	/// If the message queue is already full (the number of messages on the queue equals the queue's `mq_maxmsg` attribute) then the call instead fails immediately with the error `EAGAIN`.
	///
	/// `abs_timeout` can be `NULL` (this mirrors then `mq_send()`).
	///
	/// On success, `mq_timedsend()` returns zero.
	/// On error, `-1` is returned, with `errno` set to indicate the error.
	///
	/// Errors documented to be returned from `mq_getattr()` in `errno`:-
	///
	/// * `EAGAIN`: The queue was full, and the `O_NONBLOCK` flag was set for the message queue description referred to by `mqdes`.
	/// * `EBADF`: The descriptor specified in `mqdes` was invalid or not opened for writing.
	/// * `EINTR`: The call was interrupted by a signal handler.
	/// * `EINVAL`: The call would have blocked, and `abs_timeout` was invalid, either because `tv_sec` was less than zero, or because `tv_nsec` was less than zero or greater than 1000 million.
	/// * `EMSGSIZE`: `msg_len` was greater than the `mq_msgsize` attribute of the message queue.
	/// * `ETIMEDOUT`: The call timed out before a message could be transferred.
	pub(crate) fn mq_timedsend(mqd: mqd_t, msg_ptr: *const c_char, msg_len: size_t, prio: c_uint, abs_timeout: *const timespec) -> c_int;
}
