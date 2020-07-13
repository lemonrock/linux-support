// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
{
	storage: Storage,
	align: [Align; 0],
}

impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> where Storage: AsRef<[u8]> + AsMut<[u8]>,
{
	#[inline(always)]
	pub(crate) fn get_bit(&self, index: usize) -> bool
	{
		debug_assert!(index / 8 < self.storage.as_ref().len());
		let byte_index = index / 8;
		let byte = self.storage.as_ref()[byte_index];
		let bit_index = if cfg!(target_endian = "big")
		{
			7 - (index % 8)
		}
		else
		{
			index % 8
		};
		let mask = 1 << bit_index;
		byte & mask == mask
	}

	#[inline(always)]
	pub(crate) fn set_bit(&mut self, index: usize, val: bool)
	{
		debug_assert!(index / 8 < self.storage.as_ref().len());
		let byte_index = index / 8;
		let byte = &mut self.storage.as_mut()[byte_index];
		let bit_index = if cfg!(target_endian = "big")
		{
			7 - (index % 8)
		}
		else
		{
			index % 8
		};
		let mask = 1 << bit_index;
		if val
		{
			*byte |= mask;
		}
		else
		{
			*byte &= !mask;
		}
	}

	#[inline(always)]
	pub(crate) fn get(&self, bit_offset: usize, bit_width: u8) -> u64
	{
		debug_assert!(bit_width <= 64);
		debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
		let mut val = 0;
		for i in 0..(bit_width as usize)
		{
			if self.get_bit(i + bit_offset)
			{
				let index = if cfg!(target_endian = "big")
				{
					bit_width as usize - 1 - i
				}
				else
				{
					i
				};
				val |= 1 << index;
			}
		}
		val
	}

	#[inline(always)]
	pub(crate) fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64)
	{
		debug_assert!(bit_width <= 64);
		debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
		debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
		for i in 0..(bit_width as usize)
		{
			let mask = 1 << i;
			let val_bit_is_set = val & mask == mask;
			let index = if cfg!(target_endian = "big")
			{
				bit_width as usize - 1 - i
			}
			else
			{
				i
			};
			self.set_bit(index + bit_offset, val_bit_is_set);
		}
	}
}
