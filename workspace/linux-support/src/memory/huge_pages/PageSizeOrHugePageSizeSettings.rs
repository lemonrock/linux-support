// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Exists to provide a way of cache commonly used parameters to `mmap` and `memfd_create`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PageSizeOrHugePageSizeSettings
{
	mmap_flag_bits: i32,
	
	memfd_flag_bits: i32,

	page_size_or_huge_page_size: PageSizeOrHugePageSize,
}

impl PageSizeOrHugePageSizeSettings
{
	const MFD_HUGETLB: i32 = 0x0004;
	
	/// Is using huge pages.
	#[inline(always)]
	pub fn is_using_huge_pages(&self) -> bool
	{
		matches!(self.page_size_or_huge_page_size, PageSizeOrHugePageSize::HugePageSize(_))
	}
	
	/// Page size or huge page size.
	pub const fn page_size_or_huge_page_size(&self) -> PageSizeOrHugePageSize
	{
		self.page_size_or_huge_page_size
	}
	
	pub(crate) const fn mmap_flag_bits_and_page_size(&self) -> (i32, PageSizeOrHugePageSize)
	{
		(self.mmap_flag_bits, self.page_size_or_huge_page_size)
	}
	
	pub(crate) const fn memfd_flag_bits_and_page_size(&self) -> (i32, PageSizeOrHugePageSize)
	{
		(self.memfd_flag_bits, self.page_size_or_huge_page_size)
	}
	
	/// Settings for page size, not huge page size.
	#[inline(always)]
	pub fn for_page_size(defaults: &DefaultPageSizeAndHugePageSizes) -> Self
	{
		Self::for_default_page_size(defaults.default_page_size())
	}
	
	/// Settings for page size, not huge page size.
	#[inline(always)]
	pub fn for_default_page_size(default_page_size: PageSize) -> Self
	{
		Self::new(0, 0, PageSizeOrHugePageSize::PageSize(default_page_size))
	}
	
	/// Settings for huge page size.
	#[inline(always)]
	pub fn for_huge_page_size(huge_page_size: HugePageSize) -> Self
	{
		let mmap_and_memfd_flags_bits = huge_page_size.mmap_and_memfd_flags_bits();
		Self::new(MAP_HUGETLB | mmap_and_memfd_flags_bits, Self::MFD_HUGETLB | mmap_and_memfd_flags_bits, PageSizeOrHugePageSize::HugePageSize(huge_page_size))
	}
	
	#[inline(always)]
	fn for_default_huge_page_size(default_huge_page_size: HugePageSize) -> Self
	{
		Self::new(MAP_HUGETLB, Self::MFD_HUGETLB, PageSizeOrHugePageSize::HugePageSize(default_huge_page_size))
	}
	
	/// Creates a new instance.
	///
	/// The value of `huge_page_size` is interpreted as follows:-
	///
	/// * `None`: Use the default page size.
	/// * `Some(None)`: Use the default huge page size if huge pages are supported, otherwise fallback to the default page size.
	/// * `Some(Some())`: Use the specified huge page size if possible, falling back to a supported page size smaller than that specified, and, if not possible, fallback to the default page size.
	#[inline(always)]
	pub fn from(page_size_preference: PageSizePreference, defaults: &DefaultPageSizeAndHugePageSizes) -> Self
	{
		use self::PageSizePreference::*;
		
		match page_size_preference
		{
			DefaultPageSize => Self::for_page_size(defaults),

			DefaultHugePageSize => match defaults.default_huge_page_size()
			{
				Some(default_huge_page_size) => Self::for_default_huge_page_size(default_huge_page_size),
				
				None => Self::for_page_size(defaults)
			},
			
			PreferredHugePageSize(huge_page_size) => match defaults.this_or_next_smaller_supported_huge_page_size(huge_page_size)
			{
				Some(huge_page_size) => Self::for_huge_page_size(huge_page_size),
				
				None => Self::for_page_size(defaults)
			},
		}
	}
	
	const fn new(mmmap_flag_bits: i32, memfd_flag_bits: i32, page_size_or_huge_page_size: PageSizeOrHugePageSize) -> Self
	{
		Self
		{
			mmap_flag_bits: mmmap_flag_bits,
			
			memfd_flag_bits,
			
			page_size_or_huge_page_size,
		}
	}
}
