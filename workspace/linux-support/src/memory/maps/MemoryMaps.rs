// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is a parsing of the `/proc/<pid>/smaps` file, which is a superset of:-
///
/// * `/proc/<pid>/maps`, and,
/// * `/proc/<pid>/smaps_rollup`.
pub struct MemoryMaps
{
	maps: Vec<(MemoryMapEntry, MemoryMapEntryStatistics)>,
}

//		TODO: let numa_maps_lines = Self::open_file("numa_maps")?; - only reason to include this is per-NUMA node details.

impl MemoryMaps
{
	/// Calculated replacement for the parsing of `/proc/<pid>/smaps_rollup`.
	///
	/// Does not include the `Pss_Anon`, `Pss_File` or `Pss_Shmem` fields.
	#[inline(always)]
	pub fn roll_up(&self) -> Option<(Range<VirtualAddress>, MemoryMapEntryKilobyteStatistics)>
	{
		if unlikely!(self.maps.is_empty())
		{
			return None
		}

		let from = self.maps[0].0.memory_range.start;
		let to = self.maps[self.maps.len() - 1].0.memory_range.start;

		let mut total = self.maps[0].1.kilobyte_statistics.clone();
		for &(_, MemoryMapEntryStatistics { ref kilobyte_statistics, .. }) in &self.maps[1 .. ]
		{
			total += kilobyte_statistics
		}

		Some((from .. to, total))
	}

	/// Details for this process of `/proc/self/smaps` (which is more detailed than `/proc/self/maps`).
	#[inline(always)]
	pub fn maps_for_self(proc_path: &ProcPath) -> Result<Self, MemoryMapParseError>
	{
		Self::maps_for_process(proc_path, ProcessIdentifierChoice::Current)
	}

	/// Details for a particular process of `/proc/<process_identifier>/smaps` (which is more detailed than `/proc/<process_identifier>/maps`).
	#[inline(always)]
	pub fn maps_for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Result<Self, MemoryMapParseError>
	{
		let mut lines = Self::open_file(proc_path, process_identifier, "smaps")?.unwrap();

		let mut parse_state = ParseState::default();
		let mut maps = Vec::new();

		loop
		{
			let map_line = lines.next();
			if map_line.is_none()
			{
				break
			}

			let memory_map_entry = MemoryMapEntry::parse_maps_line(&mut parse_state, map_line.unwrap()?)?;
			let memory_map_entry_statistics = MemoryMapEntryStatistics::parse_statistics_lines(&mut lines, memory_map_entry.memory_range.clone(), memory_map_entry.protection, memory_map_entry.sharing)?;
			maps.push((memory_map_entry, memory_map_entry_statistics))
		}

		parse_state.validate()?;
		Ok
		(
			Self
			{
				maps,
			}
		)
	}

	#[inline(always)]
	fn open_file(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice, file_name: &str) -> Result<Option<impl Iterator<Item=Result<(usize, Vec<u8>), MemoryMapParseError>>>, MemoryMapParseError>
	{
		let file_path = proc_path.process_file_path(process_identifier, file_name);
		if file_path.exists()
		{
			let file = File::open(file_path).map_err(|error| CouldNotOpenFile(error))?;
			let split = BufReader::new(file).split(b'\n');
			let enumerate = split.enumerate();
			let map = enumerate.map(|(zero_based_line_number, result)| result.map(|line| (zero_based_line_number, line)).map_err(|cause| CouldNotReadLine { zero_based_line_number, cause }));
			Ok(Some(map))
		}
		else
		{
			Ok(None)
		}
	}

}
