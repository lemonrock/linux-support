// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global huge page pool size.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalHugePagePoolSize
{
	/// Maximum number of `persistent` (ie static) huge pages, ie these are the number of pages in the Kernel's huge page size pool for the associated Huge Page Size.
	///
	/// Reducing this value converts the difference from `static` huge pages to `dynamic` huge pages.
	///
	/// Sometimes called `preallocated` huge pages.
	///
	/// Can be thought of as 'capacity'.
	///
	/// Found in `/sys/kernel/mm/hugepages/hugepages-<huge_page_size>kB/nr_hugepages`.
	///
	/// This the column `Minimum` from `hugeadm --pool-list`.
	///
	/// Readable and writable.
	pub static_pool_maximum_size: u64,

	/// Readable and writable.
	///
	/// `None` for gigantic huge pages (the kernel models these with a value of zero in its files in `/sys`).
	/// On x86_64, this means this field is `Some()` for 2Mb huge pages but not 1Gb gigantic huge pages; in the former case it can be `Some(0)`.
	dynamic_pool_maximum_size: Option<u64>,

	base_path: PathBuf,
}

impl GlobalHugePagePoolSize
{
	const NumberOfHugePagesFileName: &'static str = "nr_hugepages";

	const NumberOfOvercommitHugePagesFileName: &'static str = "nr_overcommit_hugepages";

	#[inline(always)]
	pub(crate) fn new(sys_path: &SysPath, huge_page_size: HugePageSize, base_path: impl FnOnce(&SysPath, HugePageSize) -> PathBuf) -> Option<Self>
	{
		let base_path = base_path(sys_path, huge_page_size);
		if !base_path.exists()
		{
			return None
		}

		let base_path_ref = base_path.as_path();

		#[inline(always)]
		fn read(base_path: &Path, file_name: &str) -> u64
		{
			base_path.to_path_buf().append(file_name).read_value().unwrap()
		}

		Some
		(
			Self
			{
				static_pool_maximum_size: read(base_path_ref, Self::NumberOfHugePagesFileName),

				dynamic_pool_maximum_size: if huge_page_size.is_a_gigantic_huge_page()
				{
					None
				}
				else
				{
					Some(read(base_path_ref, Self::NumberOfOvercommitHugePagesFileName))
				},

				base_path,
			}
		)
	}

	/// Maximum number of `surplus` (ie dynamic) huge pages, ie these are the number of pages that *could be* in the Kernel's huge page size pool for the associated Huge Page Size.
	///
	/// `surplus` pages are created by coalescing (defragmenting) memory in the Kernel's ordinary (eg 4Kb on x86_64) page pool.
	///
	/// Can be thought of as 'capacity'.
	///
	/// Found in `/sys/kernel/mm/hugepages/hugepages-<huge_page_size>kB/nr_overcommit_hugepages`.
	#[inline(always)]
	pub fn dynamic_pool_maximum_size(&self) -> u64
	{
		self.dynamic_pool_maximum_size.unwrap_or(0)
	}

	/// The is the column `Maximum` from `hugeadm --pool-list`.
	#[inline(always)]
	pub fn maximum_number_of_huge_pages_possible(&self) -> u64
	{
		self.static_pool_maximum_size + self.dynamic_pool_maximum_size()
	}

	/// Will panic if the current user is not root.
	#[inline(always)]
	pub fn change_static_pool_maximum_size(self,  maximum_size: u64)
	{
		assert_effective_user_id_is_root(&format!("change_static_pool_maximum_size '{:?}'", maximum_size));

		self.write(Self::NumberOfHugePagesFileName, maximum_size);
	}

	/// Will panic if the current user is not root.
	///
	/// Ignored for gigantic pages.
	#[inline(always)]
	pub fn change_dynamic_pool_maximum_size(self, maximum_size: u64)
	{
		assert_effective_user_id_is_root(&format!("change_dynamic_pool_maximum_size '{:?}'", maximum_size));

		if self.dynamic_pool_maximum_size.is_some()
		{
			self.write(Self::NumberOfOvercommitHugePagesFileName, maximum_size)
		}
	}

	#[inline(always)]
	fn write(self, file_name: &str, number: u64)
	{
		self.base_path.append(file_name).write_value(number).unwrap()
	}
}
