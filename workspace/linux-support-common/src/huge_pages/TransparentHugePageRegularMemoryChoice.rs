// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transparent Huge Page (THP) regular memory choice.
///
/// Used for at least:-
///
/// * Ashmem
#[derive(Deserialize, Serialize)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TransparentHugePageRegularMemoryChoice
{
	/// Never allocate.
	Never,

	/// Always use.
	Always,

	/// Only for pages so specified by the `madvise()` (or `fadvise()`) syscall with the `MADV_HUGEPAGE` flag.
	Advise,
}

impl Default for TransparentHugePageRegularMemoryChoice
{
	#[inline(always)]
	fn default() -> Self
	{
		TransparentHugePageRegularMemoryChoice::Never
	}
}

impl TransparentHugePageRegularMemoryChoice
{
	/// To value.
	#[inline(always)]
	pub fn to_value(self) -> &'static str
	{
		use self::TransparentHugePageRegularMemoryChoice::*;

		match self
		{
			Never => "never",
			Always => "always",
			Advise => "madvise",
		}
	}

	/// Changes Transparent Huge Pages (THP) settings.
	///
	/// The value of `self` can also be specified in the Linux kernel command line parameters as one of "transparent_hugepage=never", "transparent_hugepage=always" or "transparent_hugepage=madvise".
	pub fn change_transparent_huge_pages_usage(self, sys_path: &SysPath, transparent_huge_page_shared_memory_choice: TransparentHugePageSharedMemoryChoice, use_zero_page: bool) -> io::Result<()>
	{
		let use_zero_page_value = if use_zero_page
		{
			1
		}
		else
		{
			0
		};
		sys_path.global_transparent_huge_memory_file_path("use_zero_page").write_value(use_zero_page_value)?;

		transparent_huge_page_shared_memory_choice.change_transparent_huge_pages_usage(sys_path)?;

		sys_path.global_transparent_huge_memory_file_path("enabled").write_value(self.to_value())
	}
}
