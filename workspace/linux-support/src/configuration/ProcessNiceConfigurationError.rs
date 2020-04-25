// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process niceness error.
#[derive(Debug)]
pub enum ProcessNiceConfigurationError
{
	/// Could not set current real effective user priority niceness (permission was denied in some way).
	CouldNotSetCurrentUserPriorityNice,

	/// Could not set current process group user priority niceness (permission was denied in some way).
	CouldNotSetCurrentProcessGroupPriorityNice,

	/// Could not set current process user priority niceness (permission was denied in some way).
	CouldNotSetCurrentProcessPriorityNice,

	/// Could not set current process user autogroup priority niceness.
	CouldNotSetCurrentProcessAutogroupPriorityNice(io::Error),
}

impl Display for ProcessNiceConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcessNiceConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessNiceConfigurationError::*;

		match self
		{
			&CouldNotSetCurrentUserPriorityNice => None,

			&CouldNotSetCurrentProcessGroupPriorityNice => None,

			&CouldNotSetCurrentProcessPriorityNice => None,

			&CouldNotSetCurrentProcessAutogroupPriorityNice(ref error) => Some(error),
		}
	}
}

impl From<io::Error> for ProcessNiceConfigurationError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		ProcessNiceConfigurationError::CouldNotSetCurrentProcessAutogroupPriorityNice(error)
	}
}
