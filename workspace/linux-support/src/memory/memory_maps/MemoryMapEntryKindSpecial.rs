// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// What kind of special entry is this?
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum MemoryMapEntryKindSpecial
{
	/// A heap mapping.
	///
	/// A process has a maximum of one of these (it may be absent).
	Heap
	{
		/// Only `Some` if the kernel has been built with `CONFIG_NUMA`.
		page_counts: Option<PageCounts>,
	},

	/// A stack mapping.
	///
	/// A process has exactly one of these.
	Stack
	{
		/// Only `Some` if the kernel has been built with `CONFIG_NUMA`.
		page_counts: Option<PageCounts>,
	},

	/// A `vdso` mapping.
	///
	/// A process has exactly one of these.
	///
	/// Does not have `page_counts`.
	vDSO,

	/// A `vvar` mapping.
	///
	/// A process has exactly one of these.
	///
	/// Does not have `page_counts`.
	VVAR,
}
