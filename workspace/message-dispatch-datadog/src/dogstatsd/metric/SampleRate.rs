// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A value restricted to between `0` and `1` inclusive representing a fraction of sample time.
///
/// Only for `COUNT`, `HISTOGRAM` and `TIMER` metrics.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SampleRate(f64);

impl Default for SampleRate
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::OneHundredPercent
	}
}

impl SampleRate
{
	/// 100%.
	pub const OneHundredPercent: Self = Self(1.0);
}
