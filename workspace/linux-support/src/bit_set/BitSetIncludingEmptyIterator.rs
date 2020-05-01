// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Iterator.
///
/// Empty items are returned as `Some(None)`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitSetIncludingEmptyIterator<'a, BSA: BitSetAware>
{
	bit_set: &'a BitSet<BSA>,
	word_index: usize,
	relative_bit_index_within_word: usize,
}

impl<'a, BSA: BitSetAware> Iterator for BitSetIncludingEmptyIterator<'a, BSA>
{
	type Item = Option<BSA>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if unlikely!(self.word_index == self.bit_set.0.len())
		{
			return None
		}

		let word = * unsafe { self.bit_set.0.get_unchecked(self.word_index) };
		let outcome = if word & (1 << self.relative_bit_index_within_word) == 0
		{
			None
		}
		else
		{
			Some(BSA::hydrate(((self.word_index * size_of::<usize>() * BitsInAByte) + self.relative_bit_index_within_word) as u16))
		};

		if self.relative_bit_index_within_word == (BitSet::<BSA>::BitsInAWord - 1)
		{
			self.word_index += 1;
			self.relative_bit_index_within_word = 0;
		}
		else
		{
			self.relative_bit_index_within_word += 1;
		}

		Some(outcome)
	}
}
