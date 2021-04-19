// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread configuration error kind.
#[derive(Debug)]
pub enum MainThreadConfigurationError
{
	#[allow(missing_docs)]
	CouldNotSetThreadName(io::Error),
	
	#[allow(missing_docs)]
	CouldNotConfigure(ThreadConfigurationError),
}

impl Display for MainThreadConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MainThreadConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::MainThreadConfigurationError::*;

		match self
		{
			&CouldNotSetThreadName(ref cause) => Some(cause),

			&CouldNotConfigure(ref cause) => Some(cause),
		}
	}
}

impl From<ThreadConfigurationError> for MainThreadConfigurationError
{
	#[inline(always)]
	fn from(cause: ThreadConfigurationError) -> Self
	{
		MainThreadConfigurationError::CouldNotConfigure(cause)
	}
}
