// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Life time.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LifeTime
{
	/// Infinite.
	Infinite,
	
	/// Finite.
	Finite(LifeTimeMicroseconds),
}

impl From<u32> for LifeTime
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		use self::LifeTime::*;
		
		if value == ifa_cacheinfo::INFINITY_LIFE_TIME
		{
			Infinite
		}
		else
		{
			Finite(LifeTimeMicroseconds::from_unchecked(value))
		}
	}
}
