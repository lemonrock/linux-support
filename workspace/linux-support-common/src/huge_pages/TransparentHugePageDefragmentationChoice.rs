// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transparent Huge Page (THP) defragmentation choice.
#[derive(Deserialize, Serialize)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TransparentHugePageDefragmentationChoice
{
	/// Never defragment.
	Never,

	/// Defer defragmentation until allocation requires it.
	Defer,

	/// Only for pages so specified by the `madvise()` (or `fadvise()`) syscall with the `MADV_HUGEPAGE` flag.
	Advise,

	/// Like `Defer` and `MAdvise`.
	///
	/// Only for pages so specified by the `madvise()` (or `fadvise()`) syscall with the `MADV_HUGEPAGE` flag.
	DeferAndAdvise,
}

impl Default for TransparentHugePageDefragmentationChoice
{
	#[inline(always)]
	fn default() -> Self
	{
		TransparentHugePageDefragmentationChoice::Never
	}
}

impl TransparentHugePageDefragmentationChoice
{
	#[inline(always)]
	fn to_value(self) -> &'static [u8]
	{
		use self::TransparentHugePageDefragmentationChoice::*;

		match self
		{
			Never => b"never\n" as &[u8],
			Defer => b"defer\n" as &[u8],
			Advise => b"madvise\n" as &[u8],
			DeferAndAdvise => b"defer+madvise\n" as &[u8],
		}
	}

	/// Defrag value.
	#[inline(always)]
	fn defrag_value(self) -> bool
	{
		use self::TransparentHugePageDefragmentationChoice::*;

		match self
		{
			Never => false,
			_ => true,
		}
	}

	/// Changes defragmentation using the Kernel-internal `khugepaged` daemon thread for Transparent Huge Pages (THP).
	///
	/// * The kernel default for `pages_to_scan` is 4096.
	/// * The kernel default for `scan_sleep_in_milliseconds` is 10_000.
	/// * The kernel default for `alloc_sleep_millisecs` is 60_000.
	/// * The kernel default for `how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages` is 511. Also known as `max_ptes_none`. A higher value leads to use additional memory for programs. A lower value produces less gains in performance. The value itself has very little effect on CPU usage.
	/// * The kernel default for `how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages` is 64. Also known as `max_ptes_swap`. A higher value can cause excessive swap IO and waste memory. A lower value can prevent THPs from being collapsed, resulting in fewer pages being collapsed into THPs, and so lower memory access performance.
	#[inline(always)]
	pub fn change_transparent_huge_pages_defragmentation(&self, sys_path: &SysPath, pages_to_scan: u16, scan_sleep_in_milliseconds: usize, allocation_sleep_in_milliseconds: usize, how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages: u16, how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages: u16) -> io::Result<()>
	{
		sys_path.khugepaged_file_path("pages_to_scan").write_value(pages_to_scan)?;
		sys_path.khugepaged_file_path("alloc_sleep_millisecs").write_value(scan_sleep_in_milliseconds)?;
		sys_path.khugepaged_file_path("scan_sleep_millisecs").write_value(allocation_sleep_in_milliseconds)?;
		sys_path.khugepaged_file_path("max_ptes_none").write_value(how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages)?;
		sys_path.khugepaged_file_path("max_ptes_swap").write_value(how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages)?;
		sys_path.khugepaged_file_path("defrag").write_value(self.defrag_value())?;
		sys_path.global_transparent_huge_memory_file_path("defrag").write_value(self.to_value())?;
		Ok(())
	}
}
