// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global Transparent Huge Pages (THP) configuration error.
#[derive(Debug)]
pub enum GlobalTransparentHugePagesConfigurationError
{
	/// Could not disable defragmentation.
	Defragmentation(io::Error),

	/// Could not disable usage.
	Usage(io::Error),
}

impl Display for GlobalTransparentHugePagesConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalTransparentHugePagesConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalTransparentHugePagesConfigurationError::*;

		match self
		{
			&Defragmentation(ref error) => Some(error),

			&Usage(ref error) => Some(error),
		}
	}
}
