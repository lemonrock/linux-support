// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `inotify_add_watch()` adds a new watch, or modifies an existing watch, for the file whose location is specified in pathname; the caller must have read permission for this file.
	///
	/// The `fd` argument is a file descriptor referring to the inotify instance whose watch list is to be modified.
	/// The events to be monitored for `pathname` are specified in the mask `mask` argument.
	///
	/// A successful call to `inotify_add_watch()` returns the unique watch descriptor associated with pathname for this inotify instance.
	/// If `pathname` was not previously being watched by this inotify instance, then the watch descriptor is newly allocated.
	/// If `pathname` was already being watched, then the descriptor for the existing watch is returned.
	///
	/// The watch descriptor is returned by later `read()` from the inotify file descriptor `fd`.
	/// These reads fetch `inotify_event` structures.
	/// The watch descriptor inside this structure identifies the object for which the event occurred.
	///
	/// On success, `inotify_add_watch()` returns a nonnegative watch descriptor.
	/// On error `-1` is returned and `errno` is set appropriately.
	///
	/// Errors documented to be returned from `inotify_add_watch()` in `errno`:-
	///
	/// * `EACCES`: Read access to the given file is not permitted.
	/// * `EBADF`: `fd` is not a valid file descriptor.
	/// * `EFAULT`: `pathname` points outside of the process's accessible address space.
	/// * `EINVAL`: The given event `mask` contains no valid events; or `fd` is not an inotify file descriptor.
	/// * `ENOENT`: A directory component in `pathname` does not exist or is a dangling symbolic link.
	/// * `ENOMEM`: Insufficient kernel memory was available.
	/// * `ENOSPC`: The user limit on the total number of inotify watches was reached or the kernel failed to allocate a needed resource.
	///
	/// Since Linux 2.6.13.
	pub(crate) fn inotify_add_watch(fd: RawFd, pathname: *const c_char, mask: u32) -> c_int;
}

/// File was accessed (read).
///
/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_ACCESS: u32 = 0x00000001;

/// File was modified.
///
/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_MODIFY: u32 = 0x00000002;

/// Metadata changed.
///
/// For example, permissions, timestamps, extended attributes, UID, GID, etc.
/// Since Linux 2.6.25, this also includes the link count.
///
/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_ATTRIB: u32 = 0x00000004;

/// File opened for writing was closed.
///
/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_CLOSE_WRITE: u32 = 0x00000008;

/// File not opened for writing was closed.
///
/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_CLOSE_NOWRITE: u32 = 0x00000010;

/// File was opened.
///
/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_OPEN: u32 = 0x00000020;

/// File moved out of watched directory.
///
/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_MOVED_FROM: u32 = 0x00000040;

/// File moved into watched directory.
///
/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_MOVED_TO: u32 = 0x00000080;

/// File/directory created in watched directory.
///
/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_CREATE: u32 = 0x00000100;

/// File/directory deleted from watched directory.
///
/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_DELETE: u32 = 0x00000200;

/// Watched file/directory was itself deleted.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_DELETE_SELF: u32 = 0x00000400;

/// Watched file/directory was itself moved.
///
/// Valid for `inotify_add_watch()`'s `mask` argument.
/// Can be set in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_MOVE_SELF: u32 = 0x00000800;

/// This is equivalent to `IN_ACCESS` | `IN_ATTRIB` | `IN_CLOSE_WRITE` | `IN_CLOSE_NOWRITE` | `IN_CREATE` | `IN_DELETE` | `IN_DELETE_SELF` | `IN_MODIFY` | `IN_MOVE_SELF` | `IN_MOVED_FROM` | `IN_MOVED_TO` | `IN_OPEN`.
///
/// Valid for `inotify_add_watch()`'s `mask` argument only.
pub(crate) const IN_ALL_EVENTS: u32 = IN_ACCESS | IN_ATTRIB | IN_CLOSE_WRITE | IN_CLOSE_NOWRITE | IN_CREATE | IN_DELETE | IN_DELETE_SELF | IN_MODIFY | IN_MOVE_SELF | IN_MOVED_FROM | IN_MOVED_TO | IN_OPEN;

/// This is equivalent to `IN_MOVED_FROM` | `IN_MOVED_TO`.
///
/// Valid for `inotify_add_watch()`'s `mask` argument only.
pub(crate) const IN_MOVE: u32 = IN_MOVED_FROM | IN_MOVED_TO;

/// This is equivalent to `IN_CLOSE_WRITE` | `IN_CLOSE_NOWRITE`.
///
/// Valid for `inotify_add_watch()`'s `mask` argument only.
pub(crate) const IN_CLOSE: u32 = IN_CLOSE_WRITE | IN_CLOSE_NOWRITE;

/// Only watch `pathname` if it is a directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument only.
///
/// Since Linux 2.6.15.
pub(crate) const IN_ONLYDIR: u32 = 0x01000000;

/// Don't dereference `pathname` if it is a symbolic link.
///
/// Valid for `inotify_add_watch()`'s `mask` argument only.
///
/// Since Linux 2.6.15.
pub(crate) const IN_DONT_FOLLOW: u32 = 0x02000000;

/// By default, when watching events on the children of a directory, events are generated for children even after they have been unlinked from the directory.
///
/// This can result in large numbers of uninteresting events for some applications (eg, if watching `/tmp`, in which many applications create temporary files whose names are immediately unlinked).
/// Specifying `IN_EXCL_UNLINK` changes the default behavior, so that events are not generated for children after they have been unlinked from the watched directory.
///
/// Valid for `inotify_add_watch()`'s `mask` argument only.
///
/// Since Linux 2.6.36.
pub(crate) const IN_EXCL_UNLINK: u32 = 0x04000000;

/// Add (`or`) events to watch `mask` for this `pathname` if it already exists (instead of replacing `mask`).
///
/// Valid for `inotify_add_watch()`'s `mask` argument only.
pub(crate) const IN_MASK_ADD: u32 = 0x20000000;

/// Monitor `pathname` for one event, then remove from watch list.
///
/// Valid for `inotify_add_watch()`'s `mask` argument only.
pub(crate) const IN_ONESHOT: u32 = 0x80000000;

/// File system containing watched object was unmounted.
///
/// Valid only in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_UNMOUNT: u32 = 0x00002000;

/// Event queue overflowed.
///
/// `wd` is `-1` for this event.
///
/// Valid only in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_Q_OVERFLOW: u32 = 0x00004000;

/// Watch was removed.
///
/// This can have been done explicitly using `inotify_rm_watch()` or automatically because the file was deleted, or its file system was unmounted.
///
/// Valid only in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_IGNORED: u32 = 0x00008000;

/// Subject of this event is a directory.
///
/// Valid only in the `mask` field of the `inotify_event` structure returned from `read()`.
pub(crate) const IN_ISDIR: u32 = 0x40000000;
