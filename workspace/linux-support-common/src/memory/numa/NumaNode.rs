// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A NUMA node.
///
/// Internally, this is modelled as an unsigned 16-bit value as `CONFIG_NODES_SHIFT` can be as high as 10, giving a maximum of 1024 nodes, even though in practice:-
///
/// * most Linux systems have very NUMA nodes;
/// * most never change `CONFIG_NODES_SHIFT` from its default of 6 (which gives a maximum of 64 nodes).
///
/// Indeed, the trend in modern x86-64 systems with CPUs such as AMD's Zen2 is to have just 2 NUMA nodes for 128 cores.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NumaNode(u16);

bit_set_aware!(NumaNode);

impl Into<u16> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl BitSetAware for NumaNode
{
	const LinuxMaximum: u16 = 1 << Self::LinuxMaximumFor_CONFIG_NUMA_SHIFT;

	const InclusiveMinimum: Self = Self(0);

	const InclusiveMaximum: Self = Self(Self::LinuxMaximum - 1);

	const Prefix: &'static [u8] = b"node";

	#[inline(always)]
	fn hydrate(value: u16) -> Self
	{
		debug_assert!(value < Self::LinuxMaximum);

		Self(value)
	}
}

impl NumaNode
{
	pub(crate) const LinuxMaximumFor_CONFIG_NUMA_SHIFT: u16 = 10;

	/// Reads the hyper thread and NUMA node of the currently executing CPU from the `IA32_TSC_AUX` model state register, which Linux populates.
	#[inline(always)]
	pub fn current() -> (Self, HyperThread)
	{
		current_numa_node_and_hyper_thread()
	}

	/// True if the Linux kernel was configured with `CONFIG_NUMA` and `/sys` is mounted.
	#[inline(always)]
	pub fn is_a_numa_machine(sys_path: &SysPath) -> bool
	{
		sys_path.numa_nodes_folder_path().exists()
	}

	/// Is this a NUMA node?
	///
	/// Note that this might be a fake NUMA node, ie one lacking any hyper threads.
	///
	/// Prefer `self.associated_hyper_threads().len() > 0`.
	///
	/// This will return false if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	///
	#[inline(always)]
	pub fn is_a_numa_node(self, sys_path: &SysPath) -> bool
	{
		sys_path.numa_node_folder_path(self).exists()
	}

	/// Hyper threads associated with this NUMA node.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	///
	/// Hyper threads themselves have not been validated for being online, etc.
	#[inline(always)]
	pub fn associated_hyper_threads(self, sys_path: &SysPath) -> Option<BitSet<HyperThread>>
	{
		let cpu_map = self.cpu_map(sys_path);
		let cpu_list = self.cpu_list(sys_path);
		debug_assert_eq!(cpu_map, cpu_list);

		let mut valid = self.has_a_folder_path(sys_path)?;
		valid.intersection(&cpu_map?);

		Some(self.remove_hyper_threads_that_do_not_have_a_mapping_to_this_numa_node(valid, sys_path))
	}

	/// How many files match `Y` in `/sys/devices/system/node/node<Self>/cpu<Y>`?
	#[inline(always)]
	fn has_a_folder_path(self, sys_path: &SysPath) -> Option<BitSet<HyperThread>>
	{
		sys_path.numa_node_folder_path(self).entries_in_folder_path::<HyperThread>().unwrap()
	}

	#[inline(always)]
	fn cpu_map(self, sys_path: &SysPath) -> Option<BitSet<HyperThread>>
	{
		let file_path = sys_path.numa_node_file_path(self, "cpumap");
		if file_path.exists()
		{
			Some(file_path.parse_hyper_thread_or_numa_node_bit_set::<HyperThread>().unwrap())
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	fn cpu_list(self, sys_path: &SysPath) -> Option<BitSet<HyperThread>>
	{
		let file_path = sys_path.numa_node_file_path(self, "cpulist");
		if file_path.exists()
		{
			Some(file_path.read_hyper_thread_or_numa_node_list::<HyperThread>().unwrap())
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	fn remove_hyper_threads_that_do_not_have_a_mapping_to_this_numa_node(self, mut valid: BitSet<HyperThread>, sys_path: &SysPath) -> BitSet<HyperThread>
	{
		let mut invalid_hyper_threads = BitSet::<HyperThread>::new();
		for hyper_thread in valid.iterate()
		{
			if let Some(numa_node) = hyper_thread.numa_node(sys_path)
			{
				if numa_node != self
				{
					invalid_hyper_threads.add(hyper_thread)
				}
			}
			else
			{
				invalid_hyper_threads.add(hyper_thread)
			}
		}
		valid.remove_all(&invalid_hyper_threads);
		valid
	}

	/// Migrates all pages from one set of NUMA nodes to another.
	///
	/// `from` and `to` ought to be the same size.
	/// If they aren't, then one of them is cloned.
	///
	/// `PageMoveError` `TargetNodeNotAllowed`, `OneOrMoreTargetNodesIsNotOnline` and `CallerNeedsToHaveSysNiceCapabilityForMoveAll` do not occur.
	pub fn migrate_all_pages(process_identifier: pid_t, from: &BitSet<Self>, to: &BitSet<Self>) -> Result<(), PageMoveError>
	{
		let from_length = from.capacity();
		let to_length = to.capacity();

		use self::Ordering::*;
		let (from, to) = match from_length.cmp(&to_length)
		{
			Less => (Cow::Owned(from.extend_clone_to(to_length)), Cow::Borrowed(to)),

			Equal => (Cow::Borrowed(from), Cow::Borrowed(to)),

			Greater => (Cow::Borrowed(from), Cow::Owned(from.extend_clone_to(from_length))),
		};

		let result = migrate_pages(process_identifier, from_length, from.as_ref().to_raw_parts().0, to.as_ref().to_raw_parts().0);

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::PageMoveError::*;

			match errno().0
			{
				EACCES => Err(TargetNodeNotAllowed),
				ENODEV => Err(OneOrMoreTargetNodesIsNotOnline),
				ESRCH => Err(ProcessDoesNotExist(process_identifier)),
				EPERM => if process_identifier == 0
				{
					panic!("We need to have CAP_SYS_NICE for ourselves?!")
				}
				else
				{
					Err(CallerNeedsToHaveSysNiceCapabilityToMoveAnotherPagesOfAnotherProcess(process_identifier))
				},

				EINVAL => panic!("Flags other than MPOL_MF_MOVE and MPOL_MF_MOVE_ALL was specified or an attempt was made to migrate pages of a kernel thread"),
				E2BIG => panic!("Kernel should not generate E2BIG"),

				unexpected @ _ => panic!("Unexpected error number '{}'", unexpected),
			}
		}
		else
		{
			panic!("Unexpected result {}", result);
		}
	}

	/// Move pages to another NUMA node.
	///
	/// See also `move_pages()`.
	///
	/// `PageMoveError` `TargetNodeNotAllowed`, `OneOrMoreTargetNodesIsNotOnline` and `CallerNeedsToHaveSysNiceCapabilityForMoveAll` do not occur.
	#[inline(always)]
	pub fn status_of_pages(process_identifier: pid_t, pages: &[NonNull<u8>]) -> Result<Box<[Self]>, PageMoveError>
	{
		let count = pages.len();
		if unlikely!(count == 0)
		{
			return Ok(Vec::new().into_boxed_slice())
		}

		let mut status: Vec<Self> = Vec::with_capacity(count);

		let result = syscall::move_pages(process_identifier, count, pages.as_ptr() as *const *const c_void, null(), status.as_mut_ptr() as *mut i32, MemoryBindFlags::empty());

		if likely!(result == 0)
		{
			unsafe { status.set_len(count) }
			Ok(status.into_boxed_slice())
		}
		else if likely!(result == -1)
		{
			use self::PageMoveError::*;

			match errno().0
			{
				EACCES => panic!("TargetNodeNotAllowed"),
				ENODEV => panic!("OneOrMoreTargetNodesIsNotOnline"),
				ESRCH => Err(ProcessDoesNotExist(process_identifier)),
				EPERM => if process_identifier == 0
				{
					panic!("We need to have CAP_SYS_NICE for ourselves?!")
				}
				else
				{
					Err(CallerNeedsToHaveSysNiceCapabilityToMoveAnotherPagesOfAnotherProcess(process_identifier))
				},

				EINVAL => panic!("Flags other than MPOL_MF_MOVE and MPOL_MF_MOVE_ALL was specified or an attempt was made to migrate pages of a kernel thread"),
				E2BIG => panic!("Kernel should not generate E2BIG"),

				unexpected @ _ => panic!("Unexpected error number '{}'", unexpected),
			}
		}
		else
		{
			panic!("Unknown result '{}'", result)
		}
	}

	/// Move pages to another NUMA node.
	///
	/// See also `NumaNode::status_of_pages()`.
	#[inline(always)]
	pub fn move_pages(process_identifier: pid_t, pages_to_move: &[(NonNull<u8>, NumaNode)], move_all: bool) -> Result<Box<[PageMoveStatus]>, PageMoveError>
	{
		let count = pages_to_move.len();
		if unlikely!(count == 0)
		{
			return Ok(Vec::new().into_boxed_slice())
		}

		let mut status: Vec<PageMoveStatus> = Vec::with_capacity(count);

		let mut pages: Vec<*const c_void> = Vec::with_capacity(count);
		let mut nodes: Vec<i32> = Vec::with_capacity(count);
		for &(non_null, numa_node) in pages_to_move
		{
			pages.push(non_null.as_ptr() as *const c_void);
			nodes.push(numa_node.into());
		}

		let flags = if move_all
		{
			MemoryBindFlags::MPOL_MF_MOVE_ALL
		}
		else
		{
			MemoryBindFlags::MPOL_MF_MOVE
		};
		let result = syscall::move_pages(process_identifier, count, pages.as_ptr() as *const *const c_void, nodes.as_ptr(), status.as_mut_ptr() as *mut i32, flags);

		if likely!(result == 0)
		{
			unsafe { status.set_len(count) }
			Ok(status.into_boxed_slice())
		}
		else if likely!(result == -1)
		{
			use self::PageMoveError::*;

			match errno().0
			{
				EACCES => Err(TargetNodeNotAllowed),
				ENODEV => Err(OneOrMoreTargetNodesIsNotOnline),
				ESRCH => Err(ProcessDoesNotExist(process_identifier)),
				EPERM => match move_all
				{
					true => Err(CallerNeedsToHaveSysNiceCapabilityForMoveAll),
					false => if process_identifier == 0
					{
						panic!("We need to have CAP_SYS_NICE for ourselves?!")
					}
					else
					{
						Err(CallerNeedsToHaveSysNiceCapabilityToMoveAnotherPagesOfAnotherProcess(process_identifier))
					},
				},

				EINVAL => panic!("Flags other than MPOL_MF_MOVE and MPOL_MF_MOVE_ALL was specified or an attempt was made to migrate pages of a kernel thread"),
				E2BIG => panic!("Kernel should not generate E2BIG"),

				unexpected @ _ => panic!("Unexpected error number '{}'", unexpected),
			}
		}
		else
		{
			panic!("Unknown result '{}'", result)
		}
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

	/// Value used for distance calculations.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`.
	///
	/// Minimum value seems to be 10.
	#[inline(always)]
	pub fn distance(self, sys_path: &SysPath) -> Option<u8>
	{
		let file_path = sys_path.numa_node_file_path(self.into(), "distance");
		if file_path.exists()
		{
			Some(file_path.read_value().unwrap())
		}
		else
		{
			None
		}
	}

	/// This is a subset of `self.zoned_virtual_memory_statistics()`.
	///
	/// This will return `Err()` if this the Linux kernel wasn't configured with `CONFIG_NUMA` or the NUMA node is not present.
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
	/// This will return `Err()` if this the Linux kernel wasn't configured with `CONFIG_NUMA` or the NUMA node is not present.
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
	/// This will return `Err()` if this the Linux kernel wasn't configured with `CONFIG_NUMA` or the NUMA node is not present.
	#[inline(always)]
	pub fn memory_information(self, sys_path: &SysPath, memory_information_name_prefix: &[u8]) -> Result<MemoryInformation, MemoryInformationParseError>
	{
		let file_path = sys_path.numa_node_file_path(self.into(), "meminfo");
		MemoryInformation::parse_memory_information_file(&file_path, memory_information_name_prefix)
	}
}
