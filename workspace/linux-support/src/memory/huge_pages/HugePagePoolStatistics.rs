// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Huge page pool statistics.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HugePagePoolStatistics
{
	/// Maximum number of `persistent` (ie static) huge pages, ie these are the number of pages in the Kernel's huge page size pool for the associated Huge Page Size.
	///
	/// Reducing this value converts the difference from `static` huge pages to `dynamic` huge pages.
	///
	/// Sometimes called `preallocated` huge pages.
	///
	/// Can be thought of as 'capacity'.
	///
	/// Found in `hugepages/hugepages-<huge_page_size>kB/nr_hugepages` for absolute paths:-
	///
	/// * `/sys/kernel/mm`;
	/// * `/sys/devices/system/node/node<numa_node>`.
	///
	/// This the column `Minimum` from `hugeadm --pool-list`.
	///
	/// Readable and writable.
	pub static_pool_maximum_size: u64,

	/// Number of static pages free.
	///
	/// Can be thought of as 'available'.
	///
	/// Found in `hugepages/hugepages-<huge_page_size>kB/free_hugepages` for absolute paths:-
	///
	/// * `/sys/kernel/mm`;
	/// * `/sys/devices/system/node/node<numa_node>`.
	///
	/// Readable only.
	pub number_of_static_pages_free: u64,

	///
	/// Found in `hugepages/hugepages-<huge_page_size>kB/resv_hugepages` for absolute paths:-
	///
	/// * `/sys/kernel/mm`;
	/// * `/sys/devices/system/node/node<numa_node>`.
	///
	/// Readable only.
	pub reserved: u64,

	/// Dynamic huge page pool only exists if the huge page is *not* a gigantic huge page.
	pub dynamic: DynamicHugePagePoolStatistics,

	base_path: PathBuf,
}

impl HugePagePoolStatistics
{
	const NumberOfHugePagesFileName: &'static str = "nr_hugepages";

	const NumberOfOvercommitHugePagesFileName: &'static str = "nr_overcommit_hugepages";

	/// The is the column `Maximum` from `hugeadm --pool-list`.
	#[inline(always)]
	pub fn maximum_number_of_huge_pages_possible(&self) -> u64
	{
		self.static_pool_maximum_size + self.dynamic.dynamic_pool_maximum_size_and_number_of_dynamic_pages_in_use().0
	}

	/// Also known as 'surplus'.
	#[inline(always)]
	pub fn number_of_dynamic_pages_in_use(&self) -> u64
	{
		self.dynamic.dynamic_pool_maximum_size_and_number_of_dynamic_pages_in_use().1
	}

	#[inline(always)]
	pub(crate) fn new(sys_path: &SysPath, huge_page_size: HugePageSize, constructor: impl FnOnce(&SysPath, HugePageSize) -> PathBuf) -> HugePagePoolStatistics
	{
		let base_path = constructor(sys_path, huge_page_size);
		let base_path_ref = base_path.as_path();

		#[inline(always)]
		fn read(base_path: &Path, file_name: &str) -> u64
		{
			base_path.to_path_buf().append(file_name).read_value().unwrap()
		}

		const FreeHugePagesFileName: &'static str = "free_hugepages";
		const SurpulusHugePagesFileName: &'static str = "surplus_hugepages";
		const ReservedHugePagesFileName: &'static str = "resv_hugepages";

		use self::DynamicHugePagePoolStatistics::*;

		Self
		{
			static_pool_maximum_size: read(base_path_ref, Self::NumberOfHugePagesFileName),
			number_of_static_pages_free: read(base_path_ref, FreeHugePagesFileName),
			reserved: read(base_path_ref, ReservedHugePagesFileName),
			dynamic: if huge_page_size.is_a_gigantic_huge_page()
			{
				GiganticPageDoesNotHaveADynamicHugePagePagePool
			}
			else
			{
				DynamicHugePagePagePool
				{
					dynamic_pool_maximum_size: read(base_path_ref, Self::NumberOfOvercommitHugePagesFileName),
					number_of_dynamic_pages_in_use: read(base_path_ref, SurpulusHugePagesFileName),
				}
			},
			base_path,
		}
	}

	/// Will only work as root.
	#[inline(always)]
	pub fn change_static_pool_maximum_size(self,  maximum_size: u64)
	{
		assert_effective_user_id_is_root(&format!("change_static_pool_maximum_size number_of_huge_pages '{:?}'", maximum_size));

		self.write(Self::NumberOfHugePagesFileName, maximum_size);
	}

	/// Will only work as root.
	///
	/// Ignored for gigantic pages.
	#[inline(always)]
	pub fn change_dynamic_pool_maximum_size(self, maximum_size: u64)
	{
		assert_effective_user_id_is_root(&format!("change_dynamic_pool_maximum_size number_of_huge_pages '{:?}'", maximum_size));

		use self::DynamicHugePagePoolStatistics::*;

		match self.dynamic
		{
			DynamicHugePagePagePool { .. } => self.write(Self::NumberOfOvercommitHugePagesFileName, maximum_size),
			_ => (),
		}

	}

	#[inline(always)]
	fn write(self, file_name: &str, number: u64)
	{
		self.base_path.append(file_name).write_value(number).unwrap()
	}
}
