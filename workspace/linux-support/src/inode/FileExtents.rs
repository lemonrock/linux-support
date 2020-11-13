// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File extents, ie non-sparse parts of a file.
// NOTE: Very similar to `Deduplicate`.
#[derive(Debug)]
pub struct FileExtents
{
	buffer: Vec<u8>,
}

impl Deref for FileExtents
{
	type Target = [fiemap_extent];

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.extents()
	}
}

impl FileExtents
{
	const Offset: usize = size_of::<fiemap>();

	const SizeOfExtent: usize = size_of::<fiemap_extent>();

	#[inline(always)]
	pub(crate) fn new(initial_number_of_extents_to_retrieve: NonZeroU32, logical_range_in_bytes: RangeInclusive<u64>, retrieve_file_extents_flags: RetrieveFileExtentsFlags, flags: u32) -> Self
	{
		let (start, end) = logical_range_in_bytes.into_inner();
		let length = end - start + 1;

		let number_of_extents_to_retrieve = initial_number_of_extents_to_retrieve.get();
		let mut buffer = Vec::with_capacity(Self::size(number_of_extents_to_retrieve));
		unsafe { buffer.set_len(Self::Offset) };
		let mut file_extents = Self
		{
			buffer
		};

		unsafe
		{
			file_extents.fiemap_mut_ptr().write
			(
				fiemap
				{
					fm_start: start,
					fm_length: length,
					fm_flags: retrieve_file_extents_flags.bits() | flags,
					fm_mapped_extents: unsafe_uninitialized(),
					fm_extent_count: number_of_extents_to_retrieve,
					fm_reserved: 0,
					fm_extents: []
				}
			);
		}

		file_extents
	}

	#[inline(always)]
	pub(crate) fn data_pointer(&mut self) -> *mut u8
	{
		self.buffer.as_mut_ptr()
	}

	#[inline(always)]
	const fn size(number_of_extents: u32) -> usize
	{
		Self::Offset + (number_of_extents as usize) * Self::SizeOfExtent
	}

	/// Set by Linux kernel.
	#[inline(always)]
	fn fm_mapped_extents(&self) -> u32
	{
		(unsafe { & * (self.buffer.as_ptr() as *const fiemap) }).fm_mapped_extents
	}

	/// Set by us.
	#[inline(always)]
	fn fm_extent_count(&self) -> u32
	{
		(unsafe { & * (self.buffer.as_ptr() as *const fiemap) }).fm_extent_count
	}

	#[inline(always)]
	pub(crate) fn fm_flags(&self) -> u32
	{
		(unsafe { & * (self.buffer.as_ptr() as *const fiemap) }).fm_flags
	}

	#[inline(always)]
	pub(crate) fn shrink_to_fit(&mut self)
	{
		let number_of_extents = self.fm_mapped_extents();
		debug_assert!(number_of_extents <= self.fm_extent_count());
		unsafe { self.buffer.set_len(Self::size(number_of_extents)) };
		self.buffer.shrink_to_fit();
	}

	#[inline(always)]
	pub(crate) fn contains_all_possible_extents(&self) -> bool
	{
		let number_of_extents_present = self.fm_mapped_extents();
		if number_of_extents_present == 0
		{
			false
		}
		else
		{
			if number_of_extents_present == self.fm_extent_count()
			{
				let last_extent = self.extents().get_unchecked_safe(number_of_extents_present as usize - 1);
				last_extent.flags().contains(FileExtentFlags::Last)
			}
			else
			{
				false
			}
		}
	}

	#[inline(always)]
	fn fiemap_mut_ptr(&mut self) -> *mut fiemap
	{
		self.buffer.as_mut_ptr() as *mut fiemap
	}

	#[inline(always)]
	fn start_of_extents_array(&self) -> *const fiemap_extent
	{
		unsafe { self.buffer.as_ptr().add(Self::Offset) as *const fiemap_extent }
	}

	#[inline(always)]
	pub(crate) fn increase_capacity(&mut self)
	{
		let current_number_of_extents = self.fm_extent_count();
		let new_number_of_extents = current_number_of_extents.saturating_mul(2);
		let bytes_to_increase_by =Self::size(new_number_of_extents) - Self::size(current_number_of_extents);

		self.buffer.reserve(bytes_to_increase_by);
		(unsafe { &mut * self.fiemap_mut_ptr() }).fm_extent_count = new_number_of_extents;
	}

	#[inline(always)]
	fn extents(&self) -> &[fiemap_extent]
	{
		unsafe { from_raw_parts(self.start_of_extents_array(), self.fm_mapped_extents() as usize) }
	}
}
