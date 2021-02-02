// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Deduplication information.
// NOTE: Very similar to `FileExtents`.
#[derive(Debug)]
pub struct Deduplicate
{
	buffer: Vec<u8>
}

impl Deref for Deduplicate
{
	type Target = [file_dedupe_range_info];

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.array()
	}
}

impl Deduplicate
{
	const Offset: usize = size_of::<file_dedupe_range>();

	const SizeOfDeduplicateRangeInformation: usize = size_of::<file_dedupe_range_info>();

	#[inline(always)]
	pub(crate) fn new<COW: CopyOnWrite>(from_logical_range_in_bytes: RangeInclusive<u64>, to_files_and_their_logical_offsets: &[(&COW, u64)]) -> Self
	{
		let (start, end) = from_logical_range_in_bytes.into_inner();
		let length = end - start + 1;
		debug_assert_ne!(length, 0, "from_logical_range_in_bytes can not be empty");

		let number_of_deduplication_range_elements = to_files_and_their_logical_offsets.len();
		debug_assert_ne!(number_of_deduplication_range_elements, 0, "to_files_and_their_logical_offsets can not be empty");
		debug_assert!(number_of_deduplication_range_elements <= u16::MAX as usize, "to_files_and_their_logical_offsets can not have more than u16::MAX elements");
		let size =
		{
			Self::size(number_of_deduplication_range_elements as u16)
		};
		if cfg!(debug_assertions)
		{
			let page_size: usize = PageSize::default().into();
			debug_assert!(size <= page_size, "to_files_and_their_logical_offsets has too many elements (more than 127 on a system with a 4Kb page size); it causes an internal size to exceed page size");
		}

		let mut buffer = Vec::with_capacity(size);
		unsafe { buffer.set_len(Self::Offset) };
		let mut deduplicate = Self
		{
			buffer
		};

		unsafe
		{
			deduplicate.file_dedupe_range_mut_ptr().write
			(
				file_dedupe_range
				{
					src_offset: start,
					src_length: length,
					dest_count: number_of_deduplication_range_elements as u16,
					reserved1: 0,
					reserved2: 0,
					info: []
				}
			);

			deduplicate.start_of_array_mut();
		}

		let array = deduplicate.array_mut();
		for (index, &(to, to_logical_offset)) in to_files_and_their_logical_offsets.iter().enumerate()
		{
			array.set_unchecked_mut_safe(index, file_dedupe_range_info
			{
				dest_fd: to.as_raw_fd() as i64,
				dest_offset: to_logical_offset,
				bytes_deduped: unsafe_uninitialized(),
				status: unsafe_uninitialized(),
				reserved: 0
			});
		}

		deduplicate
	}

	#[inline(always)]
	pub(crate) fn data_pointer(&mut self) -> *mut u8
	{
		self.buffer.as_mut_ptr()
	}

	#[inline(always)]
	const fn size(number_of_deduplication_range_elements: u16) -> usize
	{
		Self::Offset + (number_of_deduplication_range_elements as usize) * Self::SizeOfDeduplicateRangeInformation
	}

	#[inline(always)]
	fn dest_count(&self) -> u16
	{
		(unsafe { & * self.file_dedupe_range_ptr() }).dest_count
	}

	#[inline(always)]
	fn array(&self) -> &[file_dedupe_range_info]
	{
		unsafe { from_raw_parts(self.start_of_array(), self.dest_count() as usize) }
	}

	#[inline(always)]
	fn array_mut(&mut self) -> &mut [file_dedupe_range_info]
	{
		unsafe { from_raw_parts_mut(self.start_of_array_mut(), self.dest_count() as usize) }
	}

	#[inline(always)]
	fn file_dedupe_range_ptr(&self) -> *const file_dedupe_range
	{
		self.buffer.as_ptr() as *const file_dedupe_range
	}

	#[inline(always)]
	fn file_dedupe_range_mut_ptr(&mut self) -> *mut file_dedupe_range
	{
		self.buffer.as_mut_ptr() as *mut file_dedupe_range
	}

	#[inline(always)]
	fn start_of_array(&self) -> *const file_dedupe_range_info
	{
		unsafe { self.buffer.as_ptr().add(Self::Offset) as *const file_dedupe_range_info }
	}

	#[inline(always)]
	fn start_of_array_mut(&mut self) -> *mut file_dedupe_range_info
	{
		unsafe { self.buffer.as_mut_ptr().add(Self::Offset) as *mut file_dedupe_range_info }
	}

}
