// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur when opening one end of a FIFO (a named pipe) or a character device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpecialFileOpenError
{
	/// Errors common to opening or creation of most file descriptors.
	Common(CreationError),

	/// A (possibly mandatory) file lock is held on the special file path.
	///
	/// Rationally, this would not seem to make sense but the Linux documentation doesn't make it clear if it is possible or not.
	///
	/// If this is encountered then an orderly shutdown is probably the only course of action as it is not possible to epoll for lock status changes on files that haven't even be opened.
	WouldBlock,

	/// `EINTR` occurred; this can be handled by either re-trying the open of a FIFO or might actual be fatal depending on the signal handling strategy in use.
	Interrupted,

	/// Invalid path.
	InvalidPath(InvalidPathReason),

	/// Not a terminal.
	Terminal(TerminalSettingsError),
}

impl Display for SpecialFileOpenError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<SpecialFileOpenError as Debug>::fmt(self, f)
	}
}

impl error::Error for SpecialFileOpenError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::SpecialFileOpenError::*;

		match self
		{
			&Common(ref error) => Some(error),

			WouldBlock => None,

			Interrupted => None,

			&InvalidPath(_) => None,

			Terminal(ref error) => Some(error),
		}
	}
}
