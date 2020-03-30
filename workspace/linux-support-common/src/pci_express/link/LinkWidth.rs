// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Link Width, also known as Number of Lanes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LinkWidth
{
	/// 1 lane.
	x1,

	/// 2 lanes.
	x2,

	/// 4 lanes.
	x4,

	/// 8 lanes.
	x8,

	/// 16 lanes.
	x16,

	/// 32 lanes.
	///
	/// Very rare.
	x32,
}

impl FromBytes for LinkWidth
{
	type Error = ParseLinkWidthError;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		use self::LinkWidth::*;

		match bytes
		{
			b"1" => Ok(x1),
			b"2" => Ok(x2),
			b"4" => Ok(x4),
			b"8" => Ok(x8),
			b"16" => Ok(x16),
			b"32" => Ok(x32),

			_ => Err(ParseLinkWidthError::Unrecognised(bytes.to_vec()))
		}
	}
}
