// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread configuration error kind.
#[derive(Debug)]
pub enum SpawnedThreadError
{
	#[allow(missing_docs)]
	CouldNotSpawnThread(io::Error),
	
	#[allow(missing_docs)]
	CouldNotConfigure(ThreadConfigurationError),
}

impl Display for SpawnedThreadError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for SpawnedThreadError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::SpawnedThreadError::*;

		match self
		{
			&CouldNotSpawnThread(ref cause) => Some(cause),

			&CouldNotConfigure(ref cause) => Some(cause),
		}
	}
}

impl From<ThreadConfigurationError> for SpawnedThreadError
{
	#[inline(always)]
	fn from(cause: ThreadConfigurationError) -> Self
	{
		SpawnedThreadError::CouldNotConfigure(cause)
	}
}
