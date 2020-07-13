// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Value size.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ValueSizeU16(NonZeroU16);

impl ValueSizeU16
{
	const PAGE_SIZE: NonZeroU16 = Self::new_unsafe((1 << ValueSizeU32::PAGE_SHIFT) as u16);
	
	#[inline(always)]
	const fn new_unsafe(value: u16) -> Self
	{
		Self(unsafe { NonZeroU16::new_unchecked(value) })
	}
	
	#[inline(always)]
	pub(crate) const fn to_non_zero_u32(self) -> NonZeroU32
	{
		unsafe { NonZeroU32::new_unchecked(self.0.get() as u32) }
	}
}
