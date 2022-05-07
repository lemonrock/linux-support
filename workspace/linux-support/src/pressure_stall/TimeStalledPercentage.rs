// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Time stalled percentage.
///
/// TODO: Same structure for loadavg_proc_show
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TimeStalledPercentage
{
	/// Integer.
	pub integer: usize,
	
	/// Two decimal points.
	pub fraction_two_decimal_points: u8,
}

impl FromBytes for TimeStalledPercentage
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		use self::ParseNumberError::*;
		
		let index = bytes.memchr(b'.').ok_or(TooShort)?;
		
		Ok
		(
			Self
			{
				integer: usize::parse_decimal_number(&bytes[.. index])?,
				
				fraction_two_decimal_points:
				{
					
					let fraction = &bytes[(index + 1) ..];
					if unlikely!(fraction.len() != 2)
					{
						return Err(TooShort)
					}
					u8::parse_decimal_number(fraction)?
				}
			}
		)
	}
}
