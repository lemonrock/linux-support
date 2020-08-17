// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics.
///
/// All values are in bytes or simple counts.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryStatistics
{
	/// Amount of memory used in anonymous mappings such as `brk()`, `sbrk()` and `mmap(MAP_ANONYMOUS)`.
	pub anon: usize,
	
	/// Amount of memory used to cache filesystem data, including `tmpfs` and shared memory.
	pub file: usize,
	
	/// Amount of memory allocated to kernel stacks.
	pub kernel_stack: usize,
	
	/// Amount of memory used for storing in-kernel data structures.
	pub slab: usize,
	
	/// Amount of memory used in network transmission buffers.
	pub sock: usize,
	
	/// Amount of cached filesystem data that is swap-backed, such as `tmpfs`, `shm` segments, shared anonymous `mmap()`s.
	pub shmem: usize,
	
	/// Amount of cached filesystem data mapped with `mmap()`.
	pub file_mapped: usize,
	
	/// Amount of cached filesystem data that was modified but not yet written back to disk.
	pub file_dirty: usize,
	
	/// Amount of cached filesystem data that was modified and is currently being written back to disk.
	pub file_writeback: usize,
	
	/// Transparent Huge Page statistic, if kernel was compiled with support.
	pub anon_thp: Option<usize>,
	
	/// Amount of memory, swap-backed and filesystem-backed, on the internal memory management lists used by the page reclaim algorithm.
	pub inactive_anon: usize,
	
	/// Amount of memory, swap-backed and filesystem-backed, on the internal memory management lists used by the page reclaim algorithm.
	pub active_anon: usize,
	
	/// Amount of memory, swap-backed and filesystem-backed, on the internal memory management lists used by the page reclaim algorithm.
	pub inactive_file: usize,
	
	/// Amount of memory, swap-backed and filesystem-backed, on the internal memory management lists used by the page reclaim algorithm.
	pub active_file: usize,
	
	/// Amount of memory, swap-backed and filesystem-backed, on the internal memory management lists used by the page reclaim algorithm.
	pub unevictable: usize,
	
	/// Part of “slab” that might be reclaimed, such as dentries and inodes.
	pub slab_reclaimable: usize,
	
	/// Part of “slab” that cannot be reclaimed on memory pressure..
	pub slab_unreclaimable: usize,
	
	/// Total number of page faults incurred.
	pub pgfault: usize,
	
	/// Number of major page faults incurred.
	pub pgmajfault: usize,
	
	/// Number of refaults of previously evicted pages.
	pub workingset_refault: usize,
	
	/// Number of refaulted pages that were immediately activated.
	pub workingset_activate: usize,
	
	/// Number of times a shadow node has been reclaimed.
	pub workingset_nodereclaim: usize,
	
	/// Amount of scanned pages (in an active LRU list).
	pub pgrefill: usize,
	
	/// Amount of scanned pages (in an inactive LRU list).
	pub pgscan: usize,
	
	/// Amount of reclaimed pages.
	pub pgsteal: usize,
	
	/// Amount of pages moved to the active LRU list.
	pub pgactivate: usize,
	
	/// Amount of pages moved to the inactive LRU list.
	pub pgdeactivate: usize,
	
	/// Amount of pages postponed to be freed under memory pressure.
	pub pglazyfree: usize,
	
	/// Amount of reclaimed lazyfree pages.
	pub pglazyfreed: usize,
	
	/// Transparent Huge Page statistic, if kernel was compiled with support.
	pub thp_fault_alloc: Option<usize>,

	/// Transparent Huge Page statistic, if kernel was compiled with support.
	pub thp_collapse_alloc: Option<usize>,
}

impl MemoryStatistics
{
	#[inline(always)]
	pub(crate) fn from_file(file_path: &Path) -> Result<Self, StatisticsParseError>
	{
		let mut anon = None;
		let mut file = None;
		let mut kernel_stack = None;
		let mut slab = None;
		let mut sock = None;
		let mut shmem = None;
		let mut file_mapped = None;
		let mut file_dirty = None;
		let mut file_writeback = None;
		let mut anon_thp = None;
		let mut inactive_anon = None;
		let mut active_anon = None;
		let mut inactive_file = None;
		let mut active_file = None;
		let mut unevictable = None;
		let mut slab_reclaimable = None;
		let mut slab_unreclaimable = None;
		let mut pgfault = None;
		let mut pgmajfault = None;
		let mut workingset_refault = None;
		let mut workingset_activate = None;
		let mut workingset_nodereclaim = None;
		let mut pgrefill = None;
		let mut pgscan = None;
		let mut pgsteal = None;
		let mut pgactivate = None;
		let mut pgdeactivate = None;
		let mut pglazyfree = None;
		let mut pglazyfreed = None;
		let mut thp_fault_alloc = None;
		let mut thp_collapse_alloc = None;
		parse_key_value_statistics(file_path, &mut |name, value|
		{
			match name
			{
				b"anon" =>
				{
					anon = Some(value);
				}
				
				b"file" =>
				{
					file = Some(value);
				}
				
				b"kernel_stack" =>
				{
					kernel_stack = Some(value);
				}
				
				b"slab" =>
				{
					slab = Some(value);
				}
				
				b"sock" =>
				{
					sock = Some(value);
				}
				
				b"shmem" =>
				{
					shmem = Some(value);
				}
				
				b"file_mapped" =>
				{
					file_mapped = Some(value);
				}
				
				b"file_dirty" =>
				{
					file_dirty = Some(value);
				}
				
				b"file_writeback" =>
				{
					file_writeback = Some(value);
				}
				
				b"anon_thp" =>
				{
					anon_thp = Some(value);
				}
				
				b"inactive_anon" =>
				{
					inactive_anon = Some(value);
				}
				
				b"active_anon" =>
				{
					active_anon = Some(value);
				}
				
				b"inactive_file" =>
				{
					inactive_file = Some(value);
				}
				
				b"active_file" =>
				{
					active_file = Some(value);
				}
				
				b"unevictable" =>
				{
					unevictable = Some(value);
				}
				
				b"slab_reclaimable" =>
				{
					slab_reclaimable = Some(value);
				}
				
				b"slab_unreclaimable" =>
				{
					slab_unreclaimable = Some(value);
				}
				
				b"pgfault" =>
				{
					pgfault = Some(value);
				}
				
				b"pgmajfault" =>
				{
					pgmajfault = Some(value);
				}
				
				b"workingset_refault" =>
				{
					workingset_refault = Some(value);
				}
				
				b"workingset_activate" =>
				{
					workingset_activate = Some(value);
				}
				
				b"workingset_nodereclaim" =>
				{
					workingset_nodereclaim = Some(value);
				}
				
				b"pgrefill" =>
				{
					pgrefill = Some(value);
				}
				
				b"pgscan" =>
				{
					pgscan = Some(value);
				}
				
				b"pgsteal" =>
				{
					pgsteal = Some(value);
				}
				
				b"pgactivate" =>
				{
					pgactivate = Some(value);
				}
				
				b"pgdeactivate" =>
				{
					pgdeactivate = Some(value);
				}
				
				b"pglazyfree" =>
				{
					pglazyfree = Some(value);
				}
				
				b"pglazyfreed" =>
				{
					pglazyfreed = Some(value);
				}
				
				b"thp_fault_alloc" =>
				{
					thp_fault_alloc = Some(value);
				}
				
				b"thp_collapse_alloc" =>
				{
					thp_collapse_alloc = Some(value);
				}
				
				_ => (),
			};
			Ok(())
		})?;
		
		Ok
		(
			Self
			{
				anon: unwrap_statistic(anon, b"anon")?,
				file: unwrap_statistic(file, b"file")?,
				kernel_stack: unwrap_statistic(kernel_stack, b"kernel_stack")?,
				slab: unwrap_statistic(slab, b"slab")?,
				sock: unwrap_statistic(sock, b"sock")?,
				shmem: unwrap_statistic(shmem, b"shmem")?,
				file_mapped: unwrap_statistic(file_mapped, b"file_mapped")?,
				file_dirty: unwrap_statistic(file_dirty, b"file_dirty")?,
				file_writeback: unwrap_statistic(file_writeback, b"file_writeback")?,
				anon_thp,
				inactive_anon: unwrap_statistic(inactive_anon, b"inactive_anon")?,
				active_anon: unwrap_statistic(active_anon, b"active_anon")?,
				inactive_file: unwrap_statistic(inactive_file, b"inactive_file")?,
				active_file: unwrap_statistic(active_file, b"active_file")?,
				unevictable: unwrap_statistic(unevictable, b"unevictable")?,
				slab_reclaimable: unwrap_statistic(slab_reclaimable, b"slab_reclaimable")?,
				slab_unreclaimable: unwrap_statistic(slab_unreclaimable, b"slab_unreclaimable")?,
				pgfault: unwrap_statistic(pgfault, b"pgfault")?,
				pgmajfault: unwrap_statistic(pgmajfault, b"pgmajfault")?,
				workingset_refault: unwrap_statistic(workingset_refault, b"workingset_refault")?,
				workingset_activate: unwrap_statistic(workingset_activate, b"workingset_activate")?,
				workingset_nodereclaim: unwrap_statistic(workingset_nodereclaim, b"workingset_nodereclaim")?,
				pgrefill: unwrap_statistic(pgrefill, b"pgrefill")?,
				pgscan: unwrap_statistic(pgscan, b"pgscan")?,
				pgsteal: unwrap_statistic(pgsteal, b"pgsteal")?,
				pgactivate: unwrap_statistic(pgactivate, b"pgactivate")?,
				pgdeactivate: unwrap_statistic(pgdeactivate, b"pgdeactivate")?,
				pglazyfree: unwrap_statistic(pglazyfree, b"pglazyfree")?,
				pglazyfreed: unwrap_statistic(pglazyfreed, b"pglazyfreed")?,
				thp_fault_alloc,
				thp_collapse_alloc,
			}
		)
	}
}
