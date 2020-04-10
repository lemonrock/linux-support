// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Page size or huge page size.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PageSizeOrHugePageSize
{
	/// Page size.
	PageSize(PageSize),

	/// Huge page size.
	HugePageSize(HugePageSize),
}

impl PageSizeOrHugePageSize
{
	/// Size in kilobytes.
	#[inline(always)]
	pub fn size_in_kilobytes(self) -> NonZeroKilobyte
	{
		use self::PageSizeOrHugePageSize::*;

		match self
		{
			PageSize(page_size) => page_size.size_in_kilobytes(),
			HugePageSize(huge_page_size) => huge_page_size.size_in_kilobytes(),
		}
	}

	/// Size in bytes.
	#[inline(always)]
	pub fn size_in_bytes(self) -> NonZeroU64
	{
		use self::PageSizeOrHugePageSize::*;

		match self
		{
			PageSize(page_size) => page_size.size_in_bytes(),
			HugePageSize(huge_page_size) => huge_page_size.size_in_bytes(),
		}
	}

	/// Non-zero number of pages from non-zero number of bytes, rounded up.
	#[inline(always)]
	pub fn non_zero_number_of_pages_from_non_zero_number_of_bytes_rounded_up(self, number_of_bytes: NonZeroU64) -> NonZeroNumberOfPages
	{
		unsafe { NonZeroU64::new_unchecked(self.number_of_pages_from_number_of_bytes_rounded_up(number_of_bytes.get())) }
	}

	/// Number of pages from number of bytes, rounded up.
	#[inline(always)]
	pub fn number_of_pages_from_number_of_bytes_rounded_up(self, number_of_bytes: u64) -> NumberOfPages
	{
		let size_in_bytes = self.size_in_bytes().get();
		(number_of_bytes + size_in_bytes - 1) / size_in_bytes
	}

	/// Non-zero number of bytes rounded up to number of pages.
	#[inline(always)]
	pub fn non_zero_number_of_bytes_rounded_up_to_multiple_of_page_size(self, number_of_bytes: NonZeroU64) -> NonZeroU64
	{
		unsafe { NonZeroU64::new_unchecked(self.number_of_bytes_rounded_up_to_multiple_of_page_size(number_of_bytes.get())) }
	}

	/// Number of bytes rounded up to number of pages.
	#[inline(always)]
	pub fn number_of_bytes_rounded_up_to_multiple_of_page_size(self, number_of_bytes: u64) -> u64
	{
		let size_in_bytes = self.size_in_bytes().get();
		((number_of_bytes + size_in_bytes - 1) / size_in_bytes) * size_in_bytes
	}

	/// Is this considered a gigantic huge page?
	#[inline(always)]
	pub fn can_have_a_dynamic_huge_page_pool(self) -> bool
	{
		use self::PageSizeOrHugePageSize::*;

		match self
		{
			PageSize(page_size) => page_size.can_have_a_dynamic_huge_page_pool(),
			HugePageSize(huge_page_size) => huge_page_size.can_have_a_dynamic_huge_page_pool(),
		}
	}

	/// Is this considered a gigantic huge page?
	#[inline(always)]
	pub fn is_a_gigantic_huge_page(self) -> bool
	{
		use self::PageSizeOrHugePageSize::*;

		match self
		{
			PageSize(page_size) => page_size.is_a_gigantic_huge_page(),
			HugePageSize(huge_page_size) => huge_page_size.is_a_gigantic_huge_page(),
		}
	}

	#[inline(always)]
	pub(crate) fn from_kilobytes(value: u64) -> Option<Self>
	{
		if let Some(page_size) = PageSize::from_kilobytes(value)
		{
			Some(PageSizeOrHugePageSize::PageSize(page_size))
		}
		else if let Some(huge_page_size) = HugePageSize::from_kilobytes(value)
		{
			Some(PageSizeOrHugePageSize::HugePageSize(huge_page_size))
		}
		else
		{
			None
		}
	}
}