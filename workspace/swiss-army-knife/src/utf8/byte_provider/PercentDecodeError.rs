// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PercentDecodeError
{
	#[allow(missing_docs)]
	MissingPercentSign
	{
		decoded_byte_number: u8,
	
		invalid: u8,
	},
	
	#[allow(missing_docs)]
	InvalidHexDigit
	{
		decoded_byte_number: u8,
		
		relative_index: u8,
		
		invalid: u8,
	},
}

impl Display for PercentDecodeError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for PercentDecodeError
{
}
