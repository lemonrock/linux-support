// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FileDescriptorLabelsMapError
{
	/// Already added file descriptor label.
	AlreadyAddedFileDescriptorLabel,
	
	/// Missing file descriptor label.
	MissingFileDescriptorLabel,
}

impl Display for FileDescriptorLabelsMapError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for FileDescriptorLabelsMapError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::FileDescriptorLabelsMapError::*;
		
		match self
		{
			AlreadyAddedFileDescriptorLabel => None,
			
			MissingFileDescriptorLabel => None,
		}
	}
}
