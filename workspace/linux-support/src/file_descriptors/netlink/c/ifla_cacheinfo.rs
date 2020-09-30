// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Cache Information.
///
/// Used only by Internet Protocol version 6.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct ifla_cacheinfo
{
	/// Always the constant value `IPV6_MAXPLEN`.
	max_reasm_len: u32,
	
	/// Internet Protocol version 6 updated timestamp.
	pub(crate) tstamp: u32,
	
	/// Milliseconds.
	pub(crate) reachable_time: u32,
	
	/// Milliseconds.
	pub(crate) retrans_time: u32,
}

impl ifla_cacheinfo
{
	#[inline(always)]
	pub(crate) fn debug_assert_max_reasm_len_is_constant(&self)
	{
		debug_assert_eq!(self.max_reasm_len, IPV6_MAXPLEN);
	}
}
