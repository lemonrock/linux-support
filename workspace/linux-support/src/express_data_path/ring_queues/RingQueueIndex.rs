// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct RingQueueIndex(u32);

impl Add<u32> for RingQueueIndex
{
	type Output = RingQueueEntryIndex;
	
	#[inline(always)]
	fn add(self, relative_frame_index: u32) -> Self::Output
	{
		RingQueueEntryIndex(self.0 + relative_frame_index)
	}
}
