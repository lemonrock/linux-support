// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A list of known virtual memory statistics related to NUMA nodes.
///
/// All of these statistics were in existence before Linux 4.0 unless indicated otherwise.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VirtualMemoryStatisticName
{
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
	NumberOfPagesScanned,

	NumberOfAnonymousTransparentHugePages,
	NumberOfFreeContiguousMemoryAllocatorPages,
	
	/// Found in `/sys/devices/system/node/node<X>/vmstat` and `/sys/devices/system/node/node<X>/numastat` where `<X>` is a zero-based NUMA node number.
	NumaHit,
	NumaMiss,
	NumaForeign,
	NumaInterleaveHit,
	NumaLocalNode,
	NumaOtherNode,

	/// The amount of base pages that were marked for NUMA hinting faults.
	///
	/// Used for Automatic NUMA Balancing (see <https://documentation.suse.com/sles/15-SP1/html/SLES-all/cha-tuning-numactl.html> and <https://lkml.org/lkml/2012/12/7/106>).
	numa_pte_updates,

	/// The amount of transparent huge pages that were marked for NUMA hinting faults.
	///
	/// In combination with `numa_pte_updates` the total address space that was marked can be calculated.
	///
	/// Used for Automatic NUMA Balancing (see <https://documentation.suse.com/sles/15-SP1/html/SLES-all/cha-tuning-numactl.html> and <https://lkml.org/lkml/2012/12/7/106>).
	numa_huge_pte_updates,

	/// Records how many NUMA hinting faults were trapped.
	///
	/// Used for Automatic NUMA Balancing (see <https://documentation.suse.com/sles/15-SP1/html/SLES-all/cha-tuning-numactl.html> and <https://lkml.org/lkml/2012/12/7/106>).
	numa_hint_faults,

	/// Shows how many of the hinting faults were to local nodes.
	///
	/// In combination with `numa_hint_faults`, the percentage of local versus remote faults can be calculated.
	/// A high percentage of local hinting faults indicates that the workload is closer to being converged.
	///
	/// Used for Automatic NUMA Balancing (see <https://documentation.suse.com/sles/15-SP1/html/SLES-all/cha-tuning-numactl.html> and <https://lkml.org/lkml/2012/12/7/106>).
	numa_hint_faults_local,

	/// Records how many pages were migrated because they were misplaced.
	///
	/// As migration is a copying operation, it contributes the largest part of the overhead created by NUMA balancing.
	///
	/// Used for Automatic NUMA Balancing (see <https://documentation.suse.com/sles/15-SP1/html/SLES-all/cha-tuning-numactl.html> and <https://lkml.org/lkml/2012/12/7/106>).
	numa_pages_migrated,

	/// kcompactd related.
	compact_migrate_scanned,
	compact_free_scanned,
	compact_isolated,

	/// Transparent Huge Page related.
	compact_stall,
	compact_fail,
	compact_success,
	thp_fault_alloc,
	thp_fault_fallback,
	thp_collapse_alloc,
	thp_collapse_alloc_failed,
	thp_split,
	thp_zero_page_alloc,
	thp_zero_page_alloc_failed,

	htlb_buddy_alloc_success,
	htlb_buddy_alloc_fail,

	workingset_refault,
	workingset_activate,
	workingset_nodereclaim,
	nr_dirty_threshold,
	nr_dirty_background_threshold,
	pgpgin,
	pgpgout,
	pswpin,
	pswpout,
	pgalloc_dma,
	pgalloc_dma32,
	pgalloc_normal,
	pgalloc_high,
	pgalloc_movable,
	pgfree,
	pgactivate,
	pgdeactivate,
	pgfault,
	pgmajfault,
	pgrefill_dma,
	pgrefill_dma32,
	pgrefill_normal,
	pgrefill_high,
	pgrefill_movable,
	pgsteal_kswapd_dma,
	pgsteal_kswapd_dma32,
	pgsteal_kswapd_normal,
	pgsteal_kswapd_high,
	pgsteal_kswapd_movable,
	pgsteal_direct_dma,
	pgsteal_direct_dma32,
	pgsteal_direct_normal,
	pgsteal_direct_high,
	pgsteal_direct_movable,
	pgscan_kswapd_dma,
	pgscan_kswapd_dma32,
	pgscan_kswapd_normal,
	pgscan_kswapd_high,
	pgscan_kswapd_movable,
	pgscan_direct_dma,
	pgscan_direct_dma32,
	pgscan_direct_normal,
	pgscan_direct_high,
	pgscan_direct_movable,
	pgscan_direct_throttle,
	zone_reclaim_failed,
	pginodesteal,
	slabs_scanned,
	kswapd_inodesteal,
	kswapd_low_wmark_hit_quickly,
	kswapd_high_wmark_hit_quickly,
	pageoutrun,
	allocstall,
	pgrotated,
	drop_pagecache,
	drop_slab,
	pgmigrate_success,
	pgmigrate_fail,
	unevictable_pgs_culled,
	unevictable_pgs_scanned,
	unevictable_pgs_rescued,
	unevictable_pgs_mlocked,
	unevictable_pgs_munlocked,
	unevictable_pgs_cleared,
	unevictable_pgs_stranded,
	balloon_inflate,
	balloon_deflate,
	balloon_migrate,
	nr_tlb_remote_flush,
	nr_tlb_remote_flush_received,
	nr_tlb_local_flush_all,
	nr_tlb_local_flush_one,
	vmacache_find_calls,
	vmacache_find_hits,
	vmacache_full_flushes,
	
	/// This is for all other possibilities.
	Unknown(Box<[u8]>),
}

/*
*/

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
		let reader = file_path.read_raw()?;

		let mut statistics = HashMap::with_capacity(6);
		let mut zero_based_line_number = 0;

		for line in reader.split_bytes(b'\n')
		{
			use self::ErrorKind::InvalidData;

			let mut split = line.split_bytes_n(2, b' ');

			let statistic_name = VirtualMemoryStatisticName::parse(split.next().unwrap());

			let statistic_value = match split.next()
			{
				None => return Err(io::Error::new(InvalidData, format!("Zero based line '{}' does not have a value second column", zero_based_line_number))),
				Some(value) => u64::parse_decimal_number(value).map_err(|parse_number_error| io::Error::new(InvalidData, parse_number_error))?,
			};

			if let Some(previous) = statistics.insert(statistic_name, statistic_value)
			{
				return Err(io::Error::new(InvalidData, format!("Zero based line '{}' has a duplicate statistic (was '{}')", zero_based_line_number, previous)))
			}

			zero_based_line_number += 1;
		}

		Ok(statistics)
	}
	
	#[inline(always)]
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
			b"nr_pages_scanned" => NumberOfPagesScanned,


			b"nr_anon_transparent_hugepages" => NumberOfAnonymousTransparentHugePages,
			b"nr_free_cma" => NumberOfFreeContiguousMemoryAllocatorPages,

			// found in '/sys/devices/system/node/nodeX/vmstat' and '/sys/devices/system/node/nodeX/numastat'
			b"numa_hit" => NumaHit,
			b"numa_miss" => NumaMiss,
			b"numa_foreign" => NumaForeign,
			b"interleave_hit" => NumaInterleaveHit,
			b"local_node" => NumaLocalNode,
			b"other_node" => NumaOtherNode,

			b"numa_pte_updates" => numa_pte_updates,
			b"numa_huge_pte_updates" => numa_huge_pte_updates,
			b"numa_hint_faults" => numa_hint_faults,
			b"numa_hint_faults_local" => numa_hint_faults_local,
			b"numa_pages_migrated" => numa_pages_migrated,

			b"compact_migrate_scanned" => compact_migrate_scanned,
			b"compact_free_scanned" => compact_free_scanned,
			b"compact_isolated" => compact_isolated,

			b"compact_stall" => compact_stall,
			b"compact_fail" => compact_fail,
			b"compact_success" => compact_success,
			b"thp_fault_alloc" => thp_fault_alloc,
			b"thp_fault_fallback" => thp_fault_fallback,
			b"thp_collapse_alloc" => thp_collapse_alloc,
			b"thp_collapse_alloc_failed" => thp_collapse_alloc_failed,
			b"thp_split" => thp_split,
			b"thp_zero_page_alloc" => thp_zero_page_alloc,
			b"thp_zero_page_alloc_failed" => thp_zero_page_alloc_failed,

			b"htlb_buddy_alloc_success" => htlb_buddy_alloc_success,
			b"htlb_buddy_alloc_fail" => htlb_buddy_alloc_fail,

			b"workingset_refault" => workingset_refault,
			b"workingset_activate" => workingset_activate,
			b"workingset_nodereclaim" => workingset_nodereclaim,
			b"nr_dirty_threshold" => nr_dirty_threshold,
			b"nr_dirty_background_threshold" => nr_dirty_background_threshold,
			b"pgpgin" => pgpgin,
			b"pgpgout" => pgpgout,
			b"pswpin" => pswpin,
			b"pswpout" => pswpout,
			b"pgalloc_dma" => pgalloc_dma,
			b"pgalloc_dma32" => pgalloc_dma32,
			b"pgalloc_normal" => pgalloc_normal,
			b"pgalloc_high" => pgalloc_high,
			b"pgalloc_movable" => pgalloc_movable,
			b"pgfree" => pgfree,
			b"pgactivate" => pgactivate,
			b"pgdeactivate" => pgdeactivate,
			b"pgfault" => pgfault,
			b"pgmajfault" => pgmajfault,
			b"pgrefill_dma" => pgrefill_dma,
			b"pgrefill_dma32" => pgrefill_dma32,
			b"pgrefill_normal" => pgrefill_normal,
			b"pgrefill_high" => pgrefill_high,
			b"pgrefill_movable" => pgrefill_movable,
			b"pgsteal_kswapd_dma" => pgsteal_kswapd_dma,
			b"pgsteal_kswapd_dma32" => pgsteal_kswapd_dma32,
			b"pgsteal_kswapd_normal" => pgsteal_kswapd_normal,
			b"pgsteal_kswapd_high" => pgsteal_kswapd_high,
			b"pgsteal_kswapd_movable" => pgsteal_kswapd_movable,
			b"pgsteal_direct_dma" => pgsteal_direct_dma,
			b"pgsteal_direct_dma32" => pgsteal_direct_dma32,
			b"pgsteal_direct_normal" => pgsteal_direct_normal,
			b"pgsteal_direct_high" => pgsteal_direct_high,
			b"pgsteal_direct_movable" => pgsteal_direct_movable,
			b"pgscan_kswapd_dma" => pgscan_kswapd_dma,
			b"pgscan_kswapd_dma32" => pgscan_kswapd_dma32,
			b"pgscan_kswapd_normal" => pgscan_kswapd_normal,
			b"pgscan_kswapd_high" => pgscan_kswapd_high,
			b"pgscan_kswapd_movable" => pgscan_kswapd_movable,
			b"pgscan_direct_dma" => pgscan_direct_dma,
			b"pgscan_direct_dma32" => pgscan_direct_dma32,
			b"pgscan_direct_normal" => pgscan_direct_normal,
			b"pgscan_direct_high" => pgscan_direct_high,
			b"pgscan_direct_movable" => pgscan_direct_movable,
			b"pgscan_direct_throttle" => pgscan_direct_throttle,
			b"zone_reclaim_failed" => zone_reclaim_failed,
			b"pginodesteal" => pginodesteal,
			b"slabs_scanned" => slabs_scanned,
			b"kswapd_inodesteal" => kswapd_inodesteal,
			b"kswapd_low_wmark_hit_quickly" => kswapd_low_wmark_hit_quickly,
			b"kswapd_high_wmark_hit_quickly" => kswapd_high_wmark_hit_quickly,
			b"pageoutrun" => pageoutrun,
			b"allocstall" => allocstall,
			b"pgrotated" => pgrotated,
			b"drop_pagecache" => drop_pagecache,
			b"drop_slab" => drop_slab,
			b"pgmigrate_success" => pgmigrate_success,
			b"pgmigrate_fail" => pgmigrate_fail,
			b"unevictable_pgs_culled" => unevictable_pgs_culled,
			b"unevictable_pgs_scanned" => unevictable_pgs_scanned,
			b"unevictable_pgs_rescued" => unevictable_pgs_rescued,
			b"unevictable_pgs_mlocked" => unevictable_pgs_mlocked,
			b"unevictable_pgs_munlocked" => unevictable_pgs_munlocked,
			b"unevictable_pgs_cleared" => unevictable_pgs_cleared,
			b"unevictable_pgs_stranded" => unevictable_pgs_stranded,
			b"balloon_inflate" => balloon_inflate,
			b"balloon_deflate" => balloon_deflate,
			b"balloon_migrate" => balloon_migrate,
			b"nr_tlb_remote_flush" => nr_tlb_remote_flush,
			b"nr_tlb_remote_flush_received" => nr_tlb_remote_flush_received,
			b"nr_tlb_local_flush_all" => nr_tlb_local_flush_all,
			b"nr_tlb_local_flush_one" => nr_tlb_local_flush_one,
			b"vmacache_find_calls" => vmacache_find_calls,
			b"vmacache_find_hits" => vmacache_find_hits,
			b"vmacache_full_flushes" => vmacache_full_flushes,

			other @ _ => Unknown(other.to_vec().into_boxed_slice()),
		}
	}
}
