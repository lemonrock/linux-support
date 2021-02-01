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
		/// Page fault for read and write.
		const PageFault = UFFD_FEATURE_PAGEFAULT_FLAG_WP;
		
		/// Fork.
		///
		/// The calling process requires the capability `CAP_SYS_PTRACE`.
		const Fork = UFFD_FEATURE_EVENT_FORK;
		
		/// Remap using `mremap()`.
		const Remap = UFFD_FEATURE_EVENT_REMAP;
		
		/// Remove using `madvise(DONT_NEED)` or `madvise(REMOVE)`.
		const MAdviseDoNotNeedOrRemove = UFFD_FEATURE_EVENT_REMOVE;
		
		/// Missing hugetlbfs.
		///
		/// "`\[this\] means an `UFFDIO_REGISTER` with `UFFDIO_REGISTER_MODE_MISSING` mode will succeed on hugetlbfs virtual memory ranges.
		/// Adding or not adding `UFFD_FEATURE_MISSING_HUGETLBFS` to `uffdio_api.features` has no real functional effect after `UFFDIO_API` returns, but it's only useful for an initial feature set probe at `UFFDIO_API` time.
		/// There are two ways to use it:-
		/// 1. by adding `UFFD_FEATURE_MISSING_HUGETLBFS` to the `uffdio_api.features` before calling `UFFDIO_API`, an error will be returned by `UFFDIO_API` on a kernel without hugetlbfs missing support.
		/// 2. the `UFFD_FEATURE_MISSING_HUGETLBFS` can not be added in `uffdio_api.features` and instead it will be set by the kernel in the `uffdio_api.features` if the kernel supports it, so userland can later check if the feature flag is present in `uffdio_api.features` after `UFFDIO_API` succeeded.
		const MissingHugetlbfs = UFFD_FEATURE_MISSING_HUGETLBFS;
		
		/// Missing shared memory (shmem).
		///
		/// As above for `MissingHugetlbfs` but applies to tmpfs and other shmem-based APIs.
		const MissingSharedMemory = UFFD_FEATURE_MISSING_SHMEM;
		
		/// `munmap()`.
		const Unmap = UFFD_FEATURE_EVENT_UNMAP;
		
		/// Signal `SIGBUS`.
		///
		/// "\[this\] means no page-fault (`UFFD_EVENT_PAGEFAULT`) event will be delivered, instead a `SIGBUS` signal will be sent to the faulting process".
		const SignalSIGBUS = UFFD_FEATURE_SIGBUS;
		
		/// Thread identifier.
		///
		/// "\[this\] means pid of the page faulted task_struct will be returned, if feature is not requested 0 will be returned".
		const ThreadIdentifier = UFFD_FEATURE_THREAD_ID;
	}
}
