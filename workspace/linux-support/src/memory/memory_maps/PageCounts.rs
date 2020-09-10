// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mapping page counts.
///
/// Only present if the kernel has been built with `CONFIG_NUMA` and `/proc` is mounted.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PageCounts
{
	/// ?
	pub pages_are_hugetlb_pages: bool,

	/// Seems to not be for files.
	pub anonymous: NumberOfPages,

	/// Seems to not be for files.
	pub dirty: NumberOfPages,

	/// Only written by Linux if `md.pages != md.anon && md.pages != md.dirty`.
	///
	/// Seems to be for files only.
	pub mapped: NumberOfPages,

	/// Seems to be for files only.
	pub map_count_maximum: NonZeroNumberOfPages,

	/// Usually zero.
	pub swap_cache: NumberOfPages,

	/// Only written by Linux for non-huge pages if `md.active < md.pages`.
	pub active: Option<NumberOfPages>,

	/// Usually zero.
	pub write_back: NumberOfPages,

	/// Pages in use by NUMA node.
	///
	/// Only for NUMA nodes that have 'online memory'.
	pub by_numa_node: HashMap<NumaNode, NumberOfPages>,
}
