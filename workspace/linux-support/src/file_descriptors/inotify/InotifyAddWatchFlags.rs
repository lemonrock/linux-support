// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


bitflags!
{
	/// inotify add watch flags.
	pub struct InotifyAddWatchFlags: u32
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

		/// This is all events but not all possible valid flag values.
		///
		/// This is a combination of `Access`, `Modify`, `Attributes`, `CloseWrite`, `CloseNoWrite`, `Open`, `MovedFrom`, `MovedTo`, `Create`, `Delete`, `DeleteSelf` and `MoveSelf`.
		const AllEvents = IN_ALL_EVENTS;

		/// This is equivalent to `MovedFrom` and `MovedTo`.
		///
		/// Valid for `inotify_add_watch()`'s `mask` argument only.
		const Move = IN_MOVE;

		/// This is equivalent to `CloseWrite` and `CloseNoWrite`.
		///
		/// Valid for `inotify_add_watch()`'s `mask` argument only.
		const Close = IN_CLOSE;

		/// Only watch `pathname` if it is a directory.
		///
		/// Since Linux 2.6.15.
		const OnlyDirectory = IN_ONLYDIR;

		/// Don't dereference `pathname` if it is a symbolic link.
		///
		/// Since Linux 2.6.15.
		const DoNotDereferenceASymbolicLink = IN_DONT_FOLLOW;

		/// By default, when watching events on the children of a directory, events are generated for children even after they have been unlinked from the directory.
		///
		/// This can result in large numbers of uninteresting events for some applications (eg, if watching `/tmp`, in which many applications create temporary files whose names are immediately unlinked).
		/// Specifying `IN_EXCL_UNLINK` changes the default behavior, so that events are not generated for children after they have been unlinked from the watched directory.
		///
		/// Since Linux 2.6.36.
		const ExclusiveUnlink = IN_EXCL_UNLINK;

		/// Monitor `pathname` for one event, then remove from watch list.
		const OneShot = IN_ONESHOT;
	}
}

impl InotifyAddWatchFlags
{
	#[inline(always)]
	pub(crate) fn set_bitmask(self) -> u32
	{
		self.bits
	}

	#[inline(always)]
	pub(crate) fn add_bitmask(self) -> u32
	{
		self.bits | IN_MASK_ADD
	}
}
