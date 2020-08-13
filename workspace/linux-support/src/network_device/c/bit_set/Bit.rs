// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait Bit
{
	#[inline(always)]
	fn is_set(self, bit_set: &[BitSetWord]) -> bool
	{
		is_set_field_locator::<BitSetWord>(bit_set, |field| *field)
	}
	
	// Could be implemented generically if arrays were generic.
	#[inline(always)]
	fn is_set_field_locator<X>(self, bit_set: &[X], field_locator: impl FnOnce(&X) -> BitSetWord) -> bool
	{
		let (word_index, bit) = self.to_word_index_and_relative_bit_index_in_word();
		
		let fields = unsafe { bit_set.get_unchecked(word_index) };
		let word = field_locator(fields);
		
		(word & bit) != 0
	}
	
	#[inline(always)]
	fn set(self, bit_set: &mut [BitSetWord])
	{
		self.set_field_locator::<BitSetWord>(bit_set, |field| field)
	}
	
	#[inline(always)]
	fn set_field_locator<X>(self, bit_set: &mut [X], field_locator: impl FnOnce(&mut X) -> &mut BitSetWord)
	{
		let (word_index, bit) = self.to_word_index_and_relative_bit_index_in_word();
		
		let fields = unsafe { bit_set.get_unchecked_mut(word_index) };
		let word = field_locator(fields);
		
		*word = *word | bit
	}
	
	#[inline(always)]
	fn unset(self, bit_set: &mut [BitSetWord])
	{
		self.unset_field_locator::<BitSetWord>(bit_set, |field| field)
	}
	
	#[inline(always)]
	fn unset_field_locator<X>(self, bit_set: &mut [X], field_locator: impl FnOnce(&mut X) -> &mut BitSetWord)
	{
		let (word_index, bit) = self.to_word_index_and_relative_bit_index_in_word();
		
		let fields = unsafe { bit_set.get_unchecked_mut(word_index) };
		let word = field_locator(fields);
		
		*word = *word & !bit;
	}
	
	#[inline(always)]
	fn to_word_index_and_relative_bit_in_word(self) -> (u32, u32)
	{
		let bit = self.to_u32();
		(bit / WordSizeInBits, 1 << (bit % WordSizeInBits))
	}
	
	fn to_u32(self) -> u32;
}
