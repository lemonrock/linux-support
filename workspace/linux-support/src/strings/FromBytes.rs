// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// From bytes.
pub trait FromBytes: Sized
{
	/// Error.
	type Error: error::Error;

	/// From bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;
}

impl<P: ParseNumber> FromBytes for P
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		Self::parse_decimal_number(bytes)
	}
}
