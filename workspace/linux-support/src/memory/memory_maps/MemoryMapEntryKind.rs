// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// What kind of entry is this?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryMapEntryKind
{
	/// A mapped file.
	File
	{
		/// Offset in bytes into the file.
		offset: u32,

		/// Block device.
		block_device: BlockDevice,

		/// Inode.
		inode: Inode,

		/// Not unambigious, because Linux escapes the path badly:-
		///
		/// * Using an octal escape sequence of four characters for newline of `\012` but nothing for `\` (see `mangle_path()` in `seq_file.c` in the Linux sources).
		/// * By appending ` (deleted)` for deleted files (so file paths that genuinely end in ` (deleted)` are mistakenly interpreted.
		file_path: PathBuf,

		/// Is the associated file believed to have been deleted?
		deleted: bool,

		/// Only `Some` if the kernel has been built with `CONFIG_NUMA`.
		page_counts: Option<PageCounts>,
	},

	/// An anonymous mapping.
	Anonymous
	{
		/// Only `Some` if the kernel has been built with `CONFIG_NUMA`.
		page_counts: Option<PageCounts>,
	},

	/// Special
	Special(MemoryMapEntryKindSpecial),
}
