// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global Linux Kernel Same Page Merging (KSM) configuration error kind.
#[derive(Debug)]
pub enum GlobalLinuxKernelSamePageMergingConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeEnablement(io::Error),
}

impl Display for GlobalLinuxKernelSamePageMergingConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalLinuxKernelSamePageMergingConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalLinuxKernelSamePageMergingConfigurationError::*;
		
		match self
		{
			&CouldNotChangeEnablement(ref cause) => Some(cause),
		}
	}
}
