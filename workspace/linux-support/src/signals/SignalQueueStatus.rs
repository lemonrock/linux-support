// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Signal queue status.
#[derive(Default, Debug, Copy,Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalQueueStatus
{
	/// Number of signals queued.
	pub number_of_signals_queued: u64,

	/// Maximum number of signals that can be queued (maximum queue depth); see the description of `RLIMIT_SIGPENDING` in the manpage for `getrlimit`.
	pub maximum_number_of_signals_that_can_be_queued: u64,
}

impl FromBytes for SignalQueueStatus
{
	type Error = StatusStatisticParseError;

	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		// number of signals queued/max. number for queue
		let mut iterator = value.split_bytes_n(2, b'/');

		let number_of_signals_queued = u64::parse_decimal_number(iterator.next().unwrap())?;

		let maximum_number_of_signals_that_can_be_queued = match iterator.next()
		{
			None => return Err(StatusStatisticParseError::InvalidSeparator),

			Some(maximum_number_of_signals_that_can_be_queued) => u64::parse_decimal_number(maximum_number_of_signals_that_can_be_queued)?,
		};

		Ok
		(
			Self
			{
				number_of_signals_queued,
				maximum_number_of_signals_that_can_be_queued
			}
		)
	}
}
