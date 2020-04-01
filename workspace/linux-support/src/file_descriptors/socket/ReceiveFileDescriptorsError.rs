// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during receive of file descriptors.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReceiveFileDescriptorsError
{
	/// A read error that might be possible to handle.
	Read(StructReadError),

	/// More than one ancillary message header was received.
	MoreThanOneHeader,

	/// A received ancillary message was not for `SOL_SOCKET`.
	WasNotSocketLevelPosixMessage,

	/// A received ancillary message was not for `SCM_RIGHTS`.
	WasNotScmRights,
}

impl Display for ReceiveFileDescriptorsError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<ReceiveFileDescriptorsError as Debug>::fmt(self, f)
	}
}

impl error::Error for ReceiveFileDescriptorsError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ReceiveFileDescriptorsError::*;

		match self
		{
			&Read(ref error) => Some(error),

			&MoreThanOneHeader => None,

			&WasNotSocketLevelPosixMessage => None,

			&WasNotScmRights => None,
		}
	}
}
