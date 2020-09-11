// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Which transceiver to use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum XCVR
{
	/// PHY and MAC are in the same package.
	///
	/// Ethtool setting is `internal`.
	#[allow(dead_code)]
	XCVR_INTERNAL = 0x00,
	
	/// PHY and MAC are in different packages.
	///
	/// Ethtool setting is `external`.
	#[allow(dead_code)]
	XCVR_EXTERNAL = 0x01,
	
	#[allow(dead_code)]
	XCVR_DUMMY1 = 0x02,
	
	#[allow(dead_code)]
	XCVR_DUMMY2 = 0x03,
	
	#[allow(dead_code)]
	XCVR_DUMMY3 = 0x04,
}
