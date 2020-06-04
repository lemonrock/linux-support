// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
#[allow(missing_debug_implementations)]
pub struct ethtool_rx_ntuple
{
	pub cmd: u32,
	pub fs: ethtool_rx_ntuple_flow_spec,
}

impl Clone for ethtool_rx_ntuple
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ethtool_rx_ntuple
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}