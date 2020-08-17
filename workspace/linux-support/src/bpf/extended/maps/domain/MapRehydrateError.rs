// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A map rehydrate error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapRehydrateError
{
	#[allow(missing_docs)]
	CouldNotGetExistingMapInformation(GetExistingMapError),
	
	#[allow(missing_docs)]
	CouldNotGetExistingMapNamed(MapName),
	
	#[allow(missing_docs)]
	NoMaximumEntries,
}

impl Display for MapRehydrateError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MapRehydrateError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::MapRehydrateError::*;
		
		match self
		{
			&CouldNotGetExistingMapInformation(_) => None,
			
			&CouldNotGetExistingMapNamed(_) => None,
			
			&NoMaximumEntries => None,
		}
	}
}

impl From<GetExistingMapError> for MapRehydrateError
{
	#[inline(always)]
	fn from(value: GetExistingMapError) -> Self
	{
		MapRehydrateError::CouldNotGetExistingMapInformation(value)
	}
}
