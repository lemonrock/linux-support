// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global NUMA memory reclaim configuration error kind.
#[derive(Debug)]
pub enum GlobalNumaMemoryReclaimConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeNumaZoneReclaimMode(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMinimumSlabPercentage(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeMinimumUnmappedPercentage(io::Error),
}

impl Display for GlobalNumaMemoryReclaimConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalNumaMemoryReclaimConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalNumaMemoryReclaimConfigurationError::*;

		match self
		{
			&CouldNotChangeNumaZoneReclaimMode(ref cause) => Some(cause),
			
			&CouldNotChangeMinimumSlabPercentage(ref cause) => Some(cause),
			
			&CouldNotChangeMinimumUnmappedPercentage(ref cause) => Some(cause),
		}
	}
}
