// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A signature timestamp wraps approximately every 136 years; the next wrap does not occur until Sunday, February 7, 2106 6:28:15 AM GMT.
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Hash)]
pub struct SignatureTimestamp(SerialNumber);

impl SignatureTimestamp
{
	/// Assumes expiration exceeds inception, otherwise `Some(None)` is returned.
	/// If the difference is too large (more than 68 years or so) `None` is returned.
	#[inline(always)]
	pub fn inception_and_expiration(self, inception: Self) -> Option<Option<(NanosecondsSinceUnixEpoch, NanosecondsSinceUnixEpoch)>>
	{
		let expiration = self.0;
		let inception = inception.0;
		match expiration.difference(&inception)
		{
			None => None,
			
			Some((signature_expiration_seconds, signature_inception_seconds, difference)) =>  if unlikely!(difference <= 0)
			{
				Some(None)
			}
			else
			{
				let signature_inception_time = if unlikely!(signature_inception_seconds > signature_expiration_seconds)
				{
					NanosecondsSinceUnixEpoch::from_seconds_u32_after_next_wrap_around(signature_inception_seconds)
				}
				else
				{
					NanosecondsSinceUnixEpoch::from_seconds_u32_before_next_wrap_around(signature_inception_seconds)
				};
				let signature_expiration_time = NanosecondsSinceUnixEpoch::from_seconds_u32_before_next_wrap_around(signature_expiration_seconds);
				Some(Some(signature_inception_time, signature_expiration_time))
			}
		}
	}
}
