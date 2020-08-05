// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global out-of-memory configuration error kind.
#[derive(Debug)]
pub enum GlobalOutOfMemoryConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangePanicOnOutOfMemory(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeKillTaskThatCausedOutOfMemory(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeNumaZoneReclaimMode(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMemoryOverCommitPolicy(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMemoryOverCommitReservedKilobytesForAdministratorUsers(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMemoryOverCommitReservedKilobytesForNormalUsers(io::Error),
}

impl Display for GlobalOutOfMemoryConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalOutOfMemoryConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalOutOfMemoryConfigurationError::*;

		match self
		{
			&CouldNotChangePanicOnOutOfMemory(ref cause) => Some(cause),
			
			&CouldNotChangeKillTaskThatCausedOutOfMemory(ref cause) => Some(cause),
			
			&CouldNotChangeNumaZoneReclaimMode(ref cause) => Some(cause),
			
			&CouldNotChangeMemoryOverCommitPolicy(ref cause) => Some(cause),
			
			&CouldNotChangeMemoryOverCommitReservedKilobytesForAdministratorUsers(ref cause) => Some(cause),
			
			&CouldNotChangeMemoryOverCommitReservedKilobytesForNormalUsers(ref cause) => Some(cause),
		}
	}
}
