// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Exponential growth.
///
/// Logic is based on Rust's `RawVec::grow_amortized()` function, which is currently used by `Vec`.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ExponentiallyGrowingNewCapacityCalculator;

impl NewCapacityCalculator for ExponentiallyGrowingNewCapacityCalculator
{
	#[inline(always)]
	fn calculate<T>(current_capacity: usize, required_capacity: usize) -> Result<usize, TryReserveError>
	{
		// This constant is duplicated from Rust's RawVec, and attempts to optimize for a memory allocator's algorithm without overly wasting space.
		let MIN_NON_ZERO_CAP: usize = if size_of::<T>() == 1
		{
			8
		}
		else if size_of::<T>() <= 1024
		{
			4
		}
		else
		{
			1
		};
		
		Ok(max(MIN_NON_ZERO_CAP, max(current_capacity * 2, required_capacity)))
	}
}
