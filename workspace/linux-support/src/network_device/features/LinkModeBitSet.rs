// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LinkModeBitSet(pub(crate) [BitSetWord; Self::__ETHTOOL_LINK_MODE_MASK_NU32]);

impl LinkModeBitSet
{
	pub(crate) const __ETHTOOL_LINK_MODE_MASK_NU32: usize = divide_rounded_up_word(ethtool_link_mode_bit_indices::__ETHTOOL_LINK_MODE_MASK_NBITS);
	
	#[inline(always)]
	pub(crate) fn is_set(&self, bit: ethtool_link_mode_bit_indices) -> bool
	{
		bit.is_set(&self.0)
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn set(&mut self, bit: ethtool_link_mode_bit_indices)
	{
		bit.set(&mut self.0)
	}
	
	#[inline(always)]
	pub(crate) fn speeds(&self) -> HashSet<ethtool_link_mode_bit_indices_speed>
	{
		self.to_set(|speed| ethtool_link_mode_bit_indices { speed })
	}
	
	#[inline(always)]
	pub(crate) fn port_connectors(&self) -> HashSet<ethtool_link_mode_bit_indices_ports>
	{
		self.to_set(|ports| ethtool_link_mode_bit_indices { ports })
	}
	
	#[inline(always)]
	pub(crate) fn pauses(&self) -> HashSet<ethtool_link_mode_bit_indices_pause>
	{
		self.to_set(|pause| ethtool_link_mode_bit_indices { pause })
	}
	
	#[inline(always)]
	pub(crate) fn forward_error_corrections(&self) -> HashSet<ethtool_link_mode_bit_indices_forward_error_correction>
	{
		self.to_set(|forward_error_correction| ethtool_link_mode_bit_indices { forward_error_correction })
	}
	
	#[inline(always)]
	fn to_set<E: IntoEnumIterator + EnumCount + Eq + Hash + Copy>(&self, into_ethtool_link_mode_bit_indices: impl FnOnce(E) -> ethtool_link_mode_bit_indices + Copy) -> HashSet<E>
	{
		let mut set = HashSet::with_capacity(E::COUNT);
		for e in E::iter()
		{
			if self.is_set(into_ethtool_link_mode_bit_indices(e))
			{
				set.insert(e);
			}
		}
		set.shrink_to_fit();
		set
	}
}
