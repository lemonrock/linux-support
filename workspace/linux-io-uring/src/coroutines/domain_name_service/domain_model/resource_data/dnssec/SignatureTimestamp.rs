// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A signature timestamp wraps approximately every 136 years; the next wrap does not occur until Sunday, February 7, 2106 6:28:15 AM GMT.
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Hash)]
pub struct SignatureTimestamp(SerialNumber);

impl Into<Timespec> for SignatureTimestamp
{
	#[inline(always)]
	fn into(self) -> Timespec
	{
		let seconds: u32 = self.0.into();
		Timespec::new(seconds as i64, 0)
	}
}

impl SignatureTimestamp
{
	/// Difference.
	#[inline(always)]
	pub fn difference(&self, other: &Self) -> Option<(u32, u32, i32)>
	{
		self.0.difference(&other.0)
	}
}
