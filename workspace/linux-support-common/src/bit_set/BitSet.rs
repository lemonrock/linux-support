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

	/// Parses a Linux list string used for cpu sets, core masks and NUMA nodes such as "2,4-31,32-63" and "1,2,10-20,100-2000:2/25" (see <https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html> for an awful description of this mad syntax).
	///
	/// Returns a BitSet with the zero-based indices found in the string.
	/// For example, "2,4-31,32-63" would return a set with all values between 0 to 63 except 0, 1 and 3.
	///
	/// `linux_list_string` does not have a terminating line feed (LF).
	pub fn parse_linux_list_string(linux_list_string: &[u8]) -> Result<BitSet<BSA>, ListParseError>
	{
		#[inline(always)]
		fn parse_index(index_string: &[u8], description: &'static str) -> Result<u16, ListParseError>
		{
			use self::ListParseError::*;

			let index_string = match from_utf8(index_string)
			{
				Ok(index_string) => index_string,
				Err(cause) => return Err(CouldNotParseIndexAsNotAString { description, unparsable_index: index_string.to_vec().into_boxed_slice(), cause }),
			};

			match index_string.parse()
			{
				Ok(index) => Ok(index),
				Err(cause) => Err(CouldNotParseIndex { description, unparsable_index: index_string.to_owned(), cause }),
			}
		}

		let mut result = Self::new();

		use self::ListParseError::*;

		// Prevents mis-sorted strings
		let mut next_minimum_index_expected = 0;
		for index_or_range in split(linux_list_string, b',')
		{
			if index_or_range.is_empty()
			{
				return Err(ContainsAnEmptyIndexOrRange);
			}

			let mut range_iterator = splitn(index_or_range, 2, b'-');

			let first =
			{
				let index = parse_index(range_iterator.next().unwrap(), "first")?;
				if index < next_minimum_index_expected
				{
					return Err(ContainsMisSortedIndices { first: index, next_minimum_index_expected });
				}
				index
			};

			if let Some(second) = range_iterator.last()
			{
				// There is a weird, but rare, syntax used of `100-2000:2/25` for some ranges.
				let mut range_or_range_with_groups = splitn(second, 2, b':');

				let second =
				{
					let index = parse_index(range_or_range_with_groups.next().unwrap(), "second")?;
					if first >= index
					{
						return Err(RangeIsNotAnAscendingRangeWithMoreThanOneElement { first, second: index });
					}
					index
				};

				match range_or_range_with_groups.last()
				{
					None =>
					{
						for index in first .. (second + 1)
						{
							result.map_and_add(index)?;
						}

						next_minimum_index_expected = second;
					}

					Some(weird_but_rare_group_syntax) =>
					{
						let mut weird_but_rare_group_syntax = splitn(weird_but_rare_group_syntax, 2, b'/');
						let used_size = parse_index(weird_but_rare_group_syntax.next().unwrap(), "used_size")?;
						let group_size = parse_index(weird_but_rare_group_syntax.last().expect("a group does not have group_size"), "group_size")?;

						assert_ne!(used_size, 0, "used_size is zero");
						assert_ne!(group_size, 0, "group_size is zero");

						let mut base_cpu_index = first;
						while base_cpu_index < second
						{
							for cpu_index_increment in 0 .. used_size
							{
								let cpu_index = base_cpu_index + cpu_index_increment;
								result.map_and_add(cpu_index)?;
							}

							base_cpu_index += group_size;
						}
					}
				}
			}
			else
			{
				let sole = first;
				result.map_and_add(sole)?;
				next_minimum_index_expected = sole;
			}
		}

		Ok(result)
	}

	/// Adds.
	#[inline(always)]
	fn map_and_add(&mut self, index: u16) -> Result<(), ListParseError>
	{
		self.add(BSA::try_from(index)?);
		Ok(())
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

	/// Number of bits set.
	///
	/// This operation is a little expensive, but less expensive than `self.len()`.
	#[inline(always)]
	pub fn is_empty(&self) -> bool
	{
		for word_index in 0 .. self.capacity()
		{
			if self.get_word(word_index) != 0
			{
				return false
			}
		}
		true
	}

	/// Number of bits set.
	///
	/// This operation is relatively expensive.
	///
	/// For an emptiness check, prefer `self.is_empty()` which is slightly less expensive.
	#[cfg(all(target_arch = "x86_64", target_feature = "popcnt"))]
	#[inline(always)]
	pub fn len(&self) -> usize
	{
		let mut count = 0;
		for word_index in 0 .. self.capacity()
		{
			count += unsafe { _mm_popcnt_u64(self.get_word(word_index)) };
		}
		count as usize
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

	/// Removes.
	#[inline(always)]
	pub fn remove(&mut self, element: BSA)
	{
		let (word_index, relative_bit_index_within_word) = Self::word_index_and_relative_bit_index_within_word(element);

		if word_index >= self.capacity()
		{
			return
		}

		let word_pointer = unsafe { self.word_mut(word_index) };
		let current = *word_pointer;
		*word_pointer = current & !(1 << relative_bit_index_within_word)
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

	/// Removes bits in `other`.
	#[inline(always)]
	pub fn remove_all(&mut self, other: &Self)
	{
		let our_length = self.capacity();
		let other_length = other.capacity();

		for word_index in 0 .. min(our_length, other_length)
		{
			let our_word_pointer = unsafe { self.word_mut(word_index) };
			let our_word = *our_word_pointer;
			let other_word = other.get_word(word_index);

			*our_word_pointer = our_word & !other_word
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
