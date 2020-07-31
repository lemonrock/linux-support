// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LinkModeBitSet([BitSetWord; Self::__ETHTOOL_LINK_MODE_MASK_NU32]);

impl LinkModeBitSet
{
	pub(crate) const __ETHTOOL_LINK_MODE_MASK_NU32: usize = BitSetHelper::divide_rounded_up_word(ethtool_link_mode_bit_indices::__ETHTOOL_LINK_MODE_MASK_NBITS);
	
	#[inline(always)]
	pub(crate) fn is_set(&self, bit: ethtool_link_mode_bit_indices) -> bool
	{
		bit.is_set(&self.0)
	}
	
	#[inline(always)]
	pub(crate) fn set(&mut self, bit: ethtool_link_mode_bit_indices)
	{
		bit.set(&mut self.0)
	}
}
