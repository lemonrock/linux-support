// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Forward error correction codes.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum ForwardErrorCorrectionCode
{
	#[serde(skip)]
	#[doc(hidden)]
	ETHTOOL_FEC_NONE = ethtool_fec_config_bits::ETHTOOL_FEC_NONE_BIT.to_bit(),
	
	/// Default/Best FEC mode provided by driver.
	///
	/// Ethtool setting is `auto`.
	ETHTOOL_FEC_AUTO = ethtool_fec_config_bits::ETHTOOL_FEC_AUTO_BIT.to_bit(),
	
	/// No FEC Mode.
	///
	/// Ethtool setting is `off`.
	ETHTOOL_FEC_OFF = ethtool_fec_config_bits::ETHTOOL_FEC_OFF_BIT.to_bit(),
	
	/// Reed-Solomon Forward Error Detection mode.
	///
	/// Force RS-FEC encoding.
	///
	/// Ethtool setting is `rs`.
	ETHTOOL_FEC_RS = ethtool_fec_config_bits::ETHTOOL_FEC_RS_BIT.to_bit(),
	
	/// Base-R/Reed-Solomon Forward Error Detection mode.
	///
	/// Force BaseR encoding.
	///
	/// Ethtool setting is `baser`.
	ETHTOOL_FEC_BASER = ethtool_fec_config_bits::ETHTOOL_FEC_BASER_BIT.to_bit(),
	
	/// LLRS (?Link-Local Reed-Solomon)?
	///
	/// Force LLRS-FEC encoding.
	///
	/// Ethtool setting is `llrs`.
	ETHTOOL_FEC_LLRS = ethtool_fec_config_bits::ETHTOOL_FEC_LLRS_BIT.to_bit(),
}
