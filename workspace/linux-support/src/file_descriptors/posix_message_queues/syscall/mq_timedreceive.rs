// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `mq_timedreceive()` removes the oldest message with the highest priority from the message queue referred to by the message queue descriptor  `mqdes`, and places it in the buffer pointed to by `msg_ptr`.
	///
	/// The `msg_len` argument specifies the size of the buffer pointed to by `msg_ptr`; this must be greater than or equal to the `mq_msgsize` attribute of the queue.
	/// If `msg_prio` is not `NULL`, then the buffer to which it points is used to return the priority associated with the received message.
	///
	/// If the queue is empty, then, by default, `mq_receive()` then the call instead fails immediately with the error `EAGAIN`.
	///
	/// `abs_timeout` can be `NULL` (this mirrors then `mq_receive()`).
	///
	/// On success, `mq_timedreceive()` returns the number of bytes in the received message.
	/// On error, `-1` is returned, with `errno` set to indicate the error.
	///
	/// Errors documented to be returned from `mq_getattr()` in `errno`:-
	///
	/// * `EAGAIN`: The queue was empty, and the `O_NONBLOCK` flag was set for the message queue description referred to by `mqdes`.
	/// * `EBADF`: The descriptor specified in `mqdes` was invalid or not opened for reading.
	/// * `EINTR`: The call was interrupted by a signal handler.
	/// * `EINVAL`: The call would have blocked, and `abs_timeout` was invalid, either because `tv_sec` was less than zero, or because `tv_nsec` was less than zero or greater than 1000 million.
	/// * `EMSGSIZE`: `msg_len` was less than the `mq_msgsize` attribute of the message queue.
	/// * `ETIMEDOUT`: The call timed out before a message could be transferred.
	pub(crate) fn mq_timedreceive(mqd: mqd_t, msg_ptr: *mut c_char, msg_len: size_t, prio: *mut c_uint, abs_timeout: *const timespec) -> ssize_t;
}
