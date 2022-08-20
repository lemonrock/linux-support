// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Page move status.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PageMoveStatus(i32);

impl PageMoveStatus
{
	/// Converts to a result.
	#[inline(always)]
	pub fn to_result(&self) -> Result<NumaNode, PerPageMoveError>
	{
		if unlikely!(self.0 < 0)
		{
			Err(PerPageMoveError::from_errno(Errno(-self.0)))
		}
		else
		{
			Ok(NumaNode::try_from(self.0 as u16).unwrap())
		}
	}
}
