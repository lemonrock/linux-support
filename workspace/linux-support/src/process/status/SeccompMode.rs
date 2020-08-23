// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `seccomp` mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum SeccompMode
{
	/// Off.
	Off = 0,

	/// Strict.
	Strict = 1,

	/// Filter.
	Filter = 2,
}

impl FromBytes for SeccompMode
{
	type Error = StatusStatisticParseError;

	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		if likely!(value.len() == 1)
		{
			use self::SeccompMode::*;

			match value[0]
			{
				b'0' => Ok(Off),
				b'1' => Ok(Strict),
				b'2' => Ok(Filter),
				_ => Err(StatusStatisticParseError::OutOfRange)
			}
		}
		else
		{
			Err(StatusStatisticParseError::InvalidLength)
		}
	}
}
