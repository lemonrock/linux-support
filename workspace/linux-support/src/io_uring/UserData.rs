// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Simple trait for user data.
pub trait UserData
{
	/// Convert.
	fn into_u64(self) -> u64;
	
	/// Convert.
	fn from_u64(value: u64) -> Self;
}

impl UserData for u64
{
	#[inline(always)]
	fn into_u64(self) -> u64
	{
		self
	}
	
	#[inline(always)]
	fn from_u64(value: u64) -> Self
	{
		value
	}
}
