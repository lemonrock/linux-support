// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An enum allowing one to influence how page size is discovered.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum PageSizePreference
{
	/// The system's default page size.
	///
	/// This is:-
	///
	/// * Always 4Kb on:-
	/// 	* powerpc64.
	/// 	* risvc64.
	/// 	* x86_64.
	/// * Always 8Kb on:-
	/// 	* sparc64
	/// * Varies between 4Kb, 16Kb and 64Kb on:-
	/// 	* aarch64
	///  * Varies between 4Kb, 8Kb, 16Kb and 64Kb on:-
	/// 	* mips64
	DefaultPageSize,
	
	/// When trying to find a huge page size, the default for the system is preferred, if available; otherwise, the `DefaultPageSize` is used.
	DefaultHugePageSize,

	/// When trying to find a huge page size, this one is preferred, if available; otherwise, any smaller `HugePageSize` is chosen.
	///
	/// If huge pages are not available, then the `DefaultPageSize` is used.
	PreferredHugePageSize(HugePageSize),
}

impl Default for PageSizePreference
{
	#[inline(always)]
	fn default() -> Self
	{
		PageSizePreference::DefaultPageSize
	}
}
