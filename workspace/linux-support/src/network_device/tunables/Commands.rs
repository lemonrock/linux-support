// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[doc(hidden)]
pub enum Commands
{
	#[doc(hidden)]
	Normal,
	
	#[doc(hidden)]
	Physical,
}

impl Commands
{
	#[inline(always)]
	pub(crate) fn commands(self) -> (u32, u32)
	{
		use self::Commands::*;
		
		match self
		{
			Normal => (ETHTOOL_GTUNABLE, ETHTOOL_STUNABLE),
			
			Physical => (ETHTOOL_PHY_GTUNABLE, ETHTOOL_PHY_STUNABLE),
		}
	}
}
