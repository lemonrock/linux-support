// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `fanotify_mark()` adds, removes, or modifies an fanotify mark on a filesystem object.
	///
	/// The caller must have read permission on the filesystem object that is to be marked.
	///
	/// `flags` is a bit mask describing the modification to perform.
	/// It must include exactly one of `FAN_MARK_ADD`, `FAN_MARK_REMOVE` or `FAN_MARK_FLUSH`.
	///
	/// `mask` defines which events shall be listened for (or which shall be ignored).
	/// It is a bit mask of `FAN_ACCESS`, `FAN_MODIFY`, `FAN_CLOSE_WRITE`, `FAN_CLOSE_NOWRITE`, `FAN_OPEN`, `FAN_Q_OVERFLOW`, `FAN_OPEN_PERM`, `FAN_ACCESS_PERM`, `FAN_ONDIR` and `FAN_EVENT_ON_CHILD`.
	///
	/// The filesystem object to be marked is determined by the file descriptor `dirfd` and the pathname specified in `pathname`:-
	///
	/// * If pathname is `NULL`, `dirfd` defines the filesystem object to be marked.
	/// * If pathname is `NULL`, and `dirfd` takes the special value `AT_FDCWD`, the current working directory is to be marked.
	/// * If pathname is absolute, it defines the filesystem object to be marked, and `dirfd` is ignored.
	/// * If pathname is relative, and `dirfd` does not have the value `AT_FDCWD`, then the filesystem object to be marked is determined by interpreting `pathname` relative the directory referred to by `dirfd`.
	/// * If pathname is relative, and `dirfd` has the value `AT_FDCWD`, then the filesystem object to be marked is determined by interpreting `pathname` relative the current working directory.
	///
	/// On success, `fanotify_mark()` returns `0`.
	/// On error, `-1` is returned, and `errno` is set to indicate the error.
	///
	/// Errors documented to be returned from `fanotify_mark()` in `errno`:-
	///
	/// * `EBADF`: An invalid file descriptor was passed in `fanotify_fd`.
	/// * `EINVAL`: An invalid value was passed in `flags` or `mask`, or `fanotify_fd` was not an fanotify file descriptor, or the fanotify file descriptor was opened with `FAN_CLASS_NOTIF` and mask contains a flag for permission events (`FAN_OPEN_PERM` or `FAN_ACCESS_PERM`).
	/// * `ENOENT`: The filesystem object indicated by `dirfd` and `pathname` does not exist. This error also occurs when trying to remove a mark from an object which is not marked.
	/// * `ENOMEM`: The necessary memory could not be allocated.
	/// * `ENOSPC`: The number of marks exceeds the limit of `8192` and the `FAN_UNLIMITED_MARKS` flag was not specified when the fanotify file descriptor was created with `fanotify_init()`.
	/// * `ENOSYS` This kernel does not implement `fanotify_mark()`. The fanotify API is available only if the kernel was configured with `CONFIG_FANOTIFY`.
	/// * `ENOTDIR`: `flags` contains `FAN_MARK_ONLYDIR`, and `dirfd` and `pathname` do not specify a directory.
	pub(crate) fn fanotify_mark(fanotify_fd: RawFd, flags: c_uint, mask: u64, dirfd: RawFd, pathname: *const c_char) -> c_int;
}

/// Create an event when a file or directory is accessed (read).
pub(crate) const FAN_ACCESS: u64 = 0x01;

/// Create an event when a file is modified (write).
pub(crate) const FAN_MODIFY: u64 = 0x02;

/// Create an event when a writable file is closed.
pub(crate) const FAN_CLOSE_WRITE: u64 = 0x08;

/// Create an event when a read-only file or directory is closed.
pub(crate) const FAN_CLOSE_NOWRITE: u64 = 0x10;

/// Create an event when a file or directory is opened.
pub(crate) const FAN_OPEN: u64 = 0x20;

/// Create an event when an overflow of the event queue occurs.
///
/// The size of the event queue is limited to 16384 entries if `FAN_UNLIMITED_QUEUE` is not set in `fanotify_init()`.
pub(crate) const FAN_Q_OVERFLOW: u64 = 0x4000;

/// Create an event when a permission to open a file or directory is requested.
///
/// A fanotify file descriptor created with `FAN_CLASS_PRE_CONTENT` or `FAN_CLASS_CONTENT` is required.
pub(crate) const FAN_OPEN_PERM: u64 = 0x10000;

/// Create an event when a permission to read a file or directory is requested.
///
/// A fanotify file descriptor created with `FAN_CLASS_PRE_CONTENT` or `FAN_CLASS_CONTENT` is required.
pub(crate) const FAN_ACCESS_PERM: u64 = 0x20000;

/// Create events for directories.
///
/// For example, when `opendir()`, `readdir()` or `closedir()` are called.
///
/// Without this flag, only events for files are created.
pub(crate) const FAN_ONDIR: u64 = 0x40000000;

/// Events for the immediate children of marked directories shall be created.
///
/// The flag has no effect when marking file system mounts.
/// Note that events are not generated for children of the subdirectories of marked directories.
/// To monitor complete directory trees it is necessary to mark the relevant mount.
pub(crate) const FAN_EVENT_ON_CHILD: u64 = 0x08000000;

/// A file is closed.
///
/// This is the combination of `FAN_CLOSE_WRITE` and `FAN_CLOSE_NOWRITE`.
pub(crate) const FAN_CLOSE: u64 = FAN_CLOSE_WRITE | FAN_CLOSE_NOWRITE;

/// This is a combination of `FAN_ACCESS`, `FAN_MODIFY`, `FAN_CLOSE` and `FAN_OPEN`.
pub(crate) const FAN_ALL_EVENTS: u64 = FAN_ACCESS | FAN_MODIFY | FAN_CLOSE | FAN_OPEN;

/// This is a combination of `FAN_OPEN_PERM` and `FAN_ACCESS_PERM`.
pub(crate) const FAN_ALL_PERM_EVENTS: u64 = FAN_OPEN_PERM | FAN_ACCESS_PERM;

/// This is a combination of `FAN_ALL_EVENTS`, `FAN_ALL_PERM_EVENTS` and `FAN_Q_OVERFLOW`.
pub(crate) const FAN_ALL_OUTGOING_EVENTS: u64 = FAN_ALL_EVENTS | FAN_ALL_PERM_EVENTS | FAN_Q_OVERFLOW;

/// The events in `mask` will be added to the mark mask (or to the ignore mask).
///
/// `mask` must be nonempty or the error `EINVAL` will occur.
pub(crate) const FAN_MARK_ADD: c_uint = 0x01;

/// The events in argument `mask` will be removed from the mark mask (or from the ignore mask).
///
/// `mask` must be nonempty or the error `EINVAL` will occur.
pub(crate) const FAN_MARK_REMOVE: c_uint = 0x02;

/// Remove either all mount or all non-mount marks from the fanotify group.
///
/// If flags contains `FAN_MARK_MOUNT`, all marks for mounts are removed from the group.
/// Otherwise, all marks for directories and files are removed.
/// No flag other than `FAN_MARK_MOUNT` can be used in conjunction with `FAN_MARK_FLUSH`.
///
/// `mask is ignored`.
pub(crate) const FAN_MARK_FLUSH: c_uint = 0x80;

/// If `pathname` is a symbolic link, mark the link itself, rather than the file to which it refers.
///
/// (By default, `fanotify_mark()` dereferences pathname if it is a symbolic link).
pub(crate) const FAN_MARK_DONT_FOLLOW: c_uint = 0x04;

/// If the filesystem object to be marked is not a directory, the error `ENOTDIR` shall be raised.
pub(crate) const FAN_MARK_ONLYDIR: c_uint = 0x08;

/// Mark the mount point specified by pathname.
///
/// If `pathname` is not itself a mount point, the mount point containing `pathname` will be marked.
/// All directories, subdirectories, and the contained files of the mount point will be monitored.
pub(crate) const FAN_MARK_MOUNT: c_uint = 0x10;

/// The events in mask shall be added to or removed from the ignore mask.
pub(crate) const FAN_MARK_IGNORED_MASK: c_uint = 0x20;

/// The ignore mask shall survive modify events.
///
/// If this flag is not set, the ignore mask is cleared when a modify event occurs for the ignored file or directory.
pub(crate) const FAN_MARK_IGNORED_SURV_MODIFY: c_uint = 0x40;

/// This is a combination of `FAN_MARK_ADD`, `FAN_MARK_REMOVE`, `FAN_MARK_DONT_FOLLOW`, `FAN_MARK_ONLYDIR`, `FAN_MARK_MOUNT`, `FAN_MARK_IGNORED_MASK`, `FAN_MARK_IGNORED_SURV_MODIFY` and `FAN_MARK_FLUSH`.
#[allow(dead_code)]
pub(crate) const FAN_ALL_MARK_FLAGS: c_uint = FAN_MARK_ADD | FAN_MARK_REMOVE | FAN_MARK_DONT_FOLLOW | FAN_MARK_ONLYDIR | FAN_MARK_MOUNT | FAN_MARK_IGNORED_MASK | FAN_MARK_IGNORED_SURV_MODIFY | FAN_MARK_FLUSH;
