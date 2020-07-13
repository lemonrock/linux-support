// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Maximum number of entries.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct MaximumEntries(pub(crate) NonZeroU32);

impl MaximumEntries
{
	/// New instance.
	#[inline(always)]
	pub const fn new(value: NonZeroU32) -> Self
	{
		Self(value)
	}
	
	#[inline(always)]
	pub(crate) const fn to_u32(self) -> u32
	{
		self.0.get()
	}
}
