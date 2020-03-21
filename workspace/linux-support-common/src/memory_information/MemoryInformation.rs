// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A set of memory statistics.
///
/// Super-detailed information (hard to parse, too) is in `/proc/zoneinfo`.
/// This is broken down into DMA, DMA33 and Normal sub-zones and then by CPU for each Numa Node (aka 'zone').
/// A sort of detailed version of `/proc/vmstat`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryInformation(pub(crate) HashMap<MemoryInformationName, u64>);

impl MemoryInformation
{
	/// Memory information (from `/proc/meminfo`).
	#[inline(always)]
	pub fn parse(proc_path: &ProcPath, memory_information_name_prefix: &[u8]) -> Result<Self, MemoryInformationParseError>
	{
		proc_path.file_path("meminfo").parse_memory_information_file(memory_information_name_prefix)
	}

	/// Get a statistic.
	#[inline]
	pub fn get_statistic(&self, memory_information_name: &MemoryInformationName) -> Option<u64>
	{
		match self.0.get(memory_information_name)
		{
			None => None,
			Some(value) => Some(*value),
		}
	}
	
	/// Free physical RAM in Kilobytes.
	#[inline(always)]
	pub fn free_physical_ram(&self) -> Option<u64>
	{
		self.get_statistic(&MemoryInformationName::FreePhysicalRam)
	}
	
	/// Default huge page size.
	pub fn default_huge_page_size(&self) -> Option<HugePageSize>
	{
		if let Some(size_in_bytes) = self.get_statistic(&MemoryInformationName::SizeOfAHugePage)
		{
			HugePageSize::from_proc_mem_info_value(size_in_bytes)
		}
		else
		{
			None
		}
	}
	
	/// Used physical RAM in bytes.
	#[inline]
	pub fn used_physical_ram(&self) -> Option<u64>
	{
		if let Some(total_physical_ram) = self.get_statistic(&MemoryInformationName::TotalPhysicalRam)
		{
			if let Some(free_physical_ram) = self.get_statistic(&MemoryInformationName::FreePhysicalRam)
			{
				Some(total_physical_ram - free_physical_ram)
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
	
	/// Used swap RAM in bytes.
	#[inline]
	pub fn used_swap(&self) -> Option<u64>
	{
		if let Some(total_swap) = self.get_statistic(&MemoryInformationName::TotalSwap)
		{
			if let Some(free_swap) = self.get_statistic(&MemoryInformationName::FreeSwap)
			{
				Some(total_swap - free_swap)
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
}
