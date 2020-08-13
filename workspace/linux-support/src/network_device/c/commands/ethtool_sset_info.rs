// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// String set information.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ethtool_sset_info
{
	/// Always `ETHTOOL_GSSET_INFO`.
	cmd: u32,
	
	/// Always `0`.
	reserved: u32,
	
	/// A bitset of `(1 << ethtool_stringset)`.
	///
	/// On input, the desired sets.
	/// On output, the supported sets.
	sset_mask: u64,
	
	/// Buffer for string set sizes.
	///
	/// A string set size is an `u32`.
	///
	/// On input must be sized to be the number of entries in the `sset_mask` (ie `size_of::<u32>() * sset_mask.count_ones()`).
	/// On exit contains, in order, the supported string sets, eg if on output `sset_mask` contains `1` and `5`, `data[0]` is the size of set `1` and `data[1]` is the size of set `5`.
	data: __IncompleteArrayField<u8>
}

impl EthtoolCommand for ethtool_sset_info
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl VariablySizedEthtoolCommand for ethtool_sset_info
{
	type ArrayElement = u32;
	
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.sset_mask.count_ones()
	}
}

impl ethtool_sset_info
{
	#[inline(always)]
	pub(crate) fn new() -> VariablySizedEthtoolCommandWrapper<Self>
	{
		let mut sset_mask = 0;
		for string_set in ethtool_stringset::iter()
		{
			sset_mask |= string_set.to_u64_bit();
		}
		
		Self::new_with_initialized_header_but_uninitialized_array
		(
			Self
			{
				cmd: ETHTOOL_GSSET_INFO,
				reserved: 0,
				sset_mask,
				data: __IncompleteArrayField::new(),
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn supported_string_sets(&self) -> impl Iterator<ethtool_stringset>
	{
		struct StringSetBitSetIterator
		{
			sset_mask: u64,
			next_index: u64,
		}
		
		impl Iterator for StringSetBitSetIterator
		{
			type Item = ethtool_stringset;
			
			#[inline(always)]
			fn next(&mut self) -> Option<Self::Item>
			{
				const ExclusiveMaximum: u64 = ethtool_stringset::ETHTOOL_STRINGSET_COUNT as u64;
				debug_assert!(ExclusiveMaximum < 64, "Only 64 elements are possible in a bitset which is backed by an u64");
				debug_assert!(self.next_index <= ExclusiveMaximum);
				
				while self.next_index < ExclusiveMaximum
				{
					if self.sset_mask & (1 << next_index) != 0
					{
						self.next_index += 1;
						return Some(unsafe { transmute(next_index as u32) })
					}
					
					self.next_index += 1;
				}
				
				None
			}
		}
		
		StringSetBitSetIterator
		{
			sset_mask: self.sset_mask,
			next_index: 0,
		}
	}
	
	#[inline(always)]
	pub(crate) fn set_supported_string_sets_to_none(&mut self)
	{
		self.sset_mask = 0
	}
}
