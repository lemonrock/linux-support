// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ethtool_value
{
	pub(crate) cmd: u32,
	
	/// Is `NETIF_MSG` if `cmd` is `ETHTOOL_GMSGLVL` or `ETHTOOL_SMSGLVL`.
	///
	/// Is private flags if `cmd` is `ETHTOOL_GPFLAGS` or `ETHTOOL_SPFLAGS`.
	pub(crate) data: u32,
}

impl EthtoolCommand for ethtool_value
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}
