// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const UFFD_FEATURE_PAGEFAULT_FLAG_WP: u64 = 1 << 0;

pub(super) const UFFD_FEATURE_EVENT_FORK: u64 = 1 << 1;

pub(super) const UFFD_FEATURE_EVENT_REMAP: u64 = 1 << 2;

pub(super) const UFFD_FEATURE_EVENT_REMOVE: u64 = 1 << 3;

pub(super) const UFFD_FEATURE_MISSING_HUGETLBFS: u64 = 1 << 4;

pub(super) const UFFD_FEATURE_MISSING_SHMEM: u64 = 1 << 5;

pub(super) const UFFD_FEATURE_EVENT_UNMAP: u64 = 1 << 6;

pub(super) const UFFD_FEATURE_SIGBUS: u64 = 1 << 7;

pub(super) const UFFD_FEATURE_THREAD_ID: u64 = 1 << 8;
