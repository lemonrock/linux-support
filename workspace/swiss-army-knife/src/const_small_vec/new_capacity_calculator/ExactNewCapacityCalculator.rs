// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Exact growth.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ExactNewCapacityCalculator;

impl const NewCapacityCalculator for ExactNewCapacityCalculator
{
	#[inline(always)]
	fn calculate<T>(_current_capacity: usize, required_capacity: usize) -> Result<usize, TryReserveError>
	{
		Ok(required_capacity)
	}
}
