// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Dynamic huge page pool statistics.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DynamicHugePagePoolStatistics
{
	/// Gigantic.
	GiganticPageDoesNotHaveADynamicHugePagePagePool,

	/// Not Gigantic.
	DynamicHugePagePagePool
	{
		/// Maximum number of `surplus` (ie dynamic) huge pages, ie these are the number of pages that *could be* in the Kernel's huge page size pool for the associated Huge Page Size.
		///
		/// `surplus` pages are created by coalescing (defragmenting) memory in the Kernel's ordinary (eg 4Kb on x86-64) page pool.
		///
		/// Can be thought of as 'capacity'.
		///
		/// Found in `hugepages/hugepages-<huge_page_size>kB/nr_overcommit_hugepages` for absolute paths:-
		///
		/// * `/sys/kernel/mm`;
		/// * `/sys/devices/system/node/node<numa_node>`.
		///
		/// Readable and writable.
		dynamic_pool_maximum_size: u64,

		/// Number of `surplus` (ie dynamic) huge pages in use.
		///
		/// This value *can* exceed `self.maximum_surplus` *temporarily* if the value `self.maximum_static` has recently been reduced.
		///
		/// Found in `hugepages/hugepages-<huge_page_size>kB/surplus_hugepages` for absolute paths:-
		///
		/// * `/sys/kernel/mm`;
		/// * `/sys/devices/system/node/node<numa_node>`.
		///
		/// Readable only.
		number_of_dynamic_pages_in_use: u64,
	}
}

impl DynamicHugePagePoolStatistics
{
	/// Deconstruct.
	#[inline(always)]
	pub fn dynamic_pool_maximum_size_and_number_of_dynamic_pages_in_use(&self) -> (u64, u64)
	{
		use self::DynamicHugePagePoolStatistics::*;

		match self
		{
			&GiganticPageDoesNotHaveADynamicHugePagePagePool => (0, 0),
			&DynamicHugePagePagePool { dynamic_pool_maximum_size, number_of_dynamic_pages_in_use } => (dynamic_pool_maximum_size, number_of_dynamic_pages_in_use),
		}
	}
}
