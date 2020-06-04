// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_ringparam
{
	pub cmd: u32,
	pub rx_max_pending: u32,
	pub rx_mini_max_pending: u32,
	pub rx_jumbo_max_pending: u32,
	pub tx_max_pending: u32,
	pub rx_pending: u32,
	pub rx_mini_pending: u32,
	pub rx_jumbo_pending: u32,
	pub tx_pending: u32,
}

impl Default for ethtool_ringparam
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
