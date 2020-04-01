// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory information names for a process.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryInformationName
{
	/// Total usable RAM (physical RAM minus a few reserved bits and the kernel binary code).
	TotalPhysicalRam,

	/// Free.
	FreePhysicalRam,

	/// An estimate of physical ram available for starting new applications.
	///
	/// Always larger than `FreePhysicalRam`; see <https://git.kernel.org/cgit/linux/kernel/git/torvalds/linux.git/commit/?id=34e431b0ae398fc54ea69ff85ec700722c9da773>.
	///
	/// Since Linux 3.14.
	AvailablePhysicalRam,

	/// Relatively temporary storage for raw disk blocks that shouldn't get tremendously large (20MB or so).
	UsedAsFileBuffersPhysicalRam,

	/// In-memory cache for files read from the disk (the page cache).
	///
	/// Doesn't include `UsedAsCacheSwap`.
	UsedAsCachePhysicalRam,

	/// Total amount of swap space available.
	TotalSwap,

	/// Amount of swap space that is currently unused.
	FreeSwap,

	/// Shows the amount of memory marked by `madvise()'s `MADV_FREE`.
	///
	/// Since Linux 4.12.
	LazyFree,

	/// Memory that once was swapped out, is swapped back in but still also is in the swap file.
	///
	/// (If memory pressure is high, these pages don't need to be swapped out again because they are already in the swap file.
	/// This saves I/O).
	UsedAsCacheSwap,

	/// Memory that has been used more recently and usually not reclaimed unless absolutely necessary.
	ActiveFileBufferAndCacheInUse,

	/// Memory which has been less recently used.
	///
	/// It is more eligible to be reclaimed for other purposes.
	InactiveFileBufferAndCacheAvailable,

	/// No documentation.
	///
	/// Since Linux 2.6.28.
	AnonymousActive,

	/// No documentation.
	///
	/// Since Linux 2.6.28.
	AnonymousInactive,

	/// No documentation.
	///
	/// Since Linux 2.6.28.
	FileActive,

	/// No documentation.
	///
	/// Since Linux 2.6.28.
	FileInactive,

	/// No documentation.
	///
	/// Since Linux 2.6.28.
	Unevictable,

	/// Memory which is waiting to get written back to disk.
	///
	/// aka 'Dirty'.
	WaitingToBeWrittenBackToDisks,

	/// Memory which is actively being written back to disk.
	///
	/// aka 'Writeback'.
	CurrentlyBeingWrittenBackToDisks,

	/// Memory used for block device "bounce buffers".
	///
	/// Since Linux 2.6.18.
	UsedForBlockDeviceBounceBuffers,

	/// Since Linux 2.6.18.
	NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage,

	/// Memory used by FUSE for temporary writeback buffers.
	///
	/// Since Linux 2.6.26.
	MemoryUsedByFuseForTemporaryWritebackBuffers,

	/// Since Linux 2.6.18.
	AnonymousMemoryMappedUsingMmap,

	/// Files which have been mapped into memory (with `mmap()`), such as libraries.
	FilesMappedUsingMmap,

	/// Amount of memory consumed in tmpfs filesystems.
	///
	/// Linux 2.6.32.
	Shmem,

	/// No documentation.
	///
	/// Since Linux 2.6.28.
	LockedByMlock,

	/// In-kernel data structures cache.
	Slab,

	/// Part of `Slab`, that might be reclaimed, such as caches.
	///
	/// Since Linux 2.6.19.
	SlabReclaimable,

	/// Part of `Slab` that cannot be reclaimed on memory pressure.
	///
	/// Since Linux 2.6.19.
	SlabUnreclaimable,

	/// Kernel allocations that the kernel will attempt to reclaim under memory pressure.
	///
	/// Includes `SlabReclaimable` and other direct allocations with a shrinker.
	///
	/// Since Linux 4.20.
	KReclaimable,

	/// Amount of memory allocated to kernel stacks.
	///
	/// Since Linux 2.6.32.
	KernelStack,

	/// Amount of memory dedicated to the lowest level of page tables.
	///
	/// Since Linux 2.6.18.
	MemoryDedicatedToLowestPageTableLevel,

	/// This is the total amount of memory currently available  to be allocated on the system.
	///
	/// This limit is adhered to only if strict overcommit accounting is enabled (mode 2 in `/proc/sys/vm/overcommit_memory`).
	/// The limit is calculated according to the formula described under `/proc/sys/vm/overcommit_memory`.
	///
	/// For further details, see the kernel source file `Documentation/vm/overcommit-accounting.rst`.
	///
	/// Since Linux 2.6.10.
	CommitLimit,

	/// The amount of memory presently allocated on the system.
	///
	/// The committed memory is a sum of all of the memory which has been allocated by processes, even if it has not been "used" by them as of yet.
	/// A process which allocates 1GB of memory (using `malloc()` or similar),  but touches only 300MB of that memory will show up as using only 300MB of memory even if it has the address space allocated for the entire 1GB.
	///
	/// This 1GB is memory which has been "committed" to by the VM and can be used at any time by the allocating application.
	/// With strict overcommit enabled on the system (mode 2 in `/proc/sys/vm/overcommit_memory`), allocations which would exceed the `CommitLimit` will not be permitted.
	/// This is useful if one needs to guarantee that processes will not fail due to lack of memory once that memory has been successfully allocated.
	WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,

	/// Total size of vmalloc memory area.
	TotalVirtualAddressSpace,

	/// Amount of vmalloc area which is used.
	///
	/// Since Linux 4.4, this field is no longer calculated, and is always 0.
	///
	/// See `/proc/vmallocinfo`.
	#[deprecated]
	UsedVirtualAddressSpace,

	/// Largest contiguous block of vmalloc area which is free.
	///
	/// Since Linux 4.4, this field is no longer calculated and is hard coded as 0.
	///
	/// See `/proc/vmallocinfo`.
	#[deprecated]
	LargestContiguousChunkInVirtualAddressSpace,
	
	/// The number is derived by dividing `SizeOfDefaultHugePage` by the megabytes set aside for `hugepages` specified in `/proc/sys/vm/hugetlb_pool`.
	///
	/// It only applies for the default page size.
	///
	/// Only if the Linux Kernel has been compiled with `CONFIG_HUGETLBFS` and `CONFIG_HUGETLB_PAGE`.
	TotalNumberOfHugePages,

	/// Number of free static huge pages for default page size.
	///
	/// Only if the Linux Kernel has been compiled with `CONFIG_HUGETLBFS` and `CONFIG_HUGETLB_PAGE`.
	FreeNumberOfHugePages,

	/// Number of reserved static huge pages for default page size.
	///
	/// Only if the Linux Kernel has been compiled with `CONFIG_HUGETLBFS` and `CONFIG_HUGETLB_PAGE`.
	///
	/// Since Linux 2.6.17.
	ReservedNumberOfHugePages,

	/// Number of dynamic huge pages in use for default page size.
	///
	/// Only if the Linux Kernel has been compiled with `CONFIG_HUGETLBFS` and `CONFIG_HUGETLB_PAGE`.
	///
	/// Since Linux 2.6.24.
	SurplusNumberOfHugePages,

	/// Only if the Linux Kernel has been compiled with `CONFIG_HUGETLBFS` and `CONFIG_HUGETLB_PAGE`.
	SizeOfDefaultHugePage,

	/// Non-file backed huge pages mapped into user-space page tables
	///
	/// Only if the Linux Kernel has been compiled with `CONFIG_TRANSPARENT_HUGEPAGE`.
	///
	/// Since Linux 2.6.38.
	TransparentHugePagesMemoryUsage,

	/// Memory used by shared memory (`shmem`) and `tmpfs` allocated with huge pages.
	///
	/// Only if the Linux Kernel has been compiled with `CONFIG_TRANSPARENT_HUGEPAGE`.
	///
	/// Since Linux 4.8.
	ShmemHugePageUsage,

	/// Memory used by shared memory (`shmem`) and `tmpfs` mapped into user space with huge pages.
	///
	/// Only if the Linux Kernel has been compiled with `CONFIG_TRANSPARENT_HUGEPAGE`.
	///
	/// Since Linux 4.8.
	ShmemMemoryMappedIntoUserSpaceUsingHugePages,

	/// Since Linux 2.6.27.
	DirectMap4k,

	/// Only if the Linux Kernel has been compiled with `CONFIG_X86_64`.
	///
	/// Since Linux 2.6.27.
	DirectMap2M,

	/// Only if the Linux Kernel has been compiled with `CONFIG_X86_64` and `CONFIG_X86_DIRECT_GBPAGES` and has 1Gb pages available.
	///
	/// Since Linux 2.6.27.
	DirectMap1G,

	/// Not documented.
	///
	/// Only if the Linux Kernel has been compiled with `CONFIG_MEMORY_FAILURE`.
	///
	/// Since Linux 2.6.32.
	HardwareCorrupted,

	/// ?
	FileHugePageUsage,

	/// Only if the Linux Kernel has been compiled with `CONFIG_HIGHMEM`.
	///
	/// Obsolete.
	#[deprecated]
	TotalHighNotDirectlyMappedIntoKernelSpace,

	/// Only if the Linux Kernel has been compiled with `CONFIG_HIGHMEM`.
	///
	/// Obsolete.
	#[deprecated]
	FreeHighNotDirectlyMappedIntoKernelSpace,

	/// Only if the Linux Kernel has been compiled with `CONFIG_HIGHMEM`.
	///
	/// Obsolete.
	#[deprecated]
	TotalLowDirectlyMappedIntoKernelSpace,

	/// Only if the Linux Kernel has been compiled with `CONFIG_HIGHMEM`.
	///
	/// Obsolete.
	#[deprecated]
	FreeLowDirectlyMappedIntoKernelSpace,

	/// Only if the Linux Kernel has been compiled with `CONFIG_MMU`.
	///
	/// Since Linux 2.6.19.
	#[deprecated]
	MMapCopy,

	/// Only if the Linux Kernel has been compiled with `CONFIG_QUICKLIST`.
	///
	/// Since Linux 2.6.17.
	#[deprecated]
	Quicklists,

	/// Only if the Linux Kernel has been compiled with `CONFIG_CMA`.
	///
	/// Number of pages.
	///
	/// Since Linux 3.1.
	ContiguousMemoryAllocatorTotal,

	/// Only if the Linux Kernel has been compiled with `CONFIG_CMA`.
	///
	/// Number of pages.
	///
	/// Since Linux 3.1.
	ContiguousMemoryAllocatorFree,

	/// Unknown.
	Unknown(Box<[u8]>),
}

impl MemoryInformationName
{
	/// Parse a memory statistic name.
	///
	/// This list is NOT definitive; names come and go.
	#[inline(always)]
	#[allow(deprecated)]
	pub(crate) fn parse(value: &[u8], memory_information_name_prefix: &[u8]) -> MemoryInformationName
	{
		use self::MemoryInformationName::*;

		if !value.starts_with(memory_information_name_prefix)
		{
			return Unknown(value.to_vec().into_boxed_slice())
		}

		match &value[memory_information_name_prefix.len() .. ]
		{
			b"MemTotal" => TotalPhysicalRam,
			b"MemFree" => FreePhysicalRam,
			b"MemAvailable" => AvailablePhysicalRam,
			b"Buffers" => UsedAsFileBuffersPhysicalRam,
			b"Cached" => UsedAsCachePhysicalRam,
			b"SwapTotal" => TotalSwap,
			b"SwapFree" => FreeSwap,
			b"SwapCached" => UsedAsCacheSwap,
			b"LazyFree" => LazyFree,
			b"Active" => ActiveFileBufferAndCacheInUse,
			b"Inactive" => InactiveFileBufferAndCacheAvailable,
			b"Active(anon)" => AnonymousActive,
			b"Inactive(anon)" => AnonymousInactive,
			b"Active(file)" => FileActive,
			b"Inactive(file)" => FileInactive,
			b"Unevictable" => Unevictable,
			b"Dirty" => WaitingToBeWrittenBackToDisks,
			b"Writeback" => CurrentlyBeingWrittenBackToDisks,
			b"Bounce" => UsedForBlockDeviceBounceBuffers,
			b"NFS_Unstable" => NetworkFileSystemUnstablePagesSentToServerButNotYetCommittedToStableStorage,
			b"WritebackTmp" => MemoryUsedByFuseForTemporaryWritebackBuffers,
			b"AnonPages" => AnonymousMemoryMappedUsingMmap,
			b"Mapped" => FilesMappedUsingMmap,
			b"Shmem" => Shmem,
			b"Mlocked" => LockedByMlock,
			b"Slab" => Slab,
			b"SReclaimable" => SlabReclaimable,
			b"SUnreclaim" => SlabUnreclaimable,
			b"KernelStack" => KernelStack,
			b"PageTables" => MemoryDedicatedToLowestPageTableLevel,
			b"CommitLimit" => CommitLimit,
			b"Committed_AS" => WorstCaseScenarioMemoryRequiredToCompleteWorkloadIncludingSwapMemory,
			b"VmallocTotal" => TotalVirtualAddressSpace,
			b"VmallocUsed" => UsedVirtualAddressSpace,
			b"VmallocChunk" => LargestContiguousChunkInVirtualAddressSpace,
			b"HugePages_Total" => TotalNumberOfHugePages,
			b"HugePages_Free" => FreeNumberOfHugePages,
			b"HugePages_Rsvd" => ReservedNumberOfHugePages,
			b"HugePages_Surp" => SurplusNumberOfHugePages,
			b"Hugepagesize" => SizeOfDefaultHugePage,
			b"AnonHugePages" => TransparentHugePagesMemoryUsage,
			b"ShmemHugePages" => ShmemHugePageUsage,
			b"FileHugePages" => FileHugePageUsage,
			b"ShmemPmdMapped" => ShmemMemoryMappedIntoUserSpaceUsingHugePages,
			b"DirectMap4k" => DirectMap4k,
			b"DirectMap2M" => DirectMap2M,
			b"DirectMap1G" => DirectMap1G,
			b"HardwareCorrupted" => HardwareCorrupted,
			b"HighTotal" => TotalHighNotDirectlyMappedIntoKernelSpace,
			b"HighFree" => FreeHighNotDirectlyMappedIntoKernelSpace,
			b"LowTotal" => TotalLowDirectlyMappedIntoKernelSpace,
			b"LowFree" => FreeLowDirectlyMappedIntoKernelSpace,
			b"MmapCopy" => MMapCopy,
			b"Quicklists" => Quicklists,
			b"CmaTotal" => ContiguousMemoryAllocatorTotal,
			b"CmaFree" => ContiguousMemoryAllocatorFree,

			name @ _ => Unknown(name.to_vec().into_boxed_slice()),
		}
	}

	/// Associated memory statistic unit.
	#[inline(always)]
	pub fn unit(&self) -> MemoryInformationUnit
	{
		use self::MemoryInformationName::*;
		use self::MemoryInformationUnit::*;
		
		match *self
		{
			TotalNumberOfHugePages => Count,
			FreeNumberOfHugePages => Count,
			ReservedNumberOfHugePages => Count,
			SurplusNumberOfHugePages => Count,
			ContiguousMemoryAllocatorTotal => Count,
			ContiguousMemoryAllocatorFree => Count,

			_ => KiloByte,
		}
	}

	#[inline(always)]
	pub(crate) fn validate_unit<'a>(&self, bytes: &'a [u8], zero_based_line_number: usize) -> Result<&'a [u8], MemoryInformationParseError>
	{
		let ends_with = self.unit().ends_with();
		if likely!(bytes.ends_with(ends_with))
		{
			Ok(&bytes[0 .. bytes.len() - ends_with.len()])
		}
		else
		{
			Err(MemoryInformationParseError::InvalidUnit { zero_based_line_number })
		}
	}
}
