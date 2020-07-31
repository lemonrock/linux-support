// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Bit flags definition of ethtool_fec_configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub(crate) enum ethtool_fec_config_bits
{
	/// FEC mode configuration is not supported.
	ETHTOOL_FEC_NONE_BIT = 0,
	
	/// Default/Best FEC mode provided by driver.
	///
	/// Ethtool setting is `auto`.
	ETHTOOL_FEC_AUTO_BIT = 1,
	
	/// No FEC Mode.
	///
	/// Ethtool setting is `off`.
	ETHTOOL_FEC_OFF_BIT = 2,
	
	/// Reed-Solomon Forward Error Detection mode.
	///
	/// Ethtool setting is `rs`.
	ETHTOOL_FEC_RS_BIT = 3,
	
	/// Base-R/Reed-Solomon Forward Error Detection mode.
	///
	/// Ethtool setting is `baser`.
	ETHTOOL_FEC_BASER_BIT = 4,
	
	/// LLRS (?Link-Local Reed-Solomon)?
	///
	/// Ethtool setting is `llrs`.
	ETHTOOL_FEC_LLRS_BIT = 5,
}

impl ethtool_fec_config_bits
{
	const fn to_bit(self) -> u32
	{
		1 << (self as u32)
	}
}
