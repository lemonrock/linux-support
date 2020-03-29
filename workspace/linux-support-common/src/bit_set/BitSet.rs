// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a BitSet suitable for use with various Linux `/sys` files and NUMA syscalls.
///
/// Internally uses `usize` to match what Linux uses.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BitSet<BSA: BitSetAware>(Vec<usize>, PhantomData<BSA>);

impl<BSA: BitSetAware> BitSet<BSA>
{
	const BitsInAWord: usize = BitsInAByte * size_of::<usize>();

	const MaximumNumberOfUsizeWords: usize = (BSA::LinuxMaximum as usize + Self::BitsInAWord - 1) / Self::BitsInAWord;

	/// Creates a new empty bit set (all bits are initially zero) of the maximum possible size.
	#[inline(always)]
	pub fn new() -> Self
	{
		Self::with_capacity_in_words(Self::MaximumNumberOfUsizeWords)
	}

	/// Creates a new empty bit set (all bits are initially zero).
	#[inline(always)]
	fn with_capacity_in_32bit_tuples(size_in_32bit_tuples: usize) -> Self
	{
		Self::with_capacity_in_words((size_in_32bit_tuples / size_of::<u32>()) * size_of::<usize>())
	}

	/// Creates a new empty bit set (all bits are initially zero).
	///
	/// Panics if `size_in_words` exceeds that needed to represent `BSA::LinuxMaximum`.
	#[inline(always)]
	pub fn with_capacity_in_words(size_in_words: usize) -> Self
	{
		debug_assert!(size_in_words <= Self::MaximumNumberOfUsizeWords);

		let array_layout = Layout::array::<usize>(size_in_words).unwrap();
		let (pointer, _allocated_size) = Global.alloc_zeroed(array_layout).unwrap();
		Self(unsafe { Vec::from_raw_parts(pointer.as_ptr() as *mut usize, size_in_words, size_in_words) }, PhantomData)
	}

	/// Creates a new empty bit set with no bits at all.
	#[inline(always)]
	pub fn empty() -> Self
	{
		Self(Vec::new(), PhantomData)
	}

	/// Iterate.
	#[inline(always)]
	pub fn iterate<'a>(&'a self) -> BitSetIterator<'a, BSA>
	{
		BitSetIterator
		{
			bit_set: self,
			word_index: 0,
			relative_bit_index_within_word: 0,
		}
	}

	/// Adds.
	#[inline(always)]
	pub fn add(&mut self, element: BSA)
	{
		let (word_index, relative_bit_index_within_word) = Self::word_index_and_relative_bit_index_within_word(element);

		if word_index >= self.capacity()
		{
			self.extend(word_index + 1)
		}

		self.add_internal(word_index, relative_bit_index_within_word)
	}

	/// Adds, without checking capacity.
	#[inline(always)]
	pub unsafe fn add_unchecked(&mut self, element: BSA)
	{
		let (word_index, relative_bit_index_within_word) = Self::word_index_and_relative_bit_index_within_word(element);

		debug_assert!(word_index < self.capacity());

		self.add_internal(word_index, relative_bit_index_within_word)
	}

	/// Does this bit set contain `element`?
	#[inline(always)]
	pub fn contains(&self, element: BSA) -> bool
	{
		let (word_index, relative_bit_index_within_word) = Self::word_index_and_relative_bit_index_within_word(element);

		debug_assert!(word_index < self.capacity());

		let word = self.get_word(word_index);
		word & (1 << relative_bit_index_within_word) != 0
	}

	/// Removes top bits that are zero and tries to shrink underlying storage.
	///
	/// Use only if intending to store a BitSet in memory for a long time.
	#[inline(always)]
	pub fn shrink_to_fit(&mut self)
	{
		let current_length = self.capacity();
		let mut new_length = current_length;
		for word_index in (0 ..current_length).rev()
		{
			if self.get_word(word_index) == 0
			{
				new_length -= 1;
			}
			else
			{
				break
			}
		}
		if new_length != current_length
		{
			self.0.truncate(new_length)
		}
		self.0.shrink_to_fit()
	}

	/// Keeps only bits in both sets.
	///
	/// If sets are of different lengths, the missing bits are assumed to be zero.
	#[inline(always)]
	pub fn intersection(&mut self, other: &Self)
	{
		let our_length = self.capacity();
		let other_length = other.capacity();

		for word_index in 0 .. other_length
		{
			let our_word_pointer = unsafe { self.word_mut(word_index) };
			let our_word = *our_word_pointer;
			let other_word = other.get_word(word_index);

			*our_word_pointer = our_word | other_word
		}

		use self::Ordering::*;
		match our_length.cmp(&other_length)
		{
			Less => self.0.truncate(other_length),

			Equal => (),

			Greater =>
			{
				let extend_size = other_length - our_length;

				self.reserve_exact(extend_size);
				unsafe { other.as_ptr_offset(our_length).copy_to_nonoverlapping(self.as_mut_ptr_end(), extend_size) };
				self.set_length(other_length);
			}
		}
	}

	/// Creates a new, uninitialized bit set for use with Linux API calls.
	#[inline(always)]
	pub(crate) unsafe fn new_uninitialized() -> Self
	{
		Self::new_set_length(Self::MaximumNumberOfUsizeWords)
	}

	/// Creates a bit set from an u64 which is correctly set.
	#[inline(always)]
	pub(crate) fn new_from_u64(bits: u64) -> Self
	{
		let mut bit_set = Self::new_set_length(1);
		unsafe { bit_set.set_u64_unchecked(0, bits) };
		bit_set
	}

	#[inline(always)]
	pub(crate) fn new_from_words(words: *const usize, length: usize) -> Self
	{
		let mut bit_set = Self::new_set_length(1);
		unsafe { words.copy_to_nonoverlapping(bit_set.as_mut_ptr(), length) };
		bit_set
	}

	#[inline(always)]
	fn new_set_length(capacity: usize) -> Self
	{
		let mut new = Self::new_(Vec::with_capacity(capacity));
		new.set_length(capacity);
		new
	}

	#[inline(always)]
	fn new_(vec: Vec<usize>) -> Self
	{
		Self(vec, PhantomData)
	}

	/// Sets the byte at a byte (not bit) index to all bits in the byte.
	#[inline(always)]
	unsafe fn set_byte_unchecked(&mut self, byte_index: usize, byte: u8)
	{
		let word_index = byte_index / size_of::<usize>();
		debug_assert!(word_index < Self::MaximumNumberOfUsizeWords);

		debug_assert!(word_index < self.capacity());

		(self.as_mut_ptr() as *mut u8).offset(byte_index as isize).write(byte);
	}

	/// Sets the byte at a byte (not bit) index to all bits in the byte.
	#[cfg(target_pointer_width = "64")]
	#[inline(always)]
	pub(crate) unsafe fn set_u64_unchecked(&mut self, u64_index: usize, bits: u64)
	{
		self.set_word(u64_index, bits as usize)
	}

	#[inline(always)]
	pub(crate) unsafe fn set_word(&mut self, word_index: usize, bits: usize)
	{
		*self.word_mut(word_index) = bits
	}

	#[inline(always)]
	pub(crate) unsafe fn word_mut(&mut self, word_index: usize) -> &mut usize
	{
		debug_assert!(word_index < Self::MaximumNumberOfUsizeWords);

		debug_assert!(word_index < self.capacity());

		self.0.get_unchecked_mut(word_index)
	}

	/// Provides a pointer and a length suitable for some Linux API calls.
	#[inline(always)]
	pub(crate) fn to_raw_parts(&self) -> (*const usize, usize)
	{
		(self.as_ptr(), self.capacity())
	}

	/// Provides a pointer and a length suitable for some Linux API calls.
	#[inline(always)]
	pub(crate) fn to_raw_parts_mut(&mut self) -> (*mut usize, usize)
	{
		(self.as_mut_ptr(), self.capacity())
	}

	#[inline(always)]
	pub(crate) fn parse_hyper_thread_or_numa_node_bit_set(without_line_feed: &[u8]) -> Self
	{
		// n is number of tuples of 32-bits.
		// LENGTH is value.len()
		// LENGTH = 8 + (8 + 1)(n - 1)
		// LENGTH = 8 + 9(n - 1)
		// LENGTH = 8 - 9 + 9n
		// LENGTH = -1 + 9n
		// LENGTH = 9n - 1
		// ∴ n = (LENGTH - 1) / 9

		let length = without_line_feed.len();
		debug_assert!(length >= 8, "Length '{}' is less than 8", length);

		const Divisior: usize = 9;

		let length_less_one = length - 1;
		debug_assert_eq!(length_less_one % Divisior, 0, "Length '{}' less one is not a multiple of 9", length);

		let number_of_tuples = length_less_one / Divisior;

		let mut bit_set = Self::with_capacity_in_32bit_tuples(number_of_tuples);

		let iterator = split(&without_line_feed, b',').rev();

		let mut byte_index: usize = 0;
		for raw_tuple_value in iterator
		{
			debug_assert_eq!(raw_tuple_value.len(), 8, "Tuple '{:?}' is too long or short", raw_tuple_value);

			for relative_byte_index in 0 .. 8
			{
				// Linux uses 0-9 and a-f.
				let byte_value = match raw_tuple_value[relative_byte_index]
				{
					raw_byte @ b'0' ..= b'9' => raw_byte - b'0',
					raw_byte @ b'a' ..= b'f' => raw_byte - b'a',
					unexpected @ _ => panic!("Unexpected value in raw_tuple_value of '{:?}'", unexpected),
				};
				unsafe { bit_set.set_byte_unchecked(byte_index, byte_value) };
				byte_index += 1;
			}
		}
		bit_set
	}

	#[inline(always)]
	pub(crate) fn capacity(&self) -> usize
	{
		self.0.len()
	}

	#[inline(always)]
	pub(crate) fn extend_clone_to(&self, new_length: usize) -> Self
	{
		let current_length = self.capacity();
		debug_assert!(current_length <= new_length);

		let mut uninitialized = Self::new_set_length(new_length);
		unsafe { self.as_ptr().copy_to_nonoverlapping(uninitialized.as_mut_ptr(), current_length) }
		uninitialized.write_zeros(new_length);

		uninitialized
	}

	#[inline(always)]
	fn extend(&mut self, new_length: usize)
	{
		let current_length = self.capacity();
		let extend_size = new_length - current_length;
		self.reserve_exact(extend_size);
		self.write_zeros(new_length);
		self.set_length(new_length);
	}

	#[inline(always)]
	fn reserve_exact(&mut self, extend_size: usize)
	{
		self.0.reserve_exact(extend_size)
	}

	#[inline(always)]
	fn word_index_and_relative_bit_index_within_word(element: BSA) -> (usize, usize)
	{
		let value: u16 = element.into();
		let bit_index = value as usize;

		let word_index = bit_index / Self::BitsInAWord;
		let relative_bit_index_within_word = bit_index % Self::BitsInAWord;

		(word_index, relative_bit_index_within_word)
	}

	#[inline(always)]
	fn add_internal(&mut self, word_index: usize, relative_bit_index_within_word: usize)
	{
		let pointer = unsafe { self.word_mut(word_index) };
		let word = *pointer;
		*pointer = word | (1 << relative_bit_index_within_word)
	}

	#[inline(always)]
	fn get_word(&self, word_index: usize) -> usize
	{
		unsafe { *self.0.get_unchecked(word_index) }
	}

	#[inline(always)]
	fn as_mut_ptr_end(&mut self) -> *mut usize
	{
		self.as_mut_ptr_offset(self.capacity())
	}

	#[inline(always)]
	fn as_ptr_offset(&self, offset: usize) -> *const usize
	{
		unsafe { self.as_ptr().offset(offset as isize) }
	}

	#[inline(always)]
	fn as_mut_ptr_offset(&mut self, offset: usize) -> *mut usize
	{
		unsafe { self.as_mut_ptr().offset(offset as isize) }
	}

	#[inline(always)]
	fn as_ptr(&self) -> *const usize
	{
		self.0.as_ptr()
	}

	#[inline(always)]
	fn as_mut_ptr(&mut self) -> *mut usize
	{
		self.0.as_mut_ptr()
	}

	#[inline(always)]
	fn set_length(&mut self, length: usize)
	{
		unsafe { self.0.set_len(length) }
	}

	#[inline(always)]
	fn write_zeros(&mut self, new_length: usize)
	{
		let current_length = self.capacity();
		let extend_size = new_length - current_length;

		unsafe { self.as_mut_ptr_end().write_bytes(0x00, extend_size) };
	}
}
