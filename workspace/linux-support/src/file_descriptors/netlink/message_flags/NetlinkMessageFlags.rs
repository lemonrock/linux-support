// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct NetlinkMessageFlags(u16);

impl NetlinkMessageFlags
{
	#[inline(always)]
	pub(super) const fn new(common: NetlinkCommonMessageFlags, specific: NetlinkSpecificMessageFlags) -> Self
	{
		let specific: u16 = unsafe { transmute(specific) };
		Self(common.bits | specific)
	}
	
	#[inline(always)]
	pub(super) const fn common(self) -> NetlinkCommonMessageFlags
	{
		NetlinkCommonMessageFlags::from_bits_truncate(self.0)
	}
}
