// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Connector port.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum PORT
{
	/// Ethtool setting is `tp`.
	PORT_TP = 0x00,
	
	/// Ethtool setting is `aui`.
	PORT_AUI = 0x01,
	
	/// Ethtool setting is `mii`.
	PORT_MII = 0x02,

	PORT_FIBRE = 0x03,
	
	/// Ethtool setting is `bnc`.
	PORT_BNC = 0x04,

	PORT_DA = 0x05,

	PORT_NONE = 0xEF,

	PORT_OTHER = 0xFF,

}
