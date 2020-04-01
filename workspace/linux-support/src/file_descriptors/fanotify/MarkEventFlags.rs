// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


bitflags!
{
	/// Event flags to mark.
	pub struct MarkEventFlags: u64
	{
		/// Create an event when a file or directory is accessed (read).
		const Access = FAN_ACCESS;

		/// Create an event when a file is modified (write).
		const Modify = FAN_MODIFY;

		/// Create an event when a writable file is closed.
		const CloseWrite = FAN_CLOSE_WRITE;

		/// Create an event when a read-only file or directory is closed.
		const CloseNoWrite = FAN_CLOSE_NOWRITE;

		/// Create an event when a file or directory is opened.
		const Open = FAN_OPEN;

		/// Create an event when an overflow of the event queue occurs.
		///
		/// The size of the event queue is limited to 16384 entries if `FAN_UNLIMITED_QUEUE` is not set in `fanotify_init()`.
		const EventQueueOverflowed = FAN_Q_OVERFLOW;

		/// Create an event when a permission to open a file or directory is requested.
		///
		/// A fanotify file descriptor created with `use_precontent_class` or `use_content_class` is required.
		const OpenPermission = FAN_OPEN_PERM;

		/// Create an event when a permission to open a file or directory is requested.
		///
		/// A fanotify file descriptor created with `use_precontent_class` or `use_content_class` is required.
		const AccessPermission = FAN_ACCESS_PERM;

		/// Create events for directories.
		///
		/// For example, when `opendir()`, `readdir()` or `closedir()` are called.
		///
		/// Without this flag, only events for files are created.
		const DirectoriesRaiseEvents = FAN_ONDIR;

		/// Events for the immediate children of marked directories shall be created.
		///
		/// The flag has no effect when marking file system mounts.
		/// Note that events are not generated for children of the subdirectories of marked directories.
		/// To monitor complete directory trees it is necessary to mark the relevant mount.
		const ImmediateChildrenAreMarked = FAN_EVENT_ON_CHILD;

		/// A file is closed.
		///
		/// This is a combination of `CloseWrite` and `CloseNoWrite`.
		const Close = FAN_CLOSE;

		/// This is a combination of `Access`, `Modify`, `Close` and `Open`.
		const AllEvents = FAN_ALL_EVENTS;

		/// This is a combination of `OpenPermission` and `AccessPermission`.
		const AllPermissionEvents = FAN_ALL_PERM_EVENTS;

		/// This is a combination of `AllEvents`, `AllPermissionEvents` and `EventQueueOverflowed`.
		const AllOutgoingEvents = FAN_ALL_OUTGOING_EVENTS;
	}
}
