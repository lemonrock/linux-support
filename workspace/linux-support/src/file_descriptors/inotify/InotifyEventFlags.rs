// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


bitflags!
{
	/// inotify event flags.
	pub struct InotifyEventFlags: u32
	{
		/// File was accessed (read).
		///
		/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
		const Access = IN_ACCESS;

		/// File was modified.
		///
		/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
		const Modify = IN_MODIFY;

		/// Metadata changed.
		///
		/// For example, permissions, timestamps, extended attributes, UID, GID, etc.
		/// Since Linux 2.6.25, this also includes the link count.
		///
		/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
		const Attributes = IN_ATTRIB;

		/// File opened for writing was closed.
		///
		/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
		const CloseWrite = IN_CLOSE_WRITE;

		/// File not opened for writing was closed.
		///
		/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
		const CloseNoWrite = IN_CLOSE_NOWRITE;

		/// File was opened.
		///
		/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
		const Open = IN_OPEN;

		/// File moved out of watched directory.
		///
		/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
		const MovedFrom = IN_MOVED_FROM;

		/// File moved into watched directory.
		///
		/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
		const MovedTo = IN_MOVED_TO;

		/// File/directory created in watched directory.
		///
		/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
		const Create = IN_CREATE;

		/// File/directory deleted from watched directory.
		///
		/// When monitoring a directory, this event can occur for files in the directory, in which case the `name` field in the returned `inotify_event` from `read()` structure identifies the name of the file within the directory.
		const Delete = IN_DELETE;

		/// Watched file/directory was itself deleted.
		const DeleteSelf = IN_DELETE_SELF;

		/// Watched file/directory was itself moved.
		const MoveSelf = IN_MOVE_SELF;

		/// File system containing watched object was unmounted.
		const Unmount = IN_UNMOUNT;

		/// Event queue overflowed.
		///
		/// The watch descriptor is `-1` for this event.
		const EventQueueOverflowed = IN_Q_OVERFLOW;

		/// Watch was removed.
		///
		/// This can have been done explicitly using `inotify_rm_watch()` or automatically because the file was deleted, or its file system was unmounted.
		const Ignored = IN_IGNORED;

		/// Subject of this event is a directory.
		const IsADirectory = IN_ISDIR;
	}
}
