// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A feature.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u64)]
pub enum Feature
{
	/// ?Differentiate write page faults from write protect page faults?
	///
	/// Doesn't seem to be used as of Linux version 5.11-rc6.
	RaisePageFaultWriteProtectEvents = UFFD_FEATURE_PAGEFAULT_FLAG_WP,
	
	/// Fork.
	///
	/// Since Linux version 4.11.
	///
	/// The calling process requires the capability `CAP_SYS_PTRACE`.
	RaiseForkEvents = UFFD_FEATURE_EVENT_FORK,
	
	/// Remap using `mremap()`.
	///
	/// Since Linux version 4.11.
	RaiseRemapEvents = UFFD_FEATURE_EVENT_REMAP,
	
	/// Remove using `madvise(MADV_DONT_NEED)` or `madvise(MADV_REMOVE)`.
	///
	/// Since Linux version 4.11.
	RaiseMAdviseDoNotNeedOrRemoveEvents = UFFD_FEATURE_EVENT_REMOVE,
	
	/// `munmap()`.
	RaiseUnmapEvents = UFFD_FEATURE_EVENT_UNMAP,
	
	/// Support events on hugetlbfs memory.
	///
	/// Since Linux version 4.11.
	///
	/// "\[this\] means an `UFFDIO_REGISTER` with `UFFDIO_REGISTER_MODE_MISSING` mode will succeed on hugetlbfs virtual memory ranges.
	/// Adding or not adding `UFFD_FEATURE_MISSING_HUGETLBFS` to `uffdio_api.features` has no real functional effect after `UFFDIO_API` returns, but it's only useful for an initial feature set probe at `UFFDIO_API` time.
	///
	/// There are two ways to use this feature:-
	///
	/// 1. by adding `SupportEventsOnHugetlbfs` to the `uffdio_api.features` before calling `UFFDIO_API`, an error will be returned by `UFFDIO_API` on a kernel without hugetlbfs support.
	/// 2. the `SupportEventsOnHugetlbfs` can not be added in `uffdio_api.features` and instead it will be set by the kernel in the `uffdio_api.features` if the kernel supports it, so userland can later check if the feature flag is present in `uffdio_api.features` after `UFFDIO_API` succeeded".
	SupportEventsOnHugetlbfs = UFFD_FEATURE_MISSING_HUGETLBFS,
	
	/// Support events on shared memory (shmem).
	///
	/// Since Linux version 4.11.
	///
	/// As above for `Hugetlbfs` but applies shmem-based APIs, including:-
	///
	/// * System V shared memory.
	/// * `tmpfs`.
	/// * `memfd` (memory file descriptors).
	/// * `mmap()` with `MAP_SHARED`.
	/// * Mapping of `/dev/zero`.
	/// * `IPCSHM`.
	///
	/// There are two ways to use this feature:-
	///
	/// 1. by adding `SupportEventsOnHugetlbfs` to the `uffdio_api.features` before calling `UFFDIO_API`, an error will be returned by `UFFDIO_API` on a kernel without hugetlbfs support.
	/// 2. the `SupportEventsOnHugetlbfs` can not be added in `uffdio_api.features` and instead it will be set by the kernel in the `uffdio_api.features` if the kernel supports it, so userland can later check if the feature flag is present in `uffdio_api.features` after `UFFDIO_API` succeeded".
	SupportEventsOnSharedMemory = UFFD_FEATURE_MISSING_SHMEM,
	
	/// Do not receive events for page faults but have them signalled with signal `SIGBUS`.
	///
	/// Since Linux version 4.14.
	///
	/// "\[this\] means no page-fault (`UFFD_EVENT_PAGEFAULT`) events will be delivered, instead a `SIGBUS` signal will be sent to the faulting process
	/// Applications using this feature will not require the use of a `userfaultfd` monitor for processing memory accesses to the regions registered with `userfaultfd`".
	/// (But they will require a signal handler)!
	DoNotRaisePageFaultEventsButRaiseSIGBUSSignalInstead = UFFD_FEATURE_SIGBUS,
	
	/// Thread identifier.
	///
	/// "\[this\] means `pid` of the page faulted `task_struct` will be returned, if feature is not requested `0` will be returned".
	///
	/// ie, the field `uffd_msd.pagefault.feat.ptid` will not be `None`.
	///
	/// There is a slight cost on each page fault raised for using this feature.
	ThreadIdentifier = UFFD_FEATURE_THREAD_ID,
}
