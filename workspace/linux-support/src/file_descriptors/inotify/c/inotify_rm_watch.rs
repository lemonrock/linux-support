// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `inotify_rm_watch()` removes the watch associated with the watch descriptor `wd` from the inotify instance associated with the file descriptor `fd`.
	///
	/// Removing a watch causes an `IN_IGNORED` event to be generated for this watch descriptor.
	///
	/// On success, `inotify_rm_watch()` returns zero.
	/// On error `-1` is returned and `errno` is set appropriately.
	///
	/// Errors documented to be returned from `inotify_rm_watch()` in `errno`:-
	///
	/// * `EBADF`: `fd` is not a valid file descriptor.
	/// * `EINVAL`: The watch descriptor `wd` is not valid; or `fd` is not an inotify file descriptor.
	///
	/// Since Linux 2.6.13.
	pub(crate) fn inotify_rm_watch(fd: RawFd, wd: c_int) -> c_int;
}
