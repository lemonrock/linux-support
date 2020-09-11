// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Connector port.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum PORT
{
	/// Twisted Pair (TP) connector.
	///
	/// Also BaseT.
	///
	/// Ethtool setting is `tp`.
	#[allow(dead_code)]
	PORT_TP = 0x00,
	
	/// Attachment Unit Interface (AUI) connector.
	///
	/// Ethtool setting is `aui`.
	#[allow(dead_code)]
	PORT_AUI = 0x01,
	
	/// Media-Independent Interface (MII) connector.
	///
	/// Ethtool setting is `mii`.
	#[allow(dead_code)]
	 PORT_MII = 0x02,

	/// Fibre.
	#[allow(dead_code)]
	PORT_FIBRE = 0x03,
	
	/// Bayonet Neill–Concelman (BNC) connector.
	///
	/// Ethtool setting is `bnc`.
	#[allow(dead_code)]
	PORT_BNC = 0x04,

	/// Direct attach copper.
	///
	/// Also 'CX4'.
	#[allow(dead_code)]
	 PORT_DA = 0x05,

	/// eg Backplane.
	#[allow(dead_code)]
	PORT_NONE = 0xEF,
	
	/// Other.
	#[allow(dead_code)]
	PORT_OTHER = 0xFF,
}
