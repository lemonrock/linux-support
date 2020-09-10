// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Huge page pool statistics.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HugePagePoolStatistics
{
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
	///
	/// Dynamic huge page pool only exists if the huge page is *not* a gigantic huge page.
	/// `None` for gigantic huge pages (the kernel models these with a value of zero in its files in `/sys`).
	/// On x86_64, this means this field is `Some()` for 2Mb huge pages but not 1Gb gigantic huge pages; in the former case it can be `Some(0)`.
	number_of_dynamic_pages_in_use: Option<u64>,
}

impl HugePagePoolStatistics
{
	/// Also known as 'surplus'.
	#[inline(always)]
	pub fn number_of_dynamic_pages_in_use(&self) -> u64
	{
		self.number_of_dynamic_pages_in_use.unwrap_or(0)
	}

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

		const FreeHugePagesFileName: &'static str = "free_hugepages";
		const SurpulusHugePagesFileName: &'static str = "surplus_hugepages";
		const ReservedHugePagesFileName: &'static str = "resv_hugepages";

		Some
		(
			Self
			{
				number_of_static_pages_free: read(base_path_ref, FreeHugePagesFileName),
				reserved: read(base_path_ref, ReservedHugePagesFileName),
				number_of_dynamic_pages_in_use: if huge_page_size.is_a_gigantic_huge_page()
				{
					None
				}
				else
				{
					Some(read(base_path_ref, SurpulusHugePagesFileName))
				},
			}
		)
	}
}
