// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Features flags.
	///
	/// Note that `all()` is equivalent to `UFFD_API_FEATURES` (which has not been defined in this module).
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	pub struct Features: u64
	{
		/// ?Differentiate write page faults from write protect page faults?
		///
		/// Doesn't seem to be used as of Linux version 5.11-rc6.
		const ReceivePageFaultWriteProtectEvents = UFFD_FEATURE_PAGEFAULT_FLAG_WP;
		
		/// Fork.
		///
		/// Since Linux version 4.11.
		///
		/// The calling process requires the capability `CAP_SYS_PTRACE`.
		const ReceiveForkEvents = UFFD_FEATURE_EVENT_FORK;
		
		/// Remap using `mremap()`.
		///
		/// Since Linux version 4.11.
		const ReceiveRemapEvents = UFFD_FEATURE_EVENT_REMAP;
		
		/// Remove using `madvise(MADV_DONT_NEED)` or `madvise(MADV_REMOVE)`.
		///
		/// Since Linux version 4.11.
		const ReceiveMAdviseDoNotNeedOrRemoveEvents = UFFD_FEATURE_EVENT_REMOVE;
		
		/// Support events on hugetlbfs memory.
		///
		/// Since Linux version 4.11.
		///
		/// "`\[this\] means an `UFFDIO_REGISTER` with `UFFDIO_REGISTER_MODE_MISSING` mode will succeed on hugetlbfs virtual memory ranges.
		/// Adding or not adding `UFFD_FEATURE_MISSING_HUGETLBFS` to `uffdio_api.features` has no real functional effect after `UFFDIO_API` returns, but it's only useful for an initial feature set probe at `UFFDIO_API` time.
		/// There are two ways to use it:-
		/// 1. by adding `UFFD_FEATURE_MISSING_HUGETLBFS` to the `uffdio_api.features` before calling `UFFDIO_API`, an error will be returned by `UFFDIO_API` on a kernel without hugetlbfs missing support.
		/// 2. the `UFFD_FEATURE_MISSING_HUGETLBFS` can not be added in `uffdio_api.features` and instead it will be set by the kernel in the `uffdio_api.features` if the kernel supports it, so userland can later check if the feature flag is present in `uffdio_api.features` after `UFFDIO_API` succeeded.
		const SupportEventsOnHugetlbfs = UFFD_FEATURE_MISSING_HUGETLBFS;
		
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
		const SupportEventsOnSharedMemory = UFFD_FEATURE_MISSING_SHMEM;
		
		/// `munmap()`.
		const Unmap = UFFD_FEATURE_EVENT_UNMAP;
		
		/// Do not receive revents for page faults but have them signalled with signal `SIGBUS`.
		///
		/// Since Linux version 4.14.
		///
		/// "\[this\] means no page-fault (`UFFD_EVENT_PAGEFAULT`) events will be delivered, instead a `SIGBUS` signal will be sent to the faulting process
		/// Applications using this feature will not require the use of a `userfaultfd` monitor for processing memory accesses to the regions registered with `userfaultfd`".
		/// (But they will require a signal handler)!
		const DoNotReceivePageFaultEventsButRaiseSIGBUSSignalInstead = UFFD_FEATURE_SIGBUS;
		
		/// Thread identifier.
		///
		/// "\[this\] means `pid` of the page faulted `task_struct` will be returned, if feature is not requested 0 will be returned".
		///
		/// ie, the field `uffd_msd.pagefault.feat.ptid` will not be zero.
		const ThreadIdentifier = UFFD_FEATURE_THREAD_ID;
	}
}
