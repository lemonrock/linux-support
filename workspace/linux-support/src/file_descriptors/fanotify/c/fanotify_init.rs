// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[link(name = "c")]
extern "C"
{
	/// `fanotify_init()` initializes a new fanotify group and returns a file descriptor for the event queue associated with the group.
	///
	/// The file descriptor is used in calls to `fanotify_mark()` to specify the files, directories, and mounts for which fanotify events shall be created.
	/// These events are received by reading from the file descriptor.
	/// Some events are only informative, indicating that a file has been accessed.
	/// Other events can be used to determine whether another application is permitted to access a file or directory. Permission to access filesystem objects is granted by writing to the file descriptor.
	///
	/// Multiple programs may be using the fanotify interface at the same time to monitor the same files.
	///
	/// In the current implementation, the number of fanotify groups per user is limited to `128`.
	/// This limit cannot be overridden.
	///
	/// Calling `fanotify_init()` requires the `CAP_SYS_ADMIN` capability.
	/// This constraint might be relaxed in future versions of the API.
	/// Therefore, certain additional capability checks have been implemented.
	///
	/// The `flags` argument contains a multi-bit field defining the notification class of the listening application and further single bit fields specifying the behavior of the file descriptor.
	///
	/// If multiple listeners for permission events exist, the notification class is used to establish the sequence in which the listeners receive the events.
	/// The order of notification for listeners in the same notification class is undefined.
	///
	/// Listeners with different notification classes will receive events in the order `FAN_CLASS_PRE_CONTENT`, `FAN_CLASS_CONTENT` and `FAN_CLASS_NOTIF`.
	///
	/// The `event_f_flags` argument defines the file status flags that will be set on the open file descriptions that are created for fanotify events.
	/// For details of these flags, see the description of the flags values in `man 2 open`.
	/// `event_f_flags` includes a multi-bit field for the access mode combined from `O_RDONLY`, `O_WRONLY`, `O_RDWR`, `O_LARGEFILE`, `O_CLOEXEC`, `O_APPEND`, `O_DSYNC`, `O_NOATIME`, `O_NONBLOCK` and `O_SYNC`.
	///
	/// On success, `fanotify_init()` returns a new file descriptor.
	/// On error, `-1` is returned, and `errno` is set to indicate the error.
	///
	/// Errors documented to be returned from `fanotify_init()` in `errno`:-
	///
	/// * `EINVAL`: An invalid value was passed in `flags` or `event_f_flags`. `FAN_ALL_INIT_FLAGS` defines all allowable bits for `flags`.
	/// * `EMFILE`: The number of fanotify groups for this user exceeds 128.
	/// * `EMFILE`: The per-process limit on the number of open file descriptors has been reached.
	/// * `ENOMEM`: The allocation of memory for the notification group failed.
	/// * `ENOSYS`: This kernel does not implement `fanotify_init()`. The fanotify API is available only if the kernel was configured with `CONFIG_FANOTIFY`.
	/// * `EPERM`: The operation is not permitted because the caller lacks the `CAP_SYS_ADMIN` capability.
	pub(crate) fn fanotify_init(flags: c_uint, event_f_flags: c_uint) -> c_int;
}

/// This value allows the receipt of events notifying that a file has been accessed and events for permission decisions if a file may be accessed.
/// It is intended for event listeners that need to access files before they contain their final data.
/// This notification class might be used by hierarchical storage managers, for example.
pub(crate) const FAN_CLASS_PRE_CONTENT: c_uint = 0x08;

/// This value allows the receipt of events notifying that a file has been accessed and events for permission decisions if a file may be accessed.
/// It is intended for event listeners that need to access files when they already contain their final content.
/// This notification class might be used by malware detection programs, for example.
pub(crate) const FAN_CLASS_CONTENT: c_uint = 0x04;

/// This is the default value.
/// It does not need to be specified.
/// This value only allows the receipt of events notifying that a file has been accessed.
/// Permission decisions before the file is accessed are not possible.
pub(crate) const FAN_CLASS_NOTIF: c_uint = 0x00;

/// This is the values `FAN_CLASS_PRE_CONTENT`, `FAN_CLASS_CONTENT` and `FAN_CLASS_NOTIF`.
#[allow(dead_code)]
pub(crate) const FAN_ALL_CLASS_BITS: c_uint = FAN_CLASS_PRE_CONTENT | FAN_CLASS_CONTENT | FAN_CLASS_NOTIF;

/// Set the close-on-exec flag (`FD_CLOEXEC`) on the new file descriptor.
pub(crate) const FAN_CLOEXEC: c_uint = 0x01;

/// Enable the nonblocking flag (`O_NONBLOCK`) for the file descriptor.
pub(crate) const FAN_NONBLOCK: c_uint = 0x02;

/// Remove the limit of `16384` events for the event queue.
/// Use of this flag requires the `CAP_SYS_ADMIN` capability.
pub(crate) const FAN_UNLIMITED_QUEUE: c_uint = 0x10;

/// Remove the limit of `8192` marks.
/// Use of this flag requires the `CAP_SYS_ADMIN` capability.
pub(crate) const FAN_UNLIMITED_MARKS: c_uint = 0x20;

/// This is the values `FAN_CLOEXEC`, `FAN_NONBLOCK`, `FAN_ALL_CLASS_BITS`, `FAN_UNLIMITED_QUEUE` and `FAN_UNLIMITED_MARKS`.
#[allow(dead_code)]
pub(crate) const FAN_ALL_INIT_FLAGS: c_uint = FAN_CLOEXEC | FAN_NONBLOCK | FAN_ALL_CLASS_BITS | FAN_UNLIMITED_QUEUE | FAN_UNLIMITED_MARKS;
