// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Ethernet forward error correction (FEC) parameters.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ethtool_fecparam
{
	/// Always either `ETHTOOL_GFECPARAM` or `ETHTOOL_SFECPARAM`.
	pub(crate) cmd: u32,
	
	/// FEC mode which is active on port.
	pub(crate) active_fec: ForwardErrorCorrectionCode,
	
	/// Bitmask of supported or configured FEC modes (enum `ethtool_fec_config_bits`).
	///
	/// Drivers should reject setting `ETHTOOL_FEC_AUTO_BIT` when auto-negotiation is disabled (or not supported) for the link.
	pub(crate) fec: BitSetWord,
	
	/// Reserved for future extensions, eg a FEC bypass feature.
	pub(crate) reserved: u32,
}

impl EthtoolCommand for ethtool_fecparam
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl ethtool_fecparam
{
	#[inline(always)]
	pub(crate) const fn is_supported_forward_error_correction_code(&self, forward_error_correction_code: ForwardErrorCorrectionCode) -> bool
	{
		(self.fec & (forward_error_correction_code as u32)) != 0
	}
	
	#[inline(always)]
	pub(crate) fn to_forward_error_correction_codes(&self) -> HashSet<ForwardErrorCorrectionCode>
	{
		let mut forward_error_correction_codes = HashSet::with_capacity(ForwardErrorCorrectionCode::COUNT);
		for forward_error_correction_code in ForwardErrorCorrectionCode::iter()
		{
			if self.is_supported_forward_error_correction_code(forward_error_correction_code)
			{
				forward_error_correction_codes.insert(forward_error_correction_code);
			}
		}
		forward_error_correction_codes.shrink_to_fit();
		forward_error_correction_codes
	}
}
