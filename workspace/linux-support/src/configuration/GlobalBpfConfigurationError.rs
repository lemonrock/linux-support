// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global BPF configuration error kind.
#[derive(Debug)]
pub enum GlobalBpfConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeJustInTimeCompilation(io::Error),
	
	#[allow(missing_docs)]
	CouldNotChangeJustInTimeCompilationMemoryLimit(io::Error),
}

impl Display for GlobalBpfConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalBpfConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalBpfConfigurationError::*;

		match self
		{
			&CouldNotChangeJustInTimeCompilation(ref cause) => Some(cause),
			
			&CouldNotChangeJustInTimeCompilationMemoryLimit(ref cause) => Some(cause),
		}
	}
}
