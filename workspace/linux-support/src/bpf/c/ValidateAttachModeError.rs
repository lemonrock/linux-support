// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidateAttachModeError
{
	#[allow(missing_docs)]
	ExistingExpressDataPathProgramShouldBeOffloaded,
	
	#[allow(missing_docs)]
	ExistingExpressDataPathProgramShouldNotBeOffloaded,
	
	#[allow(missing_docs)]
	ExistingExpressDataPathProgramIsOffloadedForADifferentNetworkInterfaceIndex,
	
	#[allow(missing_docs)]
	AttachModeAndDeviceOffloadMismatch,
	
	#[allow(missing_docs)]
	DeviceOffloadRequiredButNotOffloaded,
}

impl Display for ValidateAttachModeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ValidateAttachModeError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::ValidateAttachModeError::*;
		
		match self
		{
			&ExistingExpressDataPathProgramShouldBeOffloaded => None,
			
			&ExistingExpressDataPathProgramShouldNotBeOffloaded => None,
			
			&ExistingExpressDataPathProgramIsOffloadedForADifferentNetworkInterfaceIndex => None,
			
			&AttachModeAndDeviceOffloadMismatch => None,
			
			&DeviceOffloadRequiredButNotOffloaded => None,
		}
	}
}
