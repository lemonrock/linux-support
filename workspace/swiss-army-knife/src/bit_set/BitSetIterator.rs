// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Iterator.
// TODO: Performance can be improved by using `std::arch::x86_64::_blsi_u64()` to reduce the number of loops.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitSetIterator<'a, BSA: BitSetAware>
{
	bit_set: &'a BitSet<BSA>,
	word_index: usize,
	relative_bit_index_within_word: usize,
}

impl<'a, BSA: BitSetAware> Iterator for BitSetIterator<'a, BSA>
{
	type Item = BSA;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		while likely!(self.word_index != self.bit_set.0.len())
		{
			let word = * unsafe { self.bit_set.0.get_unchecked(self.word_index) };

			// Short-cut.
			if likely!(word == 0)
			{
				self.word_index += 1;
				continue
			}

			while likely!(self.relative_bit_index_within_word < BitSet::<BSA>::BitsInAWord)
			{
				if word & (1 << self.relative_bit_index_within_word) != 0
				{
					let result = Some(BSA::hydrate(((self.word_index * size_of::<usize>() * BitsInAByte) + self.relative_bit_index_within_word) as u16));

					self.relative_bit_index_within_word += 1;
					if self.relative_bit_index_within_word == BitSet::<BSA>::BitsInAWord
					{
						self.relative_bit_index_within_word = 0;
						self.word_index +=1 ;
					}

					return result

				}

				self.relative_bit_index_within_word += 1;
			}

			self.word_index += 1;
			self.relative_bit_index_within_word = 0;
		}
		None
	}
}

impl<'a, BSA: BitSetAware> BitSetIterator<'a, BSA>
{
	// TODO: We could cache the first non-zero `word_index` and `relative_bit_index_within_word`.
	#[inline(always)]
	fn reset(&mut self)
	{
		self.word_index = 0;
		self.relative_bit_index_within_word = 0;
	}
}
