// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Priority.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventPriority
{
	/// `normal`.
	///
	/// The default.
	Normal,
	
	/// `low`.
	Low,
}

impl Default for EventPriority
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Default
	}
}

impl EventPriority
{
	const Default: Self = EventPriority::Normal;
	
	/// Is default.
	#[inline(always)]
	pub fn is_default(self) -> bool
	{
		self == Self::Default
	}
	
	/// Is not default.
	#[inline(always)]
	pub fn is_not_default(self) -> bool
	{
		self != Self::Default
	}
	
	#[inline(always)]
	fn to_bytes(self) -> &'static [u8]
	{
		use self::EventPriority::*;
		
		match self
		{
			Normal => b"normal",
			
			Low => b"low",
		}
	}
}
