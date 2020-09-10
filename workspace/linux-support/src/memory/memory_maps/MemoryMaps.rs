// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is a parsing of the `/proc/<pid>/smaps` file, which is a superset of:-
///
/// * `/proc/<pid>/maps`, and,
/// * `/proc/<pid>/smaps_rollup`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct MemoryMaps(Vec<(MemoryMapEntry, MemoryMapEntryStatistics)>);

impl MemoryMaps
{
	/// Calculated replacement for the parsing of `/proc/<pid>/smaps_rollup`.
	///
	/// Does not include the `Pss_Anon`, `Pss_File` or `Pss_Shmem` fields.
	#[inline(always)]
	pub fn smaps_roll_up(&self) -> Option<(Range<VirtualAddress>, MemoryMapEntryKilobyteStatistics)>
	{
		if unlikely!(self.maps().is_empty())
		{
			return None
		}

		let from = self.maps()[0].0.memory_range.start;
		let to = self.maps()[self.maps().len() - 1].0.memory_range.start;

		let mut total = self.maps()[0].1.kilobyte_statistics.clone();
		for &(_, MemoryMapEntryStatistics { ref kilobyte_statistics, .. }) in &self.maps()[1 .. ]
		{
			total += kilobyte_statistics
		}

		Some((from .. to, total))
	}
	
	#[inline(always)]
	fn maps(&self) -> &Vec<(MemoryMapEntry, MemoryMapEntryStatistics)>
	{
		&self.0
	}

	/// Details for this process of `/proc/self/smaps` (which is more detailed than `/proc/self/maps`).
	///
	/// Use `NumaNodes::have_movable_memory()` for `have_movable_memory`; it will be `None` if the Linux kernel is not-NUMA aware.
	#[inline(always)]
	pub fn smaps_for_self(proc_path: &ProcPath, have_movable_memory: Option<&NumaNodes>) -> Result<Self, MemoryMapParseError>
	{
		Self::smaps_for_process(proc_path, ProcessIdentifierChoice::Current, have_movable_memory)
	}

	/// Details for a particular process of `/proc/<process_identifier>/smaps` (which is more detailed than `/proc/<process_identifier>/maps`).
	///
	/// Use `NumaNodes::have_movable_memory()` for `have_movable_memory`; it will be `None` if the Linux kernel is not-NUMA aware.
	#[inline(always)]
	pub fn smaps_for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice, have_movable_memory: Option<&NumaNodes>) -> Result<Self, MemoryMapParseError>
	{
		match have_movable_memory
		{
			None => Self::smaps_for_process_non_numa(proc_path, process_identifier),
			
			Some(have_movable_memory) => match Self::open_file(proc_path, process_identifier, "numa_maps")?
			{
				None => Self::smaps_for_process_non_numa(proc_path, process_identifier),
				
				Some(reader) =>
				{
					let split = reader.split_bytes(b'\n');
					let mut numa_maps_lines = split.enumerate();
					
					let (parse_state, maps) = Self::maps_for_process_loop(proc_path, process_identifier, |memory_range, kind|
					{
						Ok
						(
							Some
							(
								MemoryMapEntry::parse_numa_maps_line
								(
									numa_maps_lines.next().ok_or(Mismatched { explanation: "Not enough lines in numa_maps" })?,
									memory_range.start,
									kind,
									have_movable_memory
								)?
							)
						)
					})?;
					
					parse_state.validate()?;
					
					if unlikely!(numa_maps_lines.next().is_some())
					{
						return Err(Mismatched { explanation: "Too many lines in numa_maps" })
					}
					
					Ok(Self(maps))
				}
			}
		}
	}
	
	#[inline(always)]
	fn smaps_for_process_non_numa(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Result<Self, MemoryMapParseError>
	{
		let (parse_state, maps) = Self::maps_for_process_loop(proc_path, process_identifier, |_, _| Ok(None))?;
		
		parse_state.validate()?;
		
		Ok(Self(maps))
	}

	#[inline(always)]
	fn maps_for_process_loop(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice, mut with_numa_data: impl FnMut(&Range<VirtualAddress>, &MemoryMapEntryKind) -> Result<Option<(NumaMemoryPolicyDetails, Option<PageCounts>)>, MemoryMapParseError>) -> Result<(ParseState, Vec<(MemoryMapEntry, MemoryMapEntryStatistics)>), MemoryMapParseError>
	{
		let reader = Self::open_file(proc_path, process_identifier, "smaps")?.ok_or(SmapsFileDoesNotExist)?;
		let split = reader.split_bytes(b'\n');
		let mut smaps_lines = split.enumerate();
		
		let mut parse_state = ParseState::default();
		let mut maps = Vec::new();

		loop
		{
			let (zero_based_line_number, map_line) =
			{
				match smaps_lines.next()
				{
					None => break,
					Some(value) => value,
				}
			};

			let memory_map_entry = MemoryMapEntry::parse_maps_line(&mut parse_state, (zero_based_line_number, map_line), &mut with_numa_data)?;

			let memory_map_entry_statistics = MemoryMapEntryStatistics::parse_statistics_lines(&mut smaps_lines, memory_map_entry.memory_range.clone(), memory_map_entry.protection, memory_map_entry.sharing)?;

			maps.push((memory_map_entry, memory_map_entry_statistics))
		}

		Ok((parse_state, maps))
	}

	#[inline(always)]
	fn open_file(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice, file_name: &str) -> Result<Option<Box<[u8]>>, MemoryMapParseError>
	{
		let file_path = proc_path.process_file_path(process_identifier, file_name);
		if file_path.exists()
		{
			Ok(Some(file_path.read_raw().map_err(CouldNotOpenFile)?))
		}
		else
		{
			Ok(None)
		}
	}

}
