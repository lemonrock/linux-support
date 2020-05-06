// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// IO Priority (ioprio or ionice) process error.
#[derive(Debug)]
pub enum ProcessIoPriorityConfigurationError
{
	/// Could not set current real user io priority (permission was denied in some way).
	CouldNotSetCurrentUserIoPriority(bool),

	/// Could not set current process group user io priority (permission was denied in some way).
	CouldNotSetCurrentProcessGroupIoPriority(bool),

	/// Could not set current process user io priority (permission was denied in some way).
	CouldNotSetCurrentProcessIoPriority(bool),
}

impl Display for ProcessIoPriorityConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcessIoPriorityConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessIoPriorityConfigurationError::*;

		match self
		{
			&CouldNotSetCurrentUserIoPriority(..) => None,

			&CouldNotSetCurrentProcessGroupIoPriority(..) => None,

			&CouldNotSetCurrentProcessIoPriority(..) => None,
		}
	}
}
