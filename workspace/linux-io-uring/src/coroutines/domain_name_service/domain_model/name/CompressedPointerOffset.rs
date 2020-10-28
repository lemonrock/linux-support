// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompressedPointerOffset(u16);

impl TryFrom<usize> for CompressedPointerOffset
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		if value >= (Self::ExclusiveMaximum as usize)
		{
			Err(())
		}
		else
		{
			Ok(Self(value as u16))
		}
	}
}

impl CompressedPointerOffset
{
	const ExclusiveMaximum: u16 = 1 << 14;
	
	#[inline(always)]
	const fn start_of_name_pointer(self, start_of_message_pointer: usize) -> usize
	{
		start_of_message_pointer + (self.0 as usize)
	}
}
