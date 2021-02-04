// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const UFFD_API_FEATURES: u64 = UFFD_FEATURE_PAGEFAULT_FLAG_WP | UFFD_FEATURE_EVENT_FORK | UFFD_FEATURE_EVENT_REMAP | UFFD_FEATURE_EVENT_REMOVE | UFFD_FEATURE_EVENT_UNMAP | UFFD_FEATURE_MISSING_HUGETLBFS | UFFD_FEATURE_MISSING_SHMEM | UFFD_FEATURE_SIGBUS | UFFD_FEATURE_THREAD_ID;
