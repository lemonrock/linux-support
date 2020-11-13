// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait Bit: Sized
{
	#[inline(always)]
	fn is_set(self, bit_set: &[BitSetWord]) -> bool
	{
		self.is_set_field_locator::<BitSetWord, _>(bit_set, |field| *field)
	}
	
	// Could be implemented generically if arrays were generic.
	#[inline(always)]
	fn is_set_field_locator<X, FL: FnOnce(&X) -> BitSetWord>(self, bit_set: &[X], field_locator: FL) -> bool
	{
		let (word_index, bit) = self.to_word_index_and_relative_bit_in_word();
		
		let fields = bit_set.get_unchecked_safe(word_index);
		let word = field_locator(fields);
		
		(word & bit) != 0
	}
	
	#[inline(always)]
	fn set(self, bit_set: &mut [BitSetWord])
	{
		self.set_field_locator::<BitSetWord, _>(bit_set, |field| field)
	}
	
	#[inline(always)]
	fn set_field_locator<X, FL: FnOnce(&mut X) -> &mut BitSetWord>(self, bit_set: &mut [X], field_locator: FL)
	{
		let (word_index, bit) = self.to_word_index_and_relative_bit_in_word();
		
		let fields = bit_set.get_unchecked_mut_safe(word_index);
		let word = field_locator(fields);
		
		*word = *word | bit
	}
	
	#[inline(always)]
	fn unset(self, bit_set: &mut [BitSetWord])
	{
		self.unset_field_locator::<BitSetWord, _>(bit_set, |field| field)
	}
	
	#[inline(always)]
	fn unset_field_locator<X, FL: FnOnce(&mut X) -> &mut BitSetWord>(self, bit_set: &mut [X], field_locator: FL)
	{
		let (word_index, bit) = self.to_word_index_and_relative_bit_in_word();
		
		let fields = bit_set.get_unchecked_mut_safe(word_index);
		let word = field_locator(fields);
		
		*word = *word & !bit;
	}
	
	#[inline(always)]
	fn to_word_index_and_relative_bit_in_word(self) -> (usize, u32)
	{
		let bit = self.to_u32();
		let word = (bit as usize) / WordSizeInBits;
		let bit_in_word = 1 << (bit % (WordSizeInBits as u32));
		(word, bit_in_word)
	}
	
	fn to_u32(self) -> u32;
}
