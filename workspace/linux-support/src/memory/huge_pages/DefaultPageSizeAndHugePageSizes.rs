// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Default page sizes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultPageSizeAndHugePageSizes
{
	default_page_size: PageSize,
	default_huge_page_size: Option<HugePageSize>,
	transparent_huge_page_size: Option<HugePageSize>,
	supported_huge_page_sizes: BTreeSet<HugePageSize>,
}

impl DefaultPageSizeAndHugePageSizes
{
	/// New instance.
	///
	/// Should be goog enough to use in a lazy_static.
	#[inline(always)]
	pub fn new(sys_path: &SysPath, proc_path: &ProcPath) -> Self
	{
		#[inline(always)]
		fn parse_and_return_if_supported(supported_huge_page_sizes: &BTreeSet<HugePageSize>, parse: impl FnOnce() -> Option<HugePageSize>) -> Option<HugePageSize>
		{
			if let Some(huge_page_size) = parse()
			{
				if supported_huge_page_sizes.contains(&huge_page_size)
				{
					Some(huge_page_size)
				}
				else
				{
					None
				}
			}
			else
			{
				None
			}
		}

		let supported_huge_page_sizes = HugePageSize::supported_huge_page_sizes(sys_path);

		Self
		{
			default_page_size: PageSize::current(),

			default_huge_page_size: parse_and_return_if_supported(&supported_huge_page_sizes, ||
			{
				let memory_information = MemoryInformation::parse(proc_path, b"", false).expect("Missing valid /proc/meminfo");
				HugePageSize::default_huge_page_size(&memory_information)
			}),

			transparent_huge_page_size: parse_and_return_if_supported(&supported_huge_page_sizes, || HugePageSize::transparent_huge_page_size(sys_path)),

			supported_huge_page_sizes,
		}
	}

	/// Default page size.
	#[inline(always)]
	pub fn default_page_size(&self) -> PageSize
	{
		self.default_page_size
	}

	/// This will return `None` if the kernel was compiled without `CONFIG_HUGETLBFS`, `sysfs` was not mounted or the default huge page size is not one of `supported_huge_page_sizes()`.
	#[inline(always)]
	pub fn default_huge_page_size(&self) -> Option<HugePageSize>
	{
		self.default_huge_page_size
	}

	/// This will return `None` if the kernel was compiled without ?`CONFIG_TRANSPARENT_HUGEPAGE`, `sysfs` was not mounted or the transparent huge page size is not one of `supported_huge_page_sizes()`.
	#[inline(always)]
	pub fn transparent_huge_page_size(&self) -> Option<HugePageSize>
	{
		self.transparent_huge_page_size
	}

	/// This will return an empty set if the kernel was compiled without `CONFIG_HUGETLBFS` or `sysfs` was not mounted.
	#[inline(always)]
	pub fn supported_huge_page_sizes(&self) -> &BTreeSet<HugePageSize>
	{
		&self.supported_huge_page_sizes
	}

	/// Best fit.
	///
	/// Best fit is defined by choosing the largest huge page size that could contain `size` as long as it doesn't waste more than `inclusive_maximum_bytes_wasted`.
	///
	/// So, if size is `1.5Gb` and `inclusive_maximum_bytes_wasted` was, say, 1024, this would prevent a 1Gb huge page size being selected.
	#[inline(always)]
	pub fn best_fit_huge_page_size_if_any(&self, size: u64, inclusive_maximum_bytes_wasted: u64) -> Option<HugePageSize>
	{
		let mut best_fit = None;
		for &huge_page_size in self.supported_huge_page_sizes.iter()
		{
			let huge_page_size_in_bytes = huge_page_size.size_in_bytes();

			let bytes_wasted = size % huge_page_size_in_bytes.get();
			if bytes_wasted > inclusive_maximum_bytes_wasted
			{
				continue
			}

			best_fit = Some(huge_page_size)
		}
		best_fit
	}

	/// Supported?
	#[inline(always)]
	pub fn this_or_next_smaller_supported_huge_page_size(&self, huge_page_size: HugePageSize) -> Option<HugePageSize>
	{
		if self.is_supported(huge_page_size)
		{
			return Some(huge_page_size)
		}

		self.supported_huge_page_sizes.range(.. huge_page_size).rev().next().map(|&huge_page_size| huge_page_size)
	}

	/// Supported?
	#[inline(always)]
	fn is_supported(&self, huge_page_size: HugePageSize) -> bool
	{
		self.supported_huge_page_sizes.contains(&huge_page_size)
	}
}
