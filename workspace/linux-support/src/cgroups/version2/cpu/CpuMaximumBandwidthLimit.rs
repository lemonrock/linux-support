// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Only for the Completely Fair Scheduler (CFS).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CpuMaximumBandwidthLimit
{
	/// Quota.
	pub quota: MaximumNumber<Microseconds>,
	
	/// Period.
	pub period: Microseconds,
}

impl Default for CpuMaximumBandwidthLimit
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			quota: MaximumNumber::default(),
			period: Microseconds(100_000)
		}
	}
}

impl FromBytes for CpuMaximumBandwidthLimit
{
	/// Error.
	type Error = ParseNumberError;
	
	/// From bytes.
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		let mut iterator = bytes.split_bytes_n(2, b' ');
		
		Ok
		(
			Self
			{
				quota: MaximumNumber::from_bytes(iterator.next().unwrap())?,
				period: Microseconds::from_bytes(iterator.next().ok_or(ParseNumberError::TooShort)?)?,
			}
		)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for CpuMaximumBandwidthLimit
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		use self::MaximumNumber::*;
		
		let string = match self.quota
		{
			Finite(value) => format!("{:?} {:?}\n", value.0, self.period.0),
			Maximum => format!("max {:?}\n", self.period.0),
		};
		
		Cow::from(string)
	}
}
