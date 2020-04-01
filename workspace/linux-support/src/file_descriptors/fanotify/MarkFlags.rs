// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


bitflags!
{
	/// Flags control the marking of a path.
	pub struct MarkFlags: u32
	{
		/// If pathname is a symbolic link, mark the link itself, rather than the file to which it refers.
		///
		/// (By default, `fanotify_mark()` dereferences pathname if it is a symbolic link).
		const DoNotFollow = FAN_MARK_DONT_FOLLOW;

		/// If the filesystem object to be marked is not a directory, the error `ENOTDIR` shall be raised.
		const OnlyDirectory = FAN_MARK_ONLYDIR;

		/// Mark the mount point specified by pathname.
		///
		/// If `pathname` is not itself a mount point, the mount point containing `pathname` will be marked.
		/// All directories, subdirectories, and the contained files of the mount point will be monitored.
		const Mount = FAN_MARK_MOUNT;

		/// The events in mask shall be added to or removed from the ignore mask.
		const IgnoredMask = FAN_MARK_IGNORED_MASK;

		/// The ignore mask shall survive modify events.
		///
		/// If this flag is not set, the ignore mask is cleared when a modify event occurs for the ignored file or directory.
		const IgnoredSurviveModify = FAN_MARK_IGNORED_SURV_MODIFY;
	}
}
