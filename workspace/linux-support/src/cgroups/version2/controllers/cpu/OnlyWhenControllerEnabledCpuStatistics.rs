// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// CPU statistics.
///
/// Available only when the `cpu` controller is enabled.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OnlyWhenControllerEnabledCpuStatistics
{
	#[allow(missing_docs)]
	pub number_of_periods: usize,
	
	#[allow(missing_docs)]
	pub number_throttled: usize,
	
	#[allow(missing_docs)]
	pub throttled_time: Microseconds,
}
