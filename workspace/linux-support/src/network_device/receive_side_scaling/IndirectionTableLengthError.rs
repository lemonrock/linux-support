// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IndirectionTableLengthError
{
	/// Should not happen.
	IndirectionTableIsTooLongForU32(TryFromIntError),
	
	/// Should not happen.
	IndirectionTableLengthIsZero,
}

impl Display for IndirectionTableLengthError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for IndirectionTableLengthError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::IndirectionTableLengthError::*;
		
		match self
		{
			&IndirectionTableIsTooLongForU32(ref error) => Some(error),
			
			&IndirectionTableLengthIsZero => None,
		}
	}
}
