// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A NUMA node.
///
/// Internally, this is modelled as an unsigned 16-bit value as `CONFIG_NODES_SHIFT` can be as high as 10, giving a maximum of 1024 nodes, even though in practice:-
///
/// * most Linux systems have very NUMA nodes;
/// * most never change `CONFIG_NODES_SHIFT` from its default of 6 (which gives a maximum of 64 nodes).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
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
	/// Internally, code in Linux checks that `number of nodes x 4 <= page size`.
	/// This is to handle writing the `distance` files in sysfs (see `node_read_distance()` in `drivers/base/node.c`).
	/// This logic, on x86_64, caps the maximum number of nodes to 1024, which is also the maximum here.
	/// An upper bound on x86_64 is also `2^12 - 1`, the maximum value encoded into the IA32_TSC_AUX MSR.
	const LinuxExclusiveMaximum: u16 = 1 << Self::LinuxMaximumFor_CONFIG_NUMA_SHIFT;

	const InclusiveMinimum: Self = Self(0);

	const InclusiveMaximum: Self = Self(Self::LinuxExclusiveMaximum - 1);

	const Prefix: &'static [u8] = b"node";

	#[inline(always)]
	fn from_validated_u16(value: u16) -> Self
	{
		debug_assert!(value < Self::LinuxExclusiveMaximum);

		Self(value)
	}
}

impl TryFrom<i32> for NumaNode
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn try_from(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error>
	{
		use self::ParseNumberError::*;

		if unlikely!(value < 0)
		{
			Err(TooSmall)
		}
		else if unlikely!(value >= Self::LinuxExclusiveMaximum as i32)
		{
			Err(TooLarge)
		}
		else
		{
			Ok(Self(value as u16))
		}
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

	/// This will be false if the Linux kernel wasn't configured with `CONFIG_NUMA` or `sys_path` is not mounted.
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
	/// This will be false if the Linux kernel wasn't configured with `CONFIG_NUMA` or `sys_path` is not mounted.
	#[inline(always)]
	pub fn is_a_numa_node(self, sys_path: &SysPath) -> bool
	{
		sys_path.numa_node_folder_path(self).exists()
	}

	/// Hyper threads associated with this NUMA node.
	///
	/// This will do nothing if the Linux kernel wasn't configured with `CONFIG_NUMA`, the NUMA node is not present or `sys_path` is not mounted.
	///
	/// Hyper threads themselves have not been validated for being online, etc.
	#[inline(always)]
	pub fn associated_hyper_threads(self, sys_path: &SysPath) -> Option<HyperThreads>
	{
		let cpu_map = self.cpu_map(sys_path);
		let cpu_list = self.cpu_list(sys_path);
		debug_assert_eq!(cpu_map, cpu_list);

		let mut valid = self.has_a_folder_path(sys_path)?;
		valid.intersection(&(&cpu_map?).0);

		Some(self.remove_hyper_threads_that_do_not_have_a_mapping_to_this_numa_node(valid, sys_path))
	}

	/// How many files match `Y` in `/sys/devices/system/node/node<Self>/cpu<Y>`?
	#[inline(always)]
	fn has_a_folder_path(self, sys_path: &SysPath) -> Option<HyperThreads>
	{
		sys_path.numa_node_folder_path(self).entries_in_folder_path::<HyperThread>().unwrap().map(|bit_set| HyperThreads(bit_set))
	}

	#[inline(always)]
	fn cpu_map(self, sys_path: &SysPath) -> Option<HyperThreads>
	{
		self.only_if_path_exists(sys_path, "cpumap", |file_path | file_path.parse_comma_separated_bit_set::<HyperThread>().map(|bit_set| HyperThreads(bit_set)))
	}

	#[inline(always)]
	fn cpu_list(self, sys_path: &SysPath) -> Option<HyperThreads>
	{
		self.only_if_path_exists(sys_path, "cpulist", |file_path | file_path.read_hyper_thread_or_numa_node_list::<HyperThread>().map(|bit_set| HyperThreads(bit_set)))
	}

	#[inline(always)]
	fn remove_hyper_threads_that_do_not_have_a_mapping_to_this_numa_node(self, mut valid: HyperThreads, sys_path: &SysPath) -> HyperThreads
	{
		let mut invalid_hyper_threads = HyperThreads(BitSet::<HyperThread>::new());
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
	pub fn migrate_all_pages(process_identifier: ProcessIdentifierChoice, from: &BitSet<Self>, to: &BitSet<Self>) -> Result<(), PageMoveError>
	{
		let from_length = from.capacity_in_words();
		let to_length = to.capacity_in_words();

		use self::Ordering::*;
		let (from, to) = match from_length.cmp(&to_length)
		{
			Less => (Cow::Owned(from.extend_clone_to(to_length)), Cow::Borrowed(to)),

			Equal => (Cow::Borrowed(from), Cow::Borrowed(to)),

			Greater => (Cow::Borrowed(from), Cow::Owned(from.extend_clone_to(from_length))),
		};
		
		use self::PageMoveError::*;

		match system_call_migrate_pages(process_identifier.into(), from_length, from.as_ref().to_raw_parts().0, to.as_ref().to_raw_parts().0).as_usize()
		{
			0 => Ok(()),
			
			SystemCallResult::EACCES_usize => Err(TargetNodeNotAllowed),
			SystemCallResult::ENODEV_usize => Err(OneOrMoreTargetNodesIsNotOnline),
			SystemCallResult::ESRCH_usize => if let ProcessIdentifierChoice::Other(process_identifier) = process_identifier
			{
				Err(ProcessDoesNotExist(process_identifier))
			}
			else
			{
				panic!("We got ESRCH for ourselves?!")
			},
			SystemCallResult::EPERM_usize => if let ProcessIdentifierChoice::Other(process_identifier) = process_identifier
			{
				Err(CallerNeedsToHaveSysNiceCapabilityToMoveAnotherPagesOfAnotherProcess(process_identifier))
			}
			else
			{
				panic!("We need to have CAP_SYS_NICE for ourselves?!")
			},
			SystemCallResult::EINVAL_usize => panic!("Flags other than MPOL_MF_MOVE and MPOL_MF_MOVE_ALL was specified or an attempt was made to migrate pages of a kernel thread"),
			SystemCallResult::E2BIG_usize => panic!("Kernel should not generate E2BIG"),
			unexpected_error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(migrate_pages, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
			
			unexpected @ _ => unexpected_result!(migrate_pages, unexpected),
		}
	}

	/// Move pages to another NUMA node.
	///
	/// See also `move_pages()`.
	///
	/// `PageMoveError` `TargetNodeNotAllowed`, `OneOrMoreTargetNodesIsNotOnline` and `CallerNeedsToHaveSysNiceCapabilityForMoveAll` do not occur.
	#[inline(always)]
	pub fn status_of_pages(process_identifier: ProcessIdentifierChoice, pages: &[NonNull<u8>]) -> Result<Box<[Self]>, PageMoveError>
	{
		use PageMoveError::*;
		use ProcessIdentifierChoice::Other;
		
		let count = pages.len();
		if unlikely!(count == 0)
		{
			return Ok(Vec::new().into_boxed_slice())
		}

		let mut status: Vec<Self> = Vec::with_capacity(count);

		match system_call_move_pages(process_identifier.into(), count, pages.as_ptr() as *const *const c_void, null(), status.as_mut_ptr() as *mut i32, MemoryBindFlags::empty()).as_usize()
		{
			0 =>
			{
				unsafe { status.set_len(count) }
				Ok(status.into_boxed_slice())
			}
			
			SystemCallResult::EACCES_usize => panic!("TargetNodeNotAllowed"),
			SystemCallResult::ENODEV_usize => panic!("OneOrMoreTargetNodesIsNotOnline"),
			SystemCallResult::ESRCH_usize => if let Other(process_identifier) = process_identifier
			{
				Err(ProcessDoesNotExist(process_identifier))
			}
			else
			{
				panic!("We got ESRCH for ourselves?!")
			},
			SystemCallResult::EPERM_usize => if let Other(process_identifier) = process_identifier
			{
				Err(CallerNeedsToHaveSysNiceCapabilityToMoveAnotherPagesOfAnotherProcess(process_identifier))
			}
			else
			{
				panic!("We need to have CAP_SYS_NICE for ourselves?!")
			},
			SystemCallResult::EINVAL_usize => panic!("Flags other than MPOL_MF_MOVE and MPOL_MF_MOVE_ALL was specified or an attempt was made to migrate pages of a kernel thread"),
			SystemCallResult::E2BIG_usize => panic!("Kernel should not generate E2BIG"),
			unexpected_error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(move_pages, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
			
			unexpected @ _ => unexpected_result!(move_pages, unexpected),
		}
	}

	/// Move pages to another NUMA node.
	///
	/// See also `NumaNode::status_of_pages()`.
	#[inline(always)]
	pub fn move_pages(process_identifier: ProcessIdentifierChoice, pages_to_move: &[(NonNull<u8>, NumaNode)], move_all: bool) -> Result<Box<[PageMoveStatus]>, PageMoveError>
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
		
		use PageMoveError::*;
		use ProcessIdentifierChoice::Other;
		match system_call_move_pages(process_identifier.into(), count, pages.as_ptr() as *const *const c_void, nodes.as_ptr(), status.as_mut_ptr() as *mut i32, flags).as_usize()
		{
			0 =>
			{
				unsafe { status.set_len(count) }
				Ok(status.into_boxed_slice())
			}
			
			SystemCallResult::EACCES_usize => Err(TargetNodeNotAllowed),
			SystemCallResult::ENODEV_usize => Err(OneOrMoreTargetNodesIsNotOnline),
			SystemCallResult::ESRCH_usize => if let Other(process_identifier) = process_identifier
			{
				Err(ProcessDoesNotExist(process_identifier))
			}
			else
			{
				panic!("We got ESRCH for ourselves?!")
			},
			SystemCallResult::EPERM_usize => match move_all
			{
				true => Err(CallerNeedsToHaveSysNiceCapabilityForMoveAll),
				false => if let Other(process_identifier) = process_identifier
				{
					Err(CallerNeedsToHaveSysNiceCapabilityToMoveAnotherPagesOfAnotherProcess(process_identifier))
				}
				else
				{
					panic!("We need to have CAP_SYS_NICE for ourselves?!")
				},
			},
			SystemCallResult::EINVAL_usize => panic!("Flags other than MPOL_MF_MOVE and MPOL_MF_MOVE_ALL was specified or an attempt was made to migrate pages of a kernel thread"),
			SystemCallResult::E2BIG_usize => panic!("Kernel should not generate E2BIG"),
			unexpected_error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(move_pages, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
			
			unexpected @ _ => unexpected_result!(move_pages, unexpected),
		}
	}

	/// Tries to compact pages for this NUMA node.
	///
	/// This will do nothing if the Linux kernel wasn't configured with `CONFIG_NUMA`, the NUMA node is not present or `sys_path` is not mounted.
	///
	/// Will panic if the current user is not root.
	#[inline(always)]
	pub fn compact_pages(self, sys_path: &SysPath)
	{
		assert_effective_user_id_is_root(&format!("Compact pages in NUMA node '{}'", self.0));
		
		self.only_if_path_exists(sys_path, "compact", |file_path| file_path.write_value(true));
	}

	/// Huge page pool statistics.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`, the NUMA node is not present or `sys_path` is not mounted.
	///
	/// This will panic if the kernel was compiled without `CONFIG_HUGETLBFS` and the `hugepages` folder is missing under the `node<N>` folder.
	#[inline(always)]
	pub fn huge_page_pool_statistics(self, sys_path: &SysPath, huge_page_size: HugePageSize) -> Option<HugePagePoolStatistics>
	{
		HugePagePoolStatistics::new(sys_path, huge_page_size, |sys_path, huge_page_size| sys_path.numa_node_hugepages_folder_path(huge_page_size, self))
	}

	/// Memory latency costs to other nodes relative to this node.
	///
	/// Only nodes Linux considers 'online' will be included.
	///
	/// This will return `None` if the Linux kernel wasn't configured with `CONFIG_NUMA`, the NUMA node is not present or `sys_path` is not mounted.
	///
	/// This will return `Some(None)` if the list of online NUMA nodes differs to the list of distances; this can happen if the list of online nodes changes whilst being used.
	///
	/// The relative memory latency for 'self' will be present.
	#[inline(always)]
	pub fn distances(self, sys_path: &SysPath) -> Option<Option<HashMap<NumaNode, MemoryLatencyRelativeCost>>>
	{
		const LinuxMaximum: usize = NumaNode::LinuxExclusiveMaximum as usize;

		fn parser(file_path: &Path, mut online_numa_nodes: BitSetIterator<NumaNode>) -> io::Result<Option<HashMap<NumaNode, MemoryLatencyRelativeCost>>>
		{
			let line = file_path.read_raw_without_line_feed()?;

			let mut raw_distances = line.split_bytes_n(LinuxMaximum, b' ').peekable();

			// NOTE: We do not use `Iterator.zip()`.
			// This is because there is a very slight chance the `online_numa_nodes` list we have may have diverged from that used internally in Linux to generate the distances.
			// By not using `zip()`, we can detect this and fail.

			// NOTE: It is legitimate for there to be no distances at all, because this NUMA node is not online.
			if raw_distances.peek().is_none()
			{
				return Ok(Some(HashMap::new()))
			}

			let mut map = HashMap::with_capacity(LinuxMaximum);

			for raw_distance in raw_distances
			{
				let online_numa_node = match online_numa_nodes.next()
				{
					None => return Ok(None),
					Some(online_numa_node) => online_numa_node,
				};
				let distance = MemoryLatencyRelativeCost::from_bytes(raw_distance).map_err(io_error_other)?;
				map.insert(online_numa_node, distance);
			}

			if unlikely!(online_numa_nodes.next().is_some())
			{
				return Ok(None)
			}

			map.shrink_to_fit();
			Ok(Some(map))
		}

		let online = NumaNodes::online(sys_path)?;
		let online = online.iterate();

		self.only_if_path_exists(sys_path, "distance", |file_path| parser(file_path, online))
	}

	/// This is a subset of `self.zoned_virtual_memory_statistics()`.
	///
	/// This will return `None` if this the Linux kernel wasn't configured with `CONFIG_NUMA`, the NUMA node is not present or `sys_path` is not mounted.
	///
	/// Interpret this by multiplying counts by page size.
	#[deprecated]
	#[inline(always)]
	pub fn numa_memory_statistics(self, sys_path: &SysPath, flush_per_cpu_statistics_first: Option<&ProcPath>) -> Option<HashMap<VirtualMemoryStatisticName, u64>>
	{
		self.only_if_path_exists(sys_path, "numastat", |file_path| VirtualMemoryStatisticName::parse_virtual_memory_statistics_file(file_path, flush_per_cpu_statistics_first))
	}

	/// Memory statistics.
	///
	/// This will return `None` if this the Linux kernel wasn't configured with `CONFIG_NUMA`, the NUMA node is not present or `sys_path` is not mounted.
	///
	/// Interpret this by multiplying counts by page size.
	#[inline(always)]
	pub fn zoned_virtual_memory_statistics(self, sys_path: &SysPath, flush_per_cpu_statistics_first: Option<&ProcPath>) -> Option<HashMap<VirtualMemoryStatisticName, u64>>
	{
		self.only_if_path_exists(sys_path, "vmstat", |file_path| VirtualMemoryStatisticName::parse_virtual_memory_statistics_file(file_path, flush_per_cpu_statistics_first))
	}

	/// Memory information.
	///
	/// This will return `None` if this the Linux kernel wasn't configured with `CONFIG_NUMA`, the NUMA node is not present or `sys_path` is not mounted.
	#[inline(always)]
	pub fn memory_information(self, sys_path: &SysPath, flush_per_cpu_statistics_first: Option<&ProcPath>) -> Option<MemoryInformation>
	{
		let mut buffer: [u8; 11] = unsafe_uninitialized();

		let memory_information_name_prefix =
		{
			buffer.set_unchecked_mut_safe(10, b' ');

			let last_decimal_digit_index = self.0.decimal(9, &mut buffer);

			buffer.set_unchecked_mut_safe(last_decimal_digit_index - 5, b'N');
			buffer.set_unchecked_mut_safe(last_decimal_digit_index - 4, b'o');
			buffer.set_unchecked_mut_safe(last_decimal_digit_index - 3, b'd');
			buffer.set_unchecked_mut_safe(last_decimal_digit_index - 2, b'e');
			buffer.set_unchecked_mut_safe(last_decimal_digit_index - 1, b' ');

			&buffer[last_decimal_digit_index - 5 .. ]
		};

		self.only_if_path_exists(sys_path, "meminfo", |file_path| MemoryInformation::parse_memory_information_file(&file_path, memory_information_name_prefix, flush_per_cpu_statistics_first))
	}

	#[inline(always)]
	fn only_if_path_exists<R, E: error::Error>(self, sys_path: &SysPath, file_name: &str, parser: impl FnOnce(&Path) -> Result<R, E>) -> Option<R>
	{
		let file_path = sys_path.numa_node_file_path(self.into(), file_name);
		if likely!(file_path.exists())
		{
			Some(parser(&file_path).unwrap())
		}
		else
		{
			None
		}
	}
}
