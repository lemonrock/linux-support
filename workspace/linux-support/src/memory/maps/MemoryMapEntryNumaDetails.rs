// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// NUMA details for a memory map entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryMapEntryNumaDetails
{
	/// NUMA memory policy.
	pub memory_policy: (),

	/// Number of anonymous pages.
	pub number_of_anonymous_pages: NumberOfPages,

	/// Number of dirty pages.
	pub number_of_dirty_pages: NumberOfPages,

	/// Number of pages by NUMA node.
	pub number_of_pages_by_numa_node: HashMap<NumaNode, NumberOfPages>,

	/// Kernel page size.
	pub kernel_page_size: PageSize,
}
