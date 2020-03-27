// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A NUMA node.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NumaNode(u8);

impl From<u8> for NumaNode
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		Self(value)
	}
}

impl TryFrom<u16> for NumaNode
{
	type Error = TryFromIntError;

	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		Ok(Self(u8::try_from(value)?))
	}
}

impl Into<u8> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Into<u16> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0 as u16
	}
}

impl Into<u32> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl Into<u64> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Into<usize> for NumaNode
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for NumaNode
{
	/// Converts data to a byte string terminated with a new line (`\n`).
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		self.0.into_line_feed_terminated_byte_string()
	}
}

impl NumaNode
{
	/// True if the Linux kernel was configured with `CONFIG_NUMA`
	#[inline(always)]
	pub fn is_a_numa_machine(sys_path: &SysPath) -> bool
	{
		sys_path.numa_nodes_folder_path().exists()
	}

	/// Reads the hyper thread and NUMA node of the currently executing CPU from the `IA32_TSC_AUX` model state register, which Linux populates.
	///
	/// Currently uses the `RDTSCP` instruction, but, once Ice Lake is widely available, could be changed to use the `RDPID` instruction.
	#[inline(always)]
	pub fn current_numa_node_and_hyper_thread() -> (NumaNode, HyperThread)
	{
		// The value of the timestamp register is stored into the `RDX` and `RAX` registers.
		// The value of the hyper thread and NUMA node is stored into the `RCX` register.
		// The top 32-bits of `RDX`, `RAX` and `RCX` are cleared (zero).
		#[inline(always)]
		unsafe fn rdtscp() -> u64
		{
			let _rax: u64;
			let _rdx: u64;
			let rcx: u64;

			asm!
			(
				"rdtscp"
				:
					"={rax}"(_rax), "={rdx}"(_rdx), "={rcx}"(rcx)
				:
				:
				:
					"volatile"
			);

			rcx
		}
		let rcx = unsafe { rdtscp() };

		let numa_node = (rcx & 0x00000000_0FFFF000) >> 12;
		debug_assert!(numa_node <= u8::MAX as u64);

		let hyper_thread = rcx & 0x00000000_00000FFF;
		debug_assert!(hyper_thread <= u16::MAX as u64);

		(Self(numa_node as u8), HyperThread::from(hyper_thread as u16))
	}

	/// Is this a NUMA node (assuming the Linux kernel wasn't configured with `CONFIG_NUMA`)?
	///
	/// Note that this might be a fake NUMA node, ie one lacking any hyper threads.
	#[inline(always)]
	pub fn is_a_numa_node(self, sys_path: &SysPath) -> bool
	{
		sys_path.numa_node_folder_path(self).exists()
	}

	/// Returns the intersection of `have_at_least_one_cpu()`, `possible()`, `online()` and `have_normal_memory()` before then checking `is_a_numa_node()`.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	pub fn valid(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		let have_cpu = Self::have_at_least_one_cpu(sys_path)?;
		let possible = Self::possible(sys_path).unwrap();
		let online = Self::online(sys_path).unwrap();
		let have_normal_memory = Self::have_normal_memory(sys_path).unwrap();

		Some
		(
			have_cpu.into_iter()
			.filter(|numa_node| possible.contains(&numa_node))
			.filter(|numa_node| online.contains(&numa_node))
			.filter(|numa_node| have_normal_memory.contains(&numa_node))
			.filter(|numa_node| numa_node.is_a_numa_node(sys_path))
			.collect()
		)
	}

	/// NUMA nodes that have a CPU.
	///
	/// NUMA nodes without a CPU are effectively fake NUMA nodes.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	fn have_at_least_one_cpu(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		Self::parse_list_mask(sys_path, "has_cpu")
	}

	/// NUMA nodes that could possibly be online at some point.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	///
	/// Not reliable, as includes NUMA nodes that can never be brought online; simply reports the number that could be used by the Linux kernel upto the `CONFIG_` number of NUMA nodes.
	#[inline(always)]
	pub fn possible(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		Self::parse_list_mask(sys_path, "possible")
	}

	/// NUMA nodes that are online at some point.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	pub fn online(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		Self::parse_list_mask(sys_path, "online")
	}

	/// NUMA nodes that have normal memory (as opposed to what was high memory, `has_high_memory`, which only exists if Linux is compiled with `CONFIG_HIGHMEM` which is an ancient setting).
	///
	/// I suspect this value is effectively useless.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	pub fn have_normal_memory(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		Self::parse_list_mask(sys_path, "has_normal_memory")
	}

	/// NUMA nodes that have hot-pluggable memory.
	///
	/// Intended to support hot-plugging of memory.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	pub fn have_movable_memory(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		Self::parse_list_mask(sys_path, "has_memory")
	}

	/// NUMA nodes that have high memory.
	///
	/// Obsolete.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	#[inline(always)]
	pub fn have_high_memory(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		Self::parse_list_mask(sys_path, "has_high_memory")
	}

	/// Huge page pool statistics.
	///
	/// This will panic if this the Linux kernel wasn't configured with `CONFIG_NUMA` or the NUMA node is not present.
	///
	/// This will also panic if the kernel was compiled without `CONFIG_HUGETLBFS` and the `hugepages` folder is missing under the `node<N>` folder.
	#[inline(always)]
	pub fn huge_page_pool_statistics(self, sys_path: &SysPath, huge_page_size: HugePageSize) -> HugePagePoolStatistics
	{
		HugePagePoolStatistics::new(sys_path, huge_page_size, |sys_path, huge_page_size| sys_path.numa_node_hugepages_folder_path(huge_page_size, self))
	}

	/// Tries to compact pages for this NUMA node.
	///
	/// This will panic if this the Linux kernel wasn't configured with `CONFIG_NUMA` or the NUMA node is not present.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn compact_pages(self, sys_path: &SysPath)
	{
		assert_effective_user_id_is_root(&format!("Compact pages in NUMA node '{}'", self.0));

		sys_path.numa_node_file_path(self.into(), "compact").write_value(1).unwrap();
	}

	/// ?Distance between NUMA nodes?
	///
	/// This will panic if this the Linux kernel wasn't configured with `CONFIG_NUMA` or the NUMA node is not present.
	///
	/// Minimum value seems to be 10.
	#[inline(always)]
	pub fn distance(self, sys_path: &SysPath) -> u8
	{
		sys_path.numa_node_file_path(self.into(), "distance").read_value().unwrap()
	}

	/// This is a subset of `self.zoned_virtual_memory_statistics()`.
	///
	/// This will panic if this the Linux kernel wasn't configured with `CONFIG_NUMA` or the NUMA node is not present.
	///
	/// Interpret this by multiplying counts by page size.
	#[deprecated]
	#[inline(always)]
	pub fn numa_memory_statistics(self, sys_path: &SysPath) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>
	{
		let file_path = sys_path.numa_node_file_path(self.into(), "numastat");
		VirtualMemoryStatisticName::parse_virtual_memory_statistics_file(&file_path)
	}

	/// Memory statistics.
	///
	/// This will panic if this the Linux kernel wasn't configured with `CONFIG_NUMA` or the NUMA node is not present.
	///
	/// Interpret this by multiplying counts by page size.
	#[inline(always)]
	pub fn zoned_virtual_memory_statistics(self, sys_path: &SysPath) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>
	{
		let file_path = sys_path.numa_node_file_path(self.into(), "vmstat");
		VirtualMemoryStatisticName::parse_virtual_memory_statistics_file(&file_path)
	}

	/// Memory information.
	///
	/// This will panic if this the Linux kernel wasn't configured with `CONFIG_NUMA` or the NUMA node is not present.
	#[inline(always)]
	pub fn memory_information(self, sys_path: &SysPath, memory_information_name_prefix: &[u8]) -> Result<MemoryInformation, MemoryInformationParseError>
	{
		let file_path = sys_path.numa_node_file_path(self.into(), "meminfo");
		MemoryInformation::parse_memory_information_file(&file_path, memory_information_name_prefix)
	}

	#[inline(always)]
	fn parse_list_mask(sys_path: &SysPath, file_name: &str) -> Option<BTreeSet<Self>>
	{
		let file_path = sys_path.numa_nodes_path(file_name);
		if file_path.exists()
		{
			Some(file_path.read_linux_core_or_numa_list(Self::try_from).unwrap())
		}
		else
		{
			None
		}
	}
}
