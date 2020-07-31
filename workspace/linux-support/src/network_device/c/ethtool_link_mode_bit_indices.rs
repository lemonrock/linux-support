// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// In `ethtool.h` this is an enum.
///
/// Used in `ethtool_link_settings.link_mode_masks`.
#[derive(Copy, Clone)]
#[repr(C)]
pub union ethtool_link_mode_bit_indices
{
	speed: ethtool_link_mode_bit_indices_speed,
	ports: ethtool_link_mode_bit_indices_ports,
	pause: ethtool_link_mode_bit_indices_pause,
	forward_error_correction: ethtool_link_mode_bit_indices_forward_error_correction,
}

impl Bit for ethtool_link_mode_bit_indices
{
	#[inline(always)]
	fn to_u32(self) -> u32
	{
		unsafe { transmute(self) }
	}
}

impl ethtool_link_mode_bit_indices
{
	pub(crate) const __ETHTOOL_LINK_MODE_MASK_NBITS: usize = 75;
}
