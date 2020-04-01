// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Reason given for why a path is an invalid for a FIFO.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InvalidPathReason
{
	/// Equivalent to `ELOOP`.
	TooManySymbolicLinks,

	/// Equivalent to `ENAMETOOLONG`.
	TooLong,

	/// Equivalent to `EISDIR`.
	IsADirectory,

	/// Does not exist.
	///
	/// A component of the FIFO path does not exist.
	DoesNotExist,

	/// One of the parent components of the path (ie not the file name) is not a directory.
	ParentComponentIsNotADirectory,

	/// The path points to something that exists but:-
	///
	/// * it is not a device special file (character or block device) backed by a device, or;
	/// * it is on a read-only file system and writes need to occur (this should not normally occur), or;
	/// * it is an executable image currently being executed and writes need to occur.
	ExistsButCanNotBeUsed,
}
