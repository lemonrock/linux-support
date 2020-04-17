// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `mq_open() `creates a new POSIX message queue or opens an existing queue.
	///
	/// The queue is identified by `name`.
	/// A `name` of the form `/somename\0`.
	/// It is a null-terminated string of up to `NAME_MAX` (`255`) characters consisting of an initial slash, followed by one or more characters, none of which are slashes.
	/// Note that this means the string `/\0` is invalid.
	///
	/// The `oflag` argument specifies flags that control the operation of the call.
	/// Exactly one of `O_RDONLY`, `O_WRONLY` or `O_RDWR` must be specified in `oflag`.
	/// In addition, one or more of the following may be bitwise or-ed into `oflag`: `O_CLOEXEC`, `O_CREAT`, `O_EXCL` and `O_NONBLOCK`.
	///
	/// If `O_CREAT` is specified in `oflag`, then two additional arguments must be supplied: `mode: mode_t` and `attr: *mut mq_attr`.
	///
	/// The `mode` argument specifies the permissions to be placed on the new queue, as for `man 2 open`.
	/// The permissions settings are masked against the process umask.
	///
	/// The fields of the `mq_attr` pointed to by `attr` specify the maximum number of messages and the maximum size of messages that the queue will allow.
	///
	/// Only the `mq_maxmsg` and `mq_msgsize` fields are employed when calling `mq_open()`; the values in the remaining fields are ignored.
	///
	/// If `attr` is `NULL`, then the queue is created with implementation- defined default attributes.
	/// Since Linux 3.5, two /proc files can be used to control these defaults.
	///
	/// On success, `mq_open(`) returns a message queue descriptor for use by other message queue functions.
	/// On error, `mq_open()` returns `-1`, with `errno` set to indicate the error.
	///
	/// Errors documented to be returned from `mq_open()` in `errno`:-
	///
	/// * `EACCES`: The queue exists, but the caller does not have permission to open it in the specified mode.
	/// * `EACCES`: `name` contained more than one slash.
	/// * `EEXIST`: Both `O_CREAT` and `O_EXCL` were specified in oflag, but a queue with this name already exists.
	/// * `EINVAL`: `name` doesn't follow the format required.
	/// * `EINVAL`: `O_CREAT` was specified in` oflag`, and `attr` was not `NULL`, but `attr.mq_maxmsg` or `attr.mq_msqsize` was invalid. Both of these fields must be greater than zero. In a process that is unprivileged (does not have the `CAP_SYS_RESOURCE` capability), `attr.mq_maxmsg` must be less than or equal to the `msg_max` limit, and `attr.mq_msgsize` must be less than or equal to the `msgsize_max` limit. In addition, even in a privileged process, `attr.mq_maxmsg` cannot exceed the `HARD_MAX` limit.
	/// * `EMFILE`: The per-process limit on the number of open file and message queue descriptors has been reached.
	/// * `ENAMETOOLONG`: name was too long.
	/// * `ENFILE`: The system-wide limit on the total number of open files and message queues has been reached.
	/// * `ENOENT`: The `O_CREAT` flag was not specified in oflag, and no queue with this name exists.
	/// * `ENOENT`: name was just "/" followed by no other characters.
	/// * `ENOMEM`: Insufficient memory.
	/// * `ENOSPC`: Insufficient space for the creation of a new message queue. This probably occurred because the `queues_max` limit was encountered.
	pub(crate) fn mq_open(name: *const c_char, oflag: c_int, ...) -> c_int;
}
