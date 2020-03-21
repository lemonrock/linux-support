// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transparent Huge Pages Configuration.
///
/// Default implementation disables transparent huge pages entirely.
#[derive(Deserialize, Serialize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransparentHugePagesConfiguration
{
	#[allow(missing_docs)]
	#[serde(default)] pub defragmentation_choice: TransparentHugePageDefragmentationChoice,

	#[allow(missing_docs)]
	#[serde(default = "TransparentHugePagesConfiguration::defragmentation_pages_to_scan_default")] pub defragmentation_pages_to_scan: u16,

	#[allow(missing_docs)]
	#[serde(default = "TransparentHugePagesConfiguration::defragmentation_scan_sleep_in_milliseconds_default")] pub defragmentation_scan_sleep_in_milliseconds: usize,

	#[allow(missing_docs)]
	#[serde(default = "TransparentHugePagesConfiguration::defragmentation_allocation_sleep_in_milliseconds_default")] pub defragmentation_allocation_sleep_in_milliseconds: usize,

	#[allow(missing_docs)]
	#[serde(default = "TransparentHugePagesConfiguration::defragmentation_how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages_default")] pub defragmentation_how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages: u16,

	#[allow(missing_docs)]
	#[serde(default = "TransparentHugePagesConfiguration::defragmentation_how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages_default")] pub defragmentation_how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages: u16,

	#[allow(missing_docs)]
	#[serde(default)] pub regular_memory_choice: TransparentHugePageRegularMemoryChoice,

	#[allow(missing_docs)]
	#[serde(default)] pub shared_memory_choice: TransparentHugePageSharedMemoryChoice,

	#[allow(missing_docs)]
	#[serde(default = "TransparentHugePagesConfiguration::use_zero_page_default")] pub use_zero_page: bool,

	#[allow(missing_docs)]
	#[serde(default)] pub enable: bool,
}

impl Default for TransparentHugePagesConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			defragmentation_choice: TransparentHugePageDefragmentationChoice::Never,
			defragmentation_pages_to_scan: Self::defragmentation_pages_to_scan_default(),
			defragmentation_scan_sleep_in_milliseconds: Self::defragmentation_scan_sleep_in_milliseconds_default(),
			defragmentation_allocation_sleep_in_milliseconds: Self::defragmentation_allocation_sleep_in_milliseconds_default(),
			defragmentation_how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages: Self::defragmentation_how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages_default(),
			defragmentation_how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages: Self::defragmentation_how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages_default(),
			regular_memory_choice: TransparentHugePageRegularMemoryChoice::Never,
			shared_memory_choice: TransparentHugePageSharedMemoryChoice::Never,
			use_zero_page: Self::use_zero_page_default(),
			enable: false,
		}
	}
}

impl TransparentHugePagesConfiguration
{
	/// Configure.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath) -> Result<(), DisableTransparentHugePagesError>
	{
		use self::DisableTransparentHugePagesError::*;

		TransparentHugePageDefragmentationChoice::Never.change_transparent_huge_pages_defragmentation(sys_path, 4096, 60_000, 10_000, 511, 64).map_err(|io_error| Defragmentation(io_error))?;

		self.regular_memory_choice.change_transparent_huge_pages_usage(sys_path, self.shared_memory_choice, self.use_zero_page).map_err(|io_error| Usage(io_error))?;

		adjust_transparent_huge_pages(self.enable);

		Ok(())
	}

	#[inline(always)]
	fn defragmentation_pages_to_scan_default() -> u16
	{
		4096
	}

	#[inline(always)]
	fn defragmentation_scan_sleep_in_milliseconds_default() -> usize
	{
		60_000
	}

	#[inline(always)]
	fn defragmentation_allocation_sleep_in_milliseconds_default() -> usize
	{
		10_000
	}

	#[inline(always)]
	fn defragmentation_how_many_extra_small_pages_not_already_mapped_can_be_allocated_when_collapsing_small_pages_default() -> u16
	{
		511
	}

	#[inline(always)]
	fn defragmentation_how_many_extra_small_pages_not_already_mapped_can_be_swapped_when_collapsing_small_pages_default() -> u16
	{
		64
	}

	#[inline(always)]
	fn use_zero_page_default() -> bool
	{
		true
	}
}
