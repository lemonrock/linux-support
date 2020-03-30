// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A list of known virtual memory statistics related to NUMA nodes.
///
/// There are far more statistics than those listed here.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VirtualMemoryStatisticName
{
	/// Found in `/sys/devices/system/node/node<X>/vmstat` where `<X>` is a zero-based NUMA node number.
	NumberOFreePages,
	NumberOfBatchAllocatedPages,
	NumberOfInactiveAnonymousPages,
	NumberOfActiveAnonymousPages,
	NumberOfInactiveFilePages,
	NumberOfActiveFilePages,
	NumberOfUnevictablePages,
	NumberOfLockedPages,
	NumberOfAnonymousPages,
	NumberOfMappedPages,
	NumberOfFilePages,
	NumberOfDirtyPages,
	NumberOfWritebackPages,
	NumberOfReclaimableSlabPages,
	NumberOfUnreclaimableSlabPages,
	NumberOfPageTablePages,
	NumberOfKernelStackPages,
	NumberOfUnstablePages,
	NumberOfBouncePages,
	NumberOfVirtualMemoryWritePages,
	NumberOfVirtualMemoryImmediateReclaimPages,
	NumberOfWritebackTemporaryPages,
	NumberOfIsolatedAnonymousPages,
	NumberOfIsolatedFilePages,
	NumberOfShmemPages,
	NumberOfDirtiedPages,
	NumberOfWrittenPages,
	NumberOfAnonymousTransparentHugePages,
	NumberOfFreeCmaPages,
	
	/// Found in `/sys/devices/system/node/node<X>/vmstat` and `/sys/devices/system/node/node<X>/numastat` where `<X>` is a zero-based NUMA node number.
	NumaHit,
	NumaMiss,
	NumaForeign,
	NumaInterleaveHit,
	NumaLocalNode,
	NumaOtherNode,
	
	/// This is for all other possibilities.
	Unknown(Box<[u8]>),
}

impl VirtualMemoryStatisticName
{
	/// Memory statistics (from `/proc/vmstat`).
	///
	/// Interpret this by multiplying counts by page size.
	///
	/// For NUMA node specific information, see `NumaNode.numa_memory_statistics()` and `NumaNode.zoned_virtual_memory_statistics()`.
	#[inline(always)]
	pub fn global_zoned_virtual_memory_statistics(proc_path: &ProcPath) -> io::Result<HashMap<Self, u64>>
	{
		let file_path = proc_path.file_path("vmstat");
		Self::parse_virtual_memory_statistics_file(&file_path)
	}

	#[inline(always)]
	pub(crate) fn parse_virtual_memory_statistics_file(file_path: &Path) -> io::Result<HashMap<Self, u64>>
	{
		let file = File::open(file_path)?;

		let reader = BufReader::with_capacity(4096, file);

		let mut statistics = HashMap::with_capacity(6);
		let mut zero_based_line_number = 0;

		for line in reader.split(b'\n')
		{
			let mut line = line?;

			{
				use self::ErrorKind::InvalidData;

				let mut split = line.splitn(2, |byte| *byte == b' ');

				let statistic_name = VirtualMemoryStatisticName::parse(split.next().unwrap());

				let statistic_value = match split.next()
				{
					None => return Err(io::Error::new(InvalidData, format!("Zero based line '{}' does not have a value second column", zero_based_line_number))),
					Some(value) =>
					{
						let str_value = match from_utf8(value)
						{
							Err(utf8_error) => return Err(io::Error::new(InvalidData, utf8_error)),
							Ok(str_value) => str_value,
						};

						match str_value.parse::<u64>()
						{
							Err(parse_error) => return Err(io::Error::new(InvalidData, parse_error)),
							Ok(value) => value,
						}
					}
				};

				if let Some(previous) = statistics.insert(statistic_name, statistic_value)
				{
					return Err(io::Error::new(InvalidData, format!("Zero based line '{}' has a duplicate statistic (was '{}')", zero_based_line_number, previous)))
				}
			}

			line.clear();
			zero_based_line_number += 1;
		}

		Ok(statistics)
	}

	#[inline]
	pub(crate) fn parse(name: &[u8]) -> Self
	{
		use self::VirtualMemoryStatisticName::*;
		
		match name
		{
			b"nr_free_pages" => NumberOFreePages,
			b"nr_alloc_batch" => NumberOfBatchAllocatedPages,
			b"nr_inactive_anon" => NumberOfInactiveAnonymousPages,
			b"nr_active_anon" => NumberOfActiveAnonymousPages,
			b"nr_inactive_file" => NumberOfInactiveFilePages,
			b"nr_active_file" => NumberOfActiveFilePages,
			b"nr_unevictable" => NumberOfUnevictablePages,
			b"nr_mlock" => NumberOfLockedPages,
			b"nr_anon_pages" => NumberOfAnonymousPages,
			b"nr_mapped" => NumberOfMappedPages,
			b"nr_file_pages" => NumberOfFilePages,
			b"nr_dirty" => NumberOfDirtyPages,
			b"nr_writeback" => NumberOfWritebackPages,
			b"nr_slab_reclaimable" => NumberOfReclaimableSlabPages,
			b"nr_slab_unreclaimable" => NumberOfUnreclaimableSlabPages,
			b"nr_page_table_pages" => NumberOfPageTablePages,
			b"nr_kernel_stack" => NumberOfKernelStackPages,
			b"nr_unstable" => NumberOfUnstablePages,
			b"nr_bounce" => NumberOfBouncePages,
			b"nr_vmscan_write" => NumberOfVirtualMemoryWritePages,
			b"nr_vmscan_immediate_reclaim" => NumberOfVirtualMemoryImmediateReclaimPages,
			b"nr_writeback_temp" => NumberOfWritebackTemporaryPages,
			b"nr_isolated_anon" => NumberOfIsolatedAnonymousPages,
			b"nr_isolated_file" => NumberOfIsolatedFilePages,
			b"nr_shmem" => NumberOfShmemPages,
			b"nr_dirtied" => NumberOfDirtiedPages,
			b"nr_written" => NumberOfWrittenPages,
			b"nr_anon_transparent_hugepages" => NumberOfAnonymousTransparentHugePages,
			b"nr_free_cma" => NumberOfFreeCmaPages,

			// found in '/sys/devices/system/node/nodeX/vmstat' and '/sys/devices/system/node/nodeX/numastat'
			b"numa_hit" => NumaHit,
			b"numa_miss" => NumaMiss,
			b"numa_foreign" => NumaForeign,
			b"interleave_hit" => NumaInterleaveHit,
			b"local_node" => NumaLocalNode,
			b"other_node" => NumaOtherNode,
			
			other @ _ => Unknown(other.to_vec().into_boxed_slice()),
		}
	}
}
