// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Rename flags.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum RenameFlags
{
	/// Just do a simple rename (move).
	JustRename = 0,

	/// Atomically exchanges name with another one; both must exist.
	///
	/// Equivalent to `RENAME_EXCHANGE`.
	AtomicallySwapName = RENAME_EXCHANGE as i32,

	/// Don't overwrite `to_path` of the rename.
	///
	/// Return an error if `to_path` already exists.
	///
	/// Equivalent to `RENAME_NOREPLACE`.
	NoReplace = RENAME_NOREPLACE as i32,

	/// Requires the process to have the `CAP_MKNOD` capability.
	///
	/// This operation makes sense only for overlay/union filesystem domain, and requires file system support from the underlying filesystem.
	///
	/// Specifying this creates a "whiteout" object at the source of the rename at the same time as performing the rename.
	/// The whole operation is atomic, so that if the rename succeeds then the whiteout will also have been created.
	///
	/// A "whiteout" is an object that has special meaning in union/overlay filesystem constructs.
	/// In these constructs, multiple layers exist and only the top one is ever modified.
	/// A whiteout on an upper layer will effectively hide a matching file in the lower layer, making it appear as if the file didn't exist.
	///
	/// When a file that exists on the lower layer is renamed, the file is first copied up (if not already on the upper layer) and then renamed on the upper, read-write layer.
	/// At the same time, the source file needs to be "whiteouted" (so that the version of the source file in the lower layer is rendered invisible).  The whole operation needs to be done atomically.
	///
	/// When not part of a union/overlay, the whiteout appears as a character device with a {0,0} device number.
	/// (Note that other union/overlay domain may employ different methods for storing whiteout entries; specifically, BSD union mount  employs a separate inode type, `DT_WHT`, which, while supported by some filesystems available in Linux, such as CODA and XFS, is ignored by the kernel's whiteout support code, as of Linux 4.19, at least).
	///
	/// Equivalent to `RENAME_WHITEOUT`.
	Whiteout = RENAME_WHITEOUT as i32,

	/// Equivalent to `RENAME_NOREPLACE | RENAME_WHITEOUT`.
	NoReplaceAndWhiteout = (RENAME_NOREPLACE | RENAME_WHITEOUT) as i32,
}
