// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Priority.
///
/// Ranks above Weight.
/// Ranks below Order.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Priority(pub u16);

impl Priority
{
	pub(crate) const Unassigned: Self = Self(0);
	
	#[inline(always)]
	pub(crate) const fn expand_range_of_u8(precendence: u8) -> Self
	{
		Self((precendence as u16) << 8)
	}
}
