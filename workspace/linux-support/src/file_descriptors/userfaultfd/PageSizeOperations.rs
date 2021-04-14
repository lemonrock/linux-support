// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct PageSizeOperations;

impl PageSizeOperations
{
	#[inline(always)]
	fn zero_page_for_write_protected_copies() -> VirtualAddress
	{
		const MaximumPossiblePageSizeOf64Kb: usize = 64 * 1024;
		
		static ZeroPageForWriteProtectedCopies: [u8; MaximumPossiblePageSizeOf64Kb] = [0; MaximumPossiblePageSizeOf64Kb];
		VirtualAddress::from_const_t(ZeroPageForWriteProtectedCopies.as_ptr())
	}
}

#[cfg(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "sparc64", target_arch = "x86_64"))]
impl PageSizeOperations
{
	#[inline(always)]
	fn default_page_size_or_huge_page_size_settings() -> &'static PageSizeOrHugePageSizeSettings
	{
		static Default: PageSizeOrHugePageSizeSettings = PageSizeOrHugePageSizeSettings::for_default_page_size();
		
		&Default
	}
	
	#[inline(always)]
	const fn number_of_bytes_from_number_of_pages(number_of_pages: usize) -> usize
	{
		number_of_pages * Self::page_size_in_bytes()
	}
	
	#[inline(always)]
	const fn number_of_pages_from_number_of_bytes_rounded_up(number_of_bytes: usize) -> usize
	{
		let page_size_in_bytes = Self::page_size_in_bytes();
		(number_of_bytes + page_size_in_bytes - 1) / page_size_in_bytes
	}
	
	#[inline(always)]
	const fn page_size_in_bytes() -> usize
	{
		Self::page_size().into_usize()
	}
	
	#[inline(always)]
	const fn page_size() -> PageSize
	{
		PageSize::default()
	}
}

#[cfg(not(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "sparc64", target_arch = "x86_64")))]
impl PageSizeOperations
{
	#[inline(always)]
	fn default_page_size_or_huge_page_size_settings() -> &'static PageSizeOrHugePageSizeSettings
	{
		static Default: SyncLazy<PageSizeOrHugePageSizeSettings> = SyncLazy::new(PageSizeOrHugePageSizeSettings::for_default_page_size);
		
		&Default
	}
	
	#[inline(always)]
	fn number_of_bytes_from_number_of_pages(number_of_pages: usize) -> usize
	{
		number_of_pages * Self::page_size_in_bytes()
	}
	
	#[inline(always)]
	fn number_of_pages_from_number_of_bytes_rounded_up(number_of_bytes: usize) -> usize
	{
		let size_in_bytes = Self::page_size_in_bytes();
		(number_of_bytes + size_in_bytes - 1) / size_in_bytes
	}
	
	#[inline(always)]
	fn page_size_in_bytes() -> usize
	{
		Self::page_size_in_bytes().into_usize()
	}
	
	#[inline(always)]
	fn page_size() -> PageSize
	{
		PageSize::default()
	}
}

