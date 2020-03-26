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

		for line in reader.split(b'\n')
		{
			let mut line = line?;

			{
				let mut split = splitn(&line, 2, b':');

				let memory_information_name = MemoryInformationName::parse(split.next().unwrap(), memory_information_name_prefix);

				let memory_information_value = match split.next()
				{
					None => return Err(MemoryInformationParseError::CouldNotParseMemoryInformationValue { zero_based_line_number, memory_information_name }),
					Some(raw_value) =>
					{
						let str_value = match from_utf8(raw_value)
						{
							Err(utf8_error) => return Err(CouldNotParseAsUtf8 { zero_based_line_number, memory_information_name, bad_value: raw_value.to_vec().into_boxed_slice(), cause: utf8_error }),
							Ok(str_value) => str_value,
						};

						let trimmed_str_value = str_value.trim();
						let ends_with = memory_information_name.unit().ends_with();

						if !trimmed_str_value.ends_with(ends_with)
						{
							return Err(CouldNotParseMemoryInformationValueTrimmed { zero_based_line_number, memory_information_name, bad_value: trimmed_str_value.to_owned() });
						}

						let trimmed = &trimmed_str_value[0 .. trimmed_str_value.len() - ends_with.len()];

						match trimmed.parse::<u64>()
						{
							Ok(value) => value,
							Err(int_parse_error) => return Err(CouldNotParseMemoryInformationValueAsU64 { zero_based_line_number, memory_information_name, bad_value: trimmed.to_owned(), cause: int_parse_error })
						}
					}
				};

				if map.contains_key(&memory_information_name)
				{
					return Err(DuplicateMemoryInformation { zero_based_line_number, memory_information_name, new_value: memory_information_value });
				}

				map.insert(memory_information_name, memory_information_value);
			}

			line.clear();
			zero_based_line_number += 1;
		}

		Ok(MemoryInformation(map))
	}
}
