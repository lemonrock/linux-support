// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A conversion error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BitSetAwareTryFromU16Error
{
	/// Value out of range.
	ValueOutOfRange,
}

impl Default for BitSetAwareTryFromU16Error
{
	#[inline(always)]
	fn default() -> Self
	{
		BitSetAwareTryFromU16Error::ValueOutOfRange
	}
}

impl Display for BitSetAwareTryFromU16Error
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<BitSetAwareTryFromU16Error as Debug>::fmt(self, f)
	}
}

impl error::Error for BitSetAwareTryFromU16Error
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::BitSetAwareTryFromU16Error::*;

		match self
		{
			&ValueOutOfRange => None,
		}
	}
}
