// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetExistingMapError
{
	#[allow(missing_docs)]
	CouldNotGetExistingMapFileDescriptor(Errno),
	
	#[allow(missing_docs)]
	CouldNotGetExistingMapInformation(Errno),
}

impl Display for GetExistingMapError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GetExistingMapError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::GetExistingMapError::*;
		
		match self
		{
			&CouldNotGetExistingMapFileDescriptor(_) => None,
			
			&CouldNotGetExistingMapInformation(_) => None,
		}
	}
}
