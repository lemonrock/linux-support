// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory advice.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum MemoryAdvice
{
	/// No special treatment.
	///
	/// This is the default.
	Normal = MADV_NORMAL,

	/// Expect page references in random order.
	///
	/// (Hence, read ahead may be less useful than normally).
	Random = MADV_RANDOM,

	/// Expect page references in sequential order.
	///
	/// (Hence, pages in the given range can be aggressively read ahead, and may be freed soon after they are accessed).
	Sequential = MADV_SEQUENTIAL,

	/// Expect access in the near future.
	///
	/// (Hence, it might be a good idea to read some pages ahead).
	WillNeed = MADV_WILLNEED,

	/// Do not expect access in the near future.
	///
	/// (For the time being, the application is finished with the given range, so the kernel can free resources associated with it).
	///
	/// After a successful operation, the semantics of memory access in the specified region are changed: subsequent accesses of pages in the range will succeed, but will result in either repopulating the memory contents from the up-to-date contents of the underlying mapped file (for shared file mappings, shared anonymous mappings, and shmem-based techniques such as System V shared memory segments) or zero-fill-on-demand pages for anonymous private mappings.
	///
	/// Note that, when applied to shared mappings, this flag might not lead to immediate freeing of the pages in the range.
	/// The kernel is free to delay freeing the pages until an appropriate moment.
	/// The resident set size (RSS) of the calling process will be immediately reduced however.
	///
	/// This cannot be applied to locked pages, Huge TLB pages, or `VM_PFNMAP` pages.
	///
	/// (Pages marked with the kernel-internal `VM_PFNMAP` flag are special memory areas that are not managed by the virtual memory subsystem.
	/// Such pages are typically created by device drivers that map the pages into user space).
	DontNeed = MADV_DONTNEED,

	/// The application no longer requires the pages in the range specified by `addr` and `len`.
	///
	/// The kernel can thus free these pages, but the freeing could be delayed until memory pressure occurs.
	/// For each of the pages that has been marked to be freed but has not yet been freed, the free operation will be canceled if the caller writes into the page.
	/// After a successful `free` operation, any stale data (ie, dirty, unwritten pages) will be lost when the kernel frees the pages.
	/// However, subsequent writes to pages in the range will succeed and then kernel cannot free those dirtied pages, so that the caller can always see just written data.
	/// If there is no subsequent write, the kernel can free the pages at any time.
	///
	/// Once pages in the range have been freed, the caller will see zero-fill-on-demand pages upon subsequent page references.
	///
	/// This operation can be applied only to private anonymous pages (see `mmap(2)`).
	///
	/// In Linux before version 4.12, when freeing pages on a swapless system, the pages in the given range are freed instantly, regardless of memory pressure.
	///
	/// Since Linux 4.5.
	/// Buggy before Linux 4.12.
	Free = MADV_FREE,

	/// Free up a given range of pages and its associated backing store.
	///
	/// This is equivalent to punching a hole in the corresponding byte range of the backing store (see `fallocate(2)`).
	///
	/// Subsequent accesses in the specified address  range will see bytes containing zero.
	///
	/// The specified address range must be mapped shared and  writable.
	/// This flag cannot be applied to locked pages, Huge TLB pages, or `VM_PFNMAP` pages.
	///
	/// (Pages marked with the kernel-internal `VM_PFNMAP` flag are special memory areas that are not managed by the virtual memory subsystem.
	/// Such pages are typically created by device drivers that map the pages into user space).
	///
	/// In the initial implementation, only `tmpfs(5`) was supported; but since Linux 3.5, any filesystem which supports the `fallocate(2)` `FALLOC_FL_PUNCH_HOLE` mode also supports this flag.
	/// Hugetlbfs fails with the error `EINVAL` and other filesystems fail with the error `EOPNOTSUPP`.
	///
	/// Since Linux 2.6.16.
	/// Enhanced in Linux 3.5.
	Remove = MADV_REMOVE,

	/// Do not make the pages in this range available to the child after a `fork(2)`.
	///
	/// This is useful to prevent copy-on-write semantics from changing the physical location of a page if the parent writes to it after a `fork(2`).
	///
	/// (Such page relocations cause problems for hardware that DMAs into the page).
	///
	/// Since Linux 2.6.16.
	DontFork = MADV_DONTFORK,

	/// Undo the effect of an earlier `DontFork`.
	///
	/// Since Linux 2.6.16.
	DoFork = MADV_DOFORK,

	/// Enable Kernel Samepage Merging (KSM) for the pages in the range specified by `addr` and `length`.
	///
	/// Only available if the kernel has been built with `CONFIG_KSM`.
	///
	/// The kernel regularly scans those areas of user memory that have been marked as  mergeable, looking for pages with identical content.
	/// These are replaced by a single write-protected page (which is automatically copied if a process later wants to update the content of the page).
	///
	/// KSM merges only private anonymous pages (see `mmap(2)`).
	///
	/// The KSM feature is intended for applications that generate many instances of the same data (eg, virtualization systems such as KVM).
	/// It can consume a lot of processing power; use with care.
	///
	/// See the Linux kernel source file `Documentation/admin-guide/mm/ksm.rst` for more details.
	///
	/// Since Linux 2.6.32.
	Mergeable = MADV_MERGEABLE,

	/// Undo the effect of an earlier `Mergeable`.
	///
	/// Only available if the kernel has been built with `CONFIG_KSM`.
	///
	/// Since Linux 2.6.32.
	Unmergeable = MADV_UNMERGEABLE,

	/// Enable Transparent Huge Pages (THP) for pages in the range specified by `addr` and `length`.
	///
	/// Only available if the kernel has been built with `CONFIG_TRANSPARENT_HUGEPAGE`.
	///
	/// Currently, Transparent Huge Pages work only with private anonymous pages (see `mmap(2)`).
	///
	/// The kernel will regularly scan the areas marked as huge page candidates to replace them with huge pages.
	/// The kernel will also allocate huge pages directly when the region is naturally aligned to the huge page size (see `posix_memalign(2)`).
	///
	/// This feature is primarily aimed at applications that use large mappings of data and access large regions of that memory at a time (eg virtualization systems such as QEMU).
	/// It can very easily waste memory (eg, a 2 MB mapping that only ever accesses 1 byte will result in 2 MB of wired memory instead of one 4 KB page).
	///
	/// See the Linux kernel source file `Documentation/admin-guide/mm/transhuge.rst` for more details.
	///
	/// Most common kernels configurations provide this behavior by default, and thus this setting is normally not necessary.
	///
	/// It is mostly intended for embedded systems, where this behavior may not be enabled by default in the kernel.
	/// On such systems, this flag can be used in order to selectively enable THP.
	/// Whenever this flag is used, it should always be in regions of memory with an access pattern that the developer knows in advance won't risk to increase the memory footprint of the application when transparent hugepages are enabled.
	///
	/// Since Linux 2.6.38.
	EnableTransparentHugePages = MADV_HUGEPAGE,

	/// Ensures that memory in the address range specified by `addr` and `length` will not be backed by transparent hugepages.
	///
	/// Only available if the kernel has been built with `CONFIG_TRANSPARENT_HUGEPAGE`.
	///
	/// Since Linux 2.6.38.
	DisableTransparentHugePages = MADV_NOHUGEPAGE,

	/// Exclude from a core dump those pages in the range specified by `addr` and `length`.
	///
	/// This is useful in applications that have large areas of memory that are known not to be useful in a core dump.
	/// The effect of `DontDump` takes precedence over the bit mask that is set via the `/proc/[pid]/coredump_filter` file (see `core(5)`).
	DontDump = MADV_DONTDUMP,

	/// Undo the effect of an earlier `DontDump`.
	///
	/// Since Linux 3.4.
	DoDump = MADV_DODUMP,

	/// Present the child process with zero-filled memory in this range after a `fork(2)`.
	///
	/// This is useful in forking servers in order to ensure that sensitive per-process data (for example, PRNG seeds, cryptographic secrets, and so on) is not handed to child processes.
	///
	/// The `WipeOnFork` operation can be applied only to private anonymous pages (see `mmap(2)`).
	///
	/// Within the child created by `fork(2)`, the `WipeOnFork` setting remains in place on the specified address range.
	/// This setting is cleared during `execve(2)`.
	WipeOnFork = MADV_WIPEONFORK,

	/// Undo the effect of an earlier `WipeOnFork`.
	///
	/// Since Linux 4.14.
	KeepOnFork = MADV_KEEPONFORK,

	/// .
	Cold = MADV_COLD,

	/// .
	PageOut = MADV_PAGEOUT,

	/// Poison the pages in the range specified by `addr` and `lengt`h and handle subsequent references to those pages like a hardware memory corruption.
	///
	/// Only available if the kernel has been built with `CONFIG_MEMORY_FAILURE`.
	///
	/// This operation is available only for processes with the `CAP_SYS_ADMIN` capability.
	/// This operation may result in the calling process receiving a `SIGBUS` and the page being unmapped.
	///
	/// This feature is intended for testing of memory error-handling code.
	///
	/// Since Linux 2.6.32.
	HardwarePoision = MADV_HWPOISON,

	/// Soft offline the pages in the range specified by `addr` and `length`.
	///
	/// Only available if the kernel has been built with `CONFIG_MEMORY_FAILURE`.
	///
	/// The memory of each page in the specified range is  preserved (ie, when next accessed, the same content will be  visible, but in a new physical page frame), and the original page is offlined (ie, no longer used, and taken out of  normal memory management).
	/// The effect of this operation is invisible to (ie, does not change the semantics of) the calling process.
	///
	/// This feature is intended for testing of memory error-handling code.
	///
	/// Since Linux 2.6.33.
	SoftOffline = MADV_SOFT_OFFLINE,
}
