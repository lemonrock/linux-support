// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Register the target file descriptor `fd` on the epoll instance referred to by the file descriptor `epfd` and associate the event `event` with the internal file linked to `fd`.
pub(crate) const EPOLL_CTL_ADD: c_int = 1;

/// Change the event `event` associated with the target file descriptor `fd`.
pub(crate) const EPOLL_CTL_DEL: c_int = 2;

/// Remove (deregister) the target file descriptor `fd` from the epoll instance referred to by `epfd`.
/// The event `event` is ignored and can be `NULL`.
pub(crate) const EPOLL_CTL_MOD: c_int = 3;

#[link(name = "c")]
extern "C"
{
	/// Adds, modifies or removes a file descriptor to monitor using epoll.
	///
	/// The value of `op` controls where an add (`EPOLL_CTL_ADD`), modification (`EPOLL_CTL_MOD`) or removal (`EPOLL_CTL_DEL`) occurs.
	///
	/// The value of `op` dictates what `event` points to
	///
	/// When successful, `epoll_ctl()` returns zero.
	/// When an error occurs, `epoll_ctl()` returns `-1` and `errno` is set appropriately.
	/// 
	/// Errors documented to be returned from `epoll_ctl()` in `errno`:-
	/// 
	/// * `EBADF`: `epfd` or `fd` is not a valid file descriptor.
	/// * `EEXIST`: `op` was `EPOLL_CTL_ADD`, and the supplied file descriptor `fd` is already registered with this epoll instance.
	/// * `EINVAL`: `epfd` is not an epoll file descriptor, or `fd` is the same as `epfd`, or the requested operation `op` is not supported by this interface.
	/// * `EINVAL`: An invalid event type was specified along with `EPOLLEXCLUSIVE` in `events`.
	/// * `EINVAL`: `op` was `EPOLL_CTL_MOD` and `events` included `EPOLLEXCLUSIVE`.
	/// * `EINVAL`: `op` was `EPOLL_CTL_MOD` and the `EPOLLEXCLUSIVE` flag has previously been applied to this `epfd`, `fd` pair.
	/// * `EINVAL`: `EPOLLEXCLUSIVE` was specified in `events` and `fd` refers to an epoll instance.
	/// * `ELOOP`: `fd` refers to an epoll instance and this `EPOLL_CTL_ADD` operation would result in a circular loop of epoll instances monitoring one another.
	/// * `ENOENT`: `op` was `EPOLL_CTL_MOD` or `EPOLL_CTL_DEL`, and `fd` is not registered with this epoll instance.
	/// * `ENOMEM`: There was insufficient memory to handle the requested `op` control operation.
	/// * `ENOSPC`: The limit imposed by `/proc/sys/fs/epoll/max_user_watches` was encountered while trying to register (`EPOLL_CTL_ADD`) a new file descriptor on an epoll instance.
	/// * `EPERM`: The target file `fd` does not support epoll. This error can occur if `fd` refers to, for example, a regular file or a directory.
	pub(crate) fn epoll_ctl(epfd: RawFd, op: c_int, fd: RawFd, event: *mut epoll_event) -> c_int;
}
