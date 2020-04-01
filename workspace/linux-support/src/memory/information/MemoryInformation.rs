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
	///
	/// For NUMA node specific memory information, see `NumaNode.memory_information()`.
	#[inline(always)]
	pub fn parse(proc_path: &ProcPath, memory_information_name_prefix: &[u8]) -> Result<Self, MemoryInformationParseError>
	{
		let file_path = proc_path.file_path("meminfo");
		Self::parse_memory_information_file(&file_path, memory_information_name_prefix)
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

	/// Parses the `meminfo` file.
	pub(crate) fn parse_memory_information_file(file_path: &Path, memory_information_name_prefix: &[u8]) -> Result<MemoryInformation, MemoryInformationParseError>
	{
		let reader = BufReader::with_capacity(4096, File::open(file_path)?);

		let mut map = HashMap::new();
		let mut zero_based_line_number = 0;

		use self::MemoryInformationParseError::*;

		// Lines such as:-
		// * `SwapTotal:       2021372 kB`
		// * `VmallocTotal:   34359738367 kB`
		for line in reader.split(b'\n')
		{
			let line = line?;

			let mut split = line.splitn(2, |byte| *byte == b':');

			let memory_information_name = MemoryInformationName::parse(split.next().unwrap(), memory_information_name_prefix);

			if map.contains_key(&memory_information_name)
			{
				return Err(DuplicateMemoryInformation { zero_based_line_number });
			}

			let raw_value = split.next().ok_or(NoValue { zero_based_line_number })?;
			let bytes = memory_information_name.validate_unit(raw_value, zero_based_line_number)?;

			let mut after_whitespace_index = 0;
			while bytes[after_whitespace_index] == b' '
			{
				after_whitespace_index += 1;
				if unlikely!(after_whitespace_index == raw_value.len())
				{
					return Err(TooMuchWhitespace { zero_based_line_number })
				}
			}
			let bytes = &bytes[after_whitespace_index .. ];

			let memory_information_value = u64::parse_decimal_number(bytes).map_err(|cause| BadValue { zero_based_line_number, cause })?;

			map.insert(memory_information_name, memory_information_value);

			zero_based_line_number += 1;
		}

		Ok(MemoryInformation(map))
	}
}
