// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Commonly supported huge page sizes for modern popular CPU architectures (x86, ARM, PowerPC).
///
/// See also <https://en.wikipedia.org/wiki/Page_(computer_memory)#Huge_pages>.
///
/// `repr(u64)` values are in bytes.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[derive(EnumIter)]
pub enum HugePageSize
{
	/// 64Kb.
	///
	/// Not used on x86_64.
	/// Used on powerpc64.
	/// Used on sparc64.
	///
	/// (Used on aarch64 as just a page size with the 64Kb translation granule).
	_64KB = 64 * 1_024,

	/// 512Kb.
	///
	/// Not used on x86_64.
	/// Used on sparc64.
	_512KB = 512 * 1_024,

	/// 1MB.
	///
	/// Not used on x86_64.
	_1MB = 1_024 * 1_024,
	
	/// 2MB.
	///
	/// Used on x86_64.
	/// Used on aarch64 as with the 4Kb translation granule.
	_2MB = 2_048 * 1_024,
	
	/// 4MB.
	///
	/// Not used on x86_64.
	/// Used on sparc64.
	_4MB = 4_096 * 1_024,

	/// 8MB.
	///
	/// Not used on x86_64.
	/// Used on sparc64.
	_8MB = 8_192 * 1_024,
	
	/// 16MB.
	///
	/// Not used on x86_64.
	/// Used on powerpc64.
	_16MB = 16_384 * 1_024,

	/// 32MB.
	///
	/// Not used on x86_64.
	/// Used on sparc64.
	/// Used on aarch64 as with the 16Kb translation granule.
	_32MB = 32_768 * 1_024,
	
	/// 256MB.
	///
	/// Not used on x86_64.
	/// Used on sparc64.
	_256MB = 262_144 * 1_024,
	
	/// 512MB.
	///
	/// Not used on x86_64.
	///
	/// Used on aarch64 as with the 16Kb translation granule.
	_512MB = 524_288 * 1_024,

	/// 1GB.
	///
	/// Used on x86_64.
	/// Used on aarch64 as with the 4Kb translation granule.
	_1GB = 1_048_576 * 1_024,
	
	/// 2GB.
	///
	/// Not used on x86_64.
	/// Used on sparc64.
	_2GB = 2_097_152 * 1_024,
	
	/// 16GB.
	///
	/// Impossible to specify as a huge page size to `mmap()` or `memfd_create()`.
	///
	/// Not used on x86_64.
	/// Used on powerpc64.
	/// Used on sparc64.
	_16GB = 16_777_216 * 1_024,
}

impl Into<u64> for HugePageSize
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self as u64
	}
}

impl Into<usize> for HugePageSize
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self as u64 as usize
	}
}

impl HugePageSize
{
	#[inline(always)]
	pub(crate) fn cgroup_file_name_fragment(self) -> &'static str
	{
		use self::HugePageSize::*;
		
		match self
		{
			_64KB => "64KB",
			_512KB => "512KB",
			_1MB => "1MB",
			_2MB => "2MB",
			_4MB => "4MB",
			_8MB => "8MB",
			_16MB => "16MB",
			_32MB => "32MB",
			_256MB => "256MB",
			_512MB => "512MB",
			_1GB => "1GB",
			_2GB => "2GB",
			_16GB => "16GB",
		}
	}
	
	/// Size in kilobytes.
	#[inline(always)]
	pub const fn size_in_kilobytes(self) -> NonZeroKilobyte
	{
		new_non_zero_u64((self as u64) / 1_024)
	}

	/// Size in bytes.
	#[inline(always)]
	pub const fn size_in_bytes(self) -> NonZeroU64
	{
		new_non_zero_u64(self as u64)
	}

	/// Non-zero number of pages from non-zero number of bytes, rounded up.
	#[inline(always)]
	pub fn non_zero_number_of_pages_from_non_zero_number_of_bytes_rounded_up(self, number_of_bytes: NonZeroU64) -> NonZeroNumberOfPages
	{
		new_non_zero_u64(self.number_of_pages_from_number_of_bytes_rounded_up(number_of_bytes.get()))
	}

	/// Number of pages from number of bytes, rounded up.
	#[inline(always)]
	pub fn number_of_pages_from_number_of_bytes_rounded_up(self, number_of_bytes: u64) -> NumberOfPages
	{
		let size_in_bytes = self.size_in_bytes().get();
		(number_of_bytes + size_in_bytes - 1) / size_in_bytes
	}

	/// Non-zero number of bytes rounded up to number of pages.
	#[inline(always)]
	pub fn non_zero_number_of_bytes_rounded_up_to_multiple_of_page_size(self, number_of_bytes: NonZeroU64) -> NonZeroU64
	{
		new_non_zero_u64(self.number_of_bytes_rounded_up_to_multiple_of_page_size(number_of_bytes.get()))
	}

	/// Number of bytes rounded up to number of pages.
	#[inline(always)]
	pub fn number_of_bytes_rounded_up_to_multiple_of_page_size(self, number_of_bytes: u64) -> u64
	{
		let size_in_bytes = self.size_in_bytes().get();
		((number_of_bytes + size_in_bytes - 1) / size_in_bytes) * size_in_bytes
	}

	/// Is this considered a gigantic huge page?
	#[inline(always)]
	pub fn can_have_a_dynamic_huge_page_pool(self) -> bool
	{
		!self.is_a_gigantic_huge_page()
	}

	/// Is this considered a gigantic huge page?
	#[inline(always)]
	pub fn is_a_gigantic_huge_page(self) -> bool
	{
		const Scalar: u64 = 2048;

		let minimum_gigantic_huge_page = (PageSize::default() as u64) * Scalar;
		self.size_in_bytes().get() >= minimum_gigantic_huge_page
	}

	/// Huge page pool statistics.
	///
	/// For per-NUMA Node statistics, see `NumaNode.huge_page_pool_statistics()`.
	///
	/// This will return `None` if the kernel was compiled without `CONFIG_HUGETLBFS` or `sys_path` is not mounted.
	#[inline(always)]
	pub fn global_huge_page_pool_size(self, sys_path: &SysPath) -> Option<GlobalHugePagePoolSize>
	{
		GlobalHugePagePoolSize::new(sys_path, self, SysPath::global_hugepages_folder_path)
	}

	/// Huge page pool statistics.
	///
	/// For per-NUMA Node statistics, see `NumaNode.huge_page_pool_statistics()`.
	///
	/// This will return `None` if the kernel was compiled without `CONFIG_HUGETLBFS` or `sys_path` is not mounted.
	#[inline(always)]
	pub fn global_huge_page_pool_statistics(self, sys_path: &SysPath) -> Option<HugePagePoolStatistics>
	{
		HugePagePoolStatistics::new(sys_path, self, SysPath::global_hugepages_folder_path)
	}

	/// Read number of memory policy global huge pages of `self` size.
	///
	/// This will return `None` if the kernel was compiled without `CONFIG_NUMA` or `sys_path` is not mounted.
	#[inline(always)]
	pub fn memory_policy_global_huge_pages(self, sys_path: &SysPath) -> Option<u64>
	{
		let file_path = sys_path.global_hugepages_folder_path(self).append("nr_hugepages_mempolicy");

		if file_path.exists()
		{
			Some(file_path.read_value().unwrap())
		}
		else
		{
			return None
		}
	}
	
	/// Default huge page size.
	///
	/// Usually 2Mb on x86_64 (but controlled by kernel command line options).
	///
	/// This will return `None` if `memory_information` is lacking the essential statistic used.
	#[inline(always)]
	fn default_huge_page_size(memory_information: &MemoryInformation) -> Option<Self>
	{
		if let Some(size_in_bytes) = memory_information.get_statistic(&MemoryInformationName::SizeOfDefaultHugePage)
		{
			Self::from_kilobytes(size_in_bytes)
		}
		else
		{
			None
		}
	}

	/// This will return `None` if the kernel was compiled without ?`CONFIG_TRANSPARENT_HUGEPAGE` or `sys_path` is not mounted.
	#[inline(always)]
	fn transparent_huge_page_size(sys_path: &SysPath) -> Option<Self>
	{
		let file_path = sys_path.transparent_huge_memory_file_path("hpage_pmd_size");
		if file_path.exists()
		{
			let value: NonZeroU64 = file_path.read_value().unwrap();
			Self::from_non_zero_kilobytes(value)
		}
		else
		{
			None
		}
	}

	/// Supported huge page sizes, sorted smallest to largest.
	///
	/// There may be no huge pages because:-
	///
	/// * The folder `/sys/kernel/mm/hugepages` does not exist (I have seen this on Alpine Linux 3.11 running the 'linux-lts' kernel on a Parallels Hypervisor).
	/// * The architecture only has huge pages we do not support (extremely unlikely).
	/// * We are on an ancient CPU that does not have huge pages (extremely unlikely).
	///
	/// On modern x86_64 from Sandy Bridge onwards, will contain 1Gb gigantic huge page and 2Mb huge page sizes.
	///
	/// This will return an empty set if the kernel was compiled without `CONFIG_HUGETLBFS` or `sys_path` is not mounted.
	#[inline(always)]
	fn supported_huge_page_sizes(sys_path: &SysPath) -> BTreeSet<Self>
	{
		let mut supported = BTreeSet::new();

		for huge_page_size in Self::iter()
		{
			if sys_path.global_hugepages_folder_path(huge_page_size).exists()
			{
				supported.insert(huge_page_size);
			}
		}
		
		supported
	}

	#[inline(always)]
	fn from_non_zero_kilobytes(value: NonZeroU64) -> Option<Self>
	{
		Self::from_kilobytes(value.get())
	}

	#[inline(always)]
	fn from_kilobytes(value: u64) -> Option<Self>
	{
		use self::HugePageSize::*;

		match value
		{
			64 => Some(_64KB),
			512 => Some(_512KB),
			1_024 => Some(_1MB),
			2_048 => Some(_2MB),
			4_096 => Some(_4MB),
			8_192 => Some(_8MB),
			16_384 => Some(_16MB),
			32_768 => Some(_32MB),
			262_144 => Some(_256MB),
			524_288 => Some(_512MB),
			1_048_576 => Some(_1GB),
			2_097_152 => Some(_2GB),
			16_777_216 => Some(_16GB),

			_ => None,
		}
	}

	/// Value for use with `flags` in `mmap()` (`MAP_HUGE_*`, eg `MAP_HUGE_2MB`) and `memfd_create()` (`MFD_HUGE_*` eg `MFD_HUGE_2MB`).
	///
	/// Note that the `mmap()` and `memfd_create()` calls can not accept huge page sizes over 2Gb as they use a 32-bit integer!
	#[inline(always)]
	fn mmap_and_memfd_flags_bits(self) -> i32
	{
		const MAP_HUGE_SHIFT: u64 = 26;
		let value: u64 = self.log_base_2_of_bytes() << MAP_HUGE_SHIFT;
		let value: u32 = value.try_into().expect("Gigantic huge pages more than 2Gb are not supported by mmap or memfd");
		value as i32
	}

	/// What value, N, gives `Self == 1 << N`.
	const fn log_base_2_of_bytes(self) -> u64
	{
		(self as u64).trailing_zeros() as u64
	}
}
