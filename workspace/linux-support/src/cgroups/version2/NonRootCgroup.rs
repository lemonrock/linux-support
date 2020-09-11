// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A version 2 non-root cgroup.
///
/// See <https://www.kernel.org/doc/Documentation/cgroup-v2.txt>.
///
/// By convention, a leaf non-root cgroup is called `leaf` but this is not enforced.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonRootCgroup
{
	#[allow(missing_docs)]
	ChildOfRoot
	{
		/// Folder name.
		name: CgroupName,
	},
	
	#[allow(missing_docs)]
	ChildOfAChild
	{
		/// Parent.
		parent: Rc<NonRootCgroup>,
		
		/// Folder name.
		name: CgroupName,
	}
}

impl Cgroup for NonRootCgroup
{
	#[inline(always)]
	fn to_path<'b>(&self, mount_point: &'b CgroupMountPoint) -> Cow<'b, Path>
	{
		use self::NonRootCgroup::*;
		
		let path = match self
		{
			&ChildOfRoot { ref name} => RootCgroup.to_owned_path(mount_point).append(name),
			&ChildOfAChild { ref name, ref parent } => parent.to_owned_path(mount_point).append(name),
		};
		Cow::Owned(path)
	}
	
	#[inline(always)]
	fn child(self: Rc<Self>, name: CgroupName) -> Rc<Self>
	{
		Rc::new(NonRootCgroup::ChildOfAChild { parent: self, name })
	}
}

impl NonRootCgroup
{
	/// Name.
	#[inline(always)]
	pub fn name(&self) -> &CgroupName
	{
		use self::NonRootCgroup::*;
		
		match self
		{
			&ChildOfRoot { ref name} => name,
			
			&ChildOfAChild { ref name, .. } => name,
		}
	}
	
	/// Creates, including parent cgroups, if does not already exist.
	///
	/// Short-circuits creation if already exists (to avoid permission failures).
	#[inline(always)]
	pub fn create(&self, mount_point: &CgroupMountPoint) -> io::Result<()>
	{
		let folder_path = self.to_path(mount_point);
		
		if folder_path.exists()
		{
			return Ok(())
		}
		
		create_dir_all(&folder_path)
	}
	
	/// Removes, excluding parent cgroups, if exists.
	///
	/// Short-circuits creation if does not exist (to avoid permission failures).
	pub fn remove(&self, mount_point: &CgroupMountPoint) -> io::Result<()>
	{
		let folder_path = self.to_path(mount_point);
		
		if !folder_path.exists()
		{
			return Ok(())
		}
		
		remove_dir(&folder_path)
	}

	/// Read type.
	#[inline(always)]
	pub fn read_type(&self, mount_point: &CgroupMountPoint) -> io::Result<NonRootCgroupType>
	{
		self.cgroup_type_file_path(mount_point).read_value()
	}
	
	/// Write type.
	#[inline(always)]
	pub fn make_type_threaded(&self, mount_point: &CgroupMountPoint) -> io::Result<()>
	{
		let path = self.cgroup_type_file_path(mount_point);
		path.write_value(b"threaded\n" as &[u8])
	}
	
	/// Freeze the cgroup.
	///
	/// All processes in this and every descendant cgroup, will be stopped not run until the cgroup is be explicitly thawed.
	#[inline(always)]
	pub fn freeze(&self, mount_point: &CgroupMountPoint) -> io::Result<()>
	{
		self.cgroup_freeze_file_path(mount_point).write_value(true)
	}
	
	/// Thaw the cgroup.
	///
	/// All processes in this and every descendant cgroup, will be run again.
	#[inline(always)]
	pub fn thaw(&self, mount_point: &CgroupMountPoint) -> io::Result<()>
	{
		self.cgroup_freeze_file_path(mount_point).write_value(false)
	}
	
	/// Is this cgroup frozen?
	#[inline(always)]
	pub fn is_frozen(&self, mount_point: &CgroupMountPoint) -> io::Result<bool>
	{
		self.cgroup_freeze_file_path(mount_point).read_zero_or_one_bool()
	}

	/// Events.
	#[inline(always)]
	pub fn read_events(&self, mount_point: &CgroupMountPoint) -> Result<EventStatistics, StatisticsParseError>
	{
		EventStatistics::from_file(&self.cgroup_events_file_path(mount_point))
	}

	/// Events file descriptor for epoll.
	///
	/// Caller must close this file descriptor.
	#[inline(always)]
	pub fn events_file_descriptor_for_epoll(&self, mount_point: &CgroupMountPoint) -> io::Result<RawFd>
	{
		let file: File = File::open(self.cgroup_events_file_path(mount_point))?;
		Ok(file.into_raw_fd())
	}
	
	/// Only works if the `cpu` controller is enabled.
	#[inline(always)]
	pub fn read_cpu_weight(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<CpuWeight>>
	{
		self.cpu_weight_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `cpu` controller is enabled.
	///
	/// Does not check that the `cpu` controller is enabled.
	#[inline(always)]
	pub fn write_cpu_weight(&self, mount_point: &CgroupMountPoint, cpu_weight: CpuWeight) -> io::Result<()>
	{
		self.cpu_weight_file_path(mount_point).write_value(cpu_weight)
	}
	
	/// Only works if the `cpu` controller is enabled.
	///
	/// Prefer the used of `read_cpu_weight().`
	#[inline(always)]
	pub fn read_cpu_weight_nice(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<Nice>>
	{
		self.cpu_weight_nice_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `cpu` controller is enabled.
	///
	/// Does not check that the `cpu` controller is enabled.
	///
	/// Prefer the used of `write_cpu_weight().`
	#[inline(always)]
	pub fn write_cpu_weight_nice(&self, mount_point: &CgroupMountPoint, nice: Nice) -> io::Result<()>
	{
		self.cpu_weight_nice_file_path(mount_point).write_value(nice)
	}
	
	/// Only works if the `cpu` controller is enabled.
	#[inline(always)]
	pub fn read_cpu_maximum_bandwidth_limit(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<CpuMaximumBandwidthLimit>>
	{
		self.cpu_max_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `cpu` controller is enabled.
	///
	/// Does not check that the `cpu` controller is enabled.
	#[inline(always)]
	pub fn write_cpu_maximum_bandwidth_limit(&self, mount_point: &CgroupMountPoint, cpu_maximum_bandwidth_limit: CpuMaximumBandwidthLimit) -> io::Result<()>
	{
		self.cpu_max_file_path(mount_point).write_value(cpu_maximum_bandwidth_limit)
	}
	
	/// Only works if the `cpuset` controller is enabled.
	pub fn read_cpuset_hyper_threads(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<HyperThreads>>
	{
		self.cpuset_cpus_file_path(mount_point).read_hyper_thread_or_numa_node_list_if_exists().map(|option| option.map(HyperThreads))
	}
	
	/// Only works if the `cpuset` controller is enabled.
	///
	/// Does a cursory check that the `cpuset` controller is enabled (but is subject to a TOCTOU flaw).
	pub fn write_cpuset_hyper_threads(&self, mount_point: &CgroupMountPoint, hyper_threads: &HyperThreads) -> io::Result<()>
	{
		hyper_threads.set_affinity_list(self.cpuset_cpus_file_path(mount_point))
	}
	
	/// Only works if the `cpuset` controller is enabled.
	pub fn read_cpuset_numa_nodes(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<NumaNodes>>
	{
		self.cpuset_mems_file_path(mount_point).read_hyper_thread_or_numa_node_list_if_exists().map(|option| option.map(NumaNodes))
	}
	
	/// Only works if the `cpuset` controller is enabled.
	///
	/// Does a cursory check that the `cpuset` controller is enabled (but is subject to a TOCTOU flaw).
	pub fn write_cpuset_numa_nodes(&self, mount_point: &CgroupMountPoint, numa_nodes: &NumaNodes) -> io::Result<()>
	{
		numa_nodes.set_affinity_list(self.cpuset_mems_file_path(mount_point))
	}
	
	/// Only works if the `cpuset` controller is enabled.
	#[inline(always)]
	pub fn read_cpuset_hyper_threads_partition(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<ReadPartition>>
	{
		self.cpuset_cpus_partition_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `cpuset` controller is enabled.
	///
	/// Setting a cgroup to `Partition::Root` will take the CPUs away from the effective CPUs of the parent cgroup.
	/// Once it is set, this file cannot be reverted back to `Partition::NonRootMember` if there are any child cgroups with cpuset enabled.
	///
	/// A parent partition cannot distribute all its CPUs to its child partitions
	/// There must be at least one cpu left in the parent partition.
	///
	/// There are constraints on where a `Partition::Root` can be set.
	///
	/// It can only be set in a cgroup if all the following conditions are true:-
	///
	/// * The `cpuset.cpus` file is not empty and the list of CPUs are exclusive, ie they are not shared by any of its siblings.
	/// * The parent cgroup is a partition root.
	/// * The `cpuset.cpus` file is also a proper subset of the parent’s `cpuset.cpus.effective`.
	/// * There is no child cgroups with cpuset enabled. This is for eliminating corner cases that have to be handled if such a condition is allowed.
	#[inline(always)]
	pub fn write_cpuset_hyper_threads_partition(&self, mount_point: &CgroupMountPoint, partition: Partition) -> io::Result<()>
	{
		self.cpuset_cpus_partition_file_path(mount_point).write_value(partition)
	}
	
	/// Only works if the `hugetlb` controller is enabled and the page size exists on the architecture.
	#[inline(always)]
	pub fn read_hugetlb_current(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> io::Result<Option<usize>>
	{
		self.hugetlb_current_file_path(mount_point, huge_page_size).read_value_if_exists()
	}
	
	/// Only works if the `hugetlb` controller is enabled and the page size exists on the architecture.
	#[inline(always)]
	pub fn read_hugetlb_maximum(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> io::Result<Option<MaximumNumber<usize>>>
	{
		self.hugetlb_max_file_path(mount_point, huge_page_size).read_value_if_exists()
	}
	
	/// Only works if the `hugetlb` controller is enabled and the page size exists on the architecture.
	#[inline(always)]
	pub fn write_hugetlb_maximum(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize, maximum: MaximumNumber<usize>) -> io::Result<()>
	{
		self.hugetlb_max_file_path(mount_point, huge_page_size).write_value(maximum)
	}
	
	/// Only works if the `hugetlb` controller is enabled and the page size exists on the architecture.
	#[inline(always)]
	pub fn read_hugetlb_reserved_current(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> io::Result<Option<usize>>
	{
		self.hugetlb_rsvd_current_file_path(mount_point, huge_page_size).read_value_if_exists()
	}
	
	/// Only works if the `hugetlb` controller is enabled and the page size exists on the architecture.
	#[inline(always)]
	pub fn read_hugetlb_reserved_maximum(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> io::Result<Option<MaximumNumber<usize>>>
	{
		self.hugetlb_rsvd_max_file_path(mount_point, huge_page_size).read_value_if_exists()
	}
	
	/// Only works if the `hugetlb` controller is enabled and the page size exists on the architecture.
	#[inline(always)]
	pub fn write_hugetlb_reserved_maximum(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize, maximum: MaximumNumber<usize>) -> io::Result<()>
	{
		self.hugetlb_rsvd_max_file_path(mount_point, huge_page_size).write_value(maximum)
	}
	
	/// Only works if the `hugetlb` controller is enabled and the page size exists on the architecture.
	#[inline(always)]
	pub fn read_hugetlb_events(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> Result<HugetlbEventStatistics, StatisticsParseError>
	{
		let path = self.hugetlb_events_file_path(mount_point, huge_page_size);
		HugetlbEventStatistics::from_file(&path)
	}
	
	/// Only works if the `hugetlb` controller is enabled and the page size exists on the architecture.
	#[inline(always)]
	pub fn read_hugetlb_events_local(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> Result<HugetlbEventStatistics, StatisticsParseError>
	{
		let path = self.hugetlb_events_local_file_path(mount_point, huge_page_size);
		HugetlbEventStatistics::from_file(&path)
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_current(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<u64>>
	{
		self.memory_current_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_minimum(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<u64>>
	{
		self.memory_min_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `memory` controller is enabled.
	///
	/// Does not check that the `memory` controller is enabled.
	#[inline(always)]
	pub fn write_memory_minimum(&self, mount_point: &CgroupMountPoint, minimum: u64) -> io::Result<()>
	{
		self.memory_min_file_path(mount_point).write_value(UnpaddedDecimalInteger(minimum))
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_low(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<u64>>
	{
		self.memory_low_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `memory` controller is enabled.
	///
	/// Does not check that the `memory` controller is enabled.
	#[inline(always)]
	pub fn write_memory_low(&self, mount_point: &CgroupMountPoint, low: u64) -> io::Result<()>
	{
		self.memory_low_file_path(mount_point).write_value(UnpaddedDecimalInteger(low))
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_high(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<MaximumNumber<u64>>>
	{
		self.memory_high_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `memory` controller is enabled.
	///
	/// Does not check that the `memory` controller is enabled.
	#[inline(always)]
	pub fn write_memory_high(&self, mount_point: &CgroupMountPoint, high: MaximumNumber<u64>) -> io::Result<()>
	{
		self.memory_high_file_path(mount_point).write_value(high)
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_maximum(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<MaximumNumber<u64>>>
	{
		self.memory_max_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `memory` controller is enabled.
	///i
	/// Does not check that the `memory` controller is enabled.
	#[inline(always)]
	pub fn write_memory_maximum(&self, mount_point: &CgroupMountPoint, maximum: MaximumNumber<u64>) -> io::Result<()>
	{
		self.memory_max_file_path(mount_point).write_value(maximum)
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_events(&self, mount_point: &CgroupMountPoint) -> Result<MemoryEventStatistics, StatisticsParseError>
	{
		let path = self.memory_events_file_path(mount_point);
		MemoryEventStatistics::from_file(&path)
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_events_local(&self, mount_point: &CgroupMountPoint) -> Result<MemoryEventStatistics, StatisticsParseError>
	{
		let path = self.memory_events_local_file_path(mount_point);
		MemoryEventStatistics::from_file(&path)
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_statistics(&self, mount_point: &CgroupMountPoint) -> Result<MemoryStatistics, StatisticsParseError>
	{
		let path = self.memory_stat_file_path(mount_point);
		MemoryStatistics::from_file(&path)
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_swap_current(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<u64>>
	{
		self.memory_swap_current_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_swap_maximum(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<MaximumNumber<u64>>>
	{
		self.memory_swap_max_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `memory` controller is enabled.
	///i
	/// Does not check that the `memory` controller is enabled.
	#[inline(always)]
	pub fn write_memory_swap_maximum(&self, mount_point: &CgroupMountPoint, maximum: MaximumNumber<u64>) -> io::Result<()>
	{
		self.memory_swap_max_file_path(mount_point).write_value(maximum)
	}
	
	/// Only works if the `memory` controller is enabled.
	#[inline(always)]
	pub fn read_memory_swap_events(&self, mount_point: &CgroupMountPoint) -> Result<MemorySwapEventStatistics, StatisticsParseError>
	{
		let path = self.memory_swap_events_file_path(mount_point);
		MemorySwapEventStatistics::from_file(&path)
	}
	
	/// Only works if the `pids` controller is enabled.
	#[inline(always)]
	pub fn read_process_identifiers_count_current(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<usize>>
	{
		self.pids_current_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `pids` controller is enabled.
	#[inline(always)]
	pub fn read_process_identifiers_count_maximum(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<ProcessIdentifiersMaximum>>
	{
		self.pids_max_file_path(mount_point).read_value_if_exists()
	}
	
	/// Only works if the `pids` controller is enabled.
	#[inline(always)]
	pub fn read_process_identifiers_events(&self, mount_point: &CgroupMountPoint) -> Result<ProcessIdentifiersEventStatistics, StatisticsParseError>
	{
		let path = self.pids_events_file_path(mount_point);
		ProcessIdentifiersEventStatistics::from_file(&path)
	}
	
	/// Only works if the `pids` controller is enabled.
	///
	/// Does not check that the `pids` controller is enabled.
	#[inline(always)]
	pub fn write_process_identifiers_count_maximum(&self, mount_point: &CgroupMountPoint, maximum: ProcessIdentifiersMaximum) -> io::Result<()>
	{
		self.pids_max_file_path(mount_point).write_value(maximum)
	}
	
	/// Only works if the `rdma` controller is enabled.
	#[inline(always)]
	pub fn read_rdma_current(&self, mount_point: &CgroupMountPoint) -> Result<RdmaFile, RdmaParseError>
	{
		let file_path = self.rdma_current_file_path(mount_point);
		RdmaFile::from_file(&file_path)
	}
	
	/// Only works if the `rdma` controller is enabled.
	#[inline(always)]
	pub fn read_rdma_maximum(&self, mount_point: &CgroupMountPoint) -> Result<RdmaFile, RdmaParseError>
	{
		let file_path = self.rdma_max_file_path(mount_point);
		RdmaFile::from_file(&file_path)
	}
	
	/// Only works if the `rdma` controller is enabled.
	///i
	/// Does not check that the `rdma` controller is enabled.
	#[inline(always)]
	pub fn write_rdma_maximum(&self, mount_point: &CgroupMountPoint, maximum: &RdmaFile) -> io::Result<()>
	{
		maximum.to_file(&self.rdma_max_file_path(mount_point))
	}
	
	#[inline(always)]
	fn cgroup_events_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.events")
	}
	
	#[inline(always)]
	fn cgroup_freeze_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.freeze")
	}
	
	#[inline(always)]
	fn cgroup_type_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.type")
	}

	#[inline(always)]
	fn cpu_weight_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cpu.weight")
	}

	#[inline(always)]
	fn cpu_weight_nice_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cpu.weight.nice")
	}

	#[inline(always)]
	fn cpu_max_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cpu.max")
	}
	
	#[inline(always)]
	fn cpuset_cpus_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cpuset.cpus")
	}
	
	#[inline(always)]
	fn cpuset_cpus_partition_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cpuset.cpus.partition")
	}
	
	#[inline(always)]
	fn cpuset_mems_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cpuset.mems")
	}
	
	#[inline(always)]
	fn hugetlb_current_file_path(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> PathBuf
	{
		self.hugetlb_file_path(mount_point, huge_page_size, "current")
	}
	
	#[inline(always)]
	fn hugetlb_events_file_path(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> PathBuf
	{
		self.hugetlb_file_path(mount_point, huge_page_size, "events")
	}
	
	#[inline(always)]
	fn hugetlb_events_local_file_path(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> PathBuf
	{
		self.hugetlb_file_path(mount_point, huge_page_size, "events.local")
	}
	
	#[inline(always)]
	fn hugetlb_max_file_path(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> PathBuf
	{
		self.hugetlb_file_path(mount_point, huge_page_size, "max")
	}
	
	#[inline(always)]
	fn hugetlb_rsvd_current_file_path(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> PathBuf
	{
		self.hugetlb_file_path(mount_point, huge_page_size, "rsvd.current")
	}
	
	#[inline(always)]
	fn hugetlb_rsvd_max_file_path(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize) -> PathBuf
	{
		self.hugetlb_file_path(mount_point, huge_page_size, "rsvd.max")
	}
	
	#[inline(always)]
	fn hugetlb_file_path(&self, mount_point: &CgroupMountPoint, huge_page_size: HugePageSize, file_extension: &str) -> PathBuf
	{
		self.file_path(mount_point, &format!("hugetlb.{}.{}", huge_page_size.cgroup_file_name_fragment(), file_extension))
	}
	
	#[inline(always)]
	fn memory_current_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.current")
	}
	
	#[inline(always)]
	fn memory_min_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.min")
	}
	
	#[inline(always)]
	fn memory_low_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.low")
	}
	
	#[inline(always)]
	fn memory_high_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.high")
	}
	
	#[inline(always)]
	fn memory_max_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.max")
	}
	
	#[inline(always)]
	fn memory_events_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.events")
	}
	
	#[inline(always)]
	fn memory_events_local_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.events.local")
	}
	
	#[inline(always)]
	fn memory_stat_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.stat")
	}
	
	#[inline(always)]
	fn memory_swap_current_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.swap.current")
	}
	
	#[inline(always)]
	fn memory_swap_max_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.swap.max")
	}
	
	#[inline(always)]
	fn memory_swap_events_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.swap.events")
	}
	
	#[inline(always)]
	fn pids_current_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "pids.current")
	}

	#[inline(always)]
	fn pids_events_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "pids.events")
	}

	#[inline(always)]
	fn pids_max_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "pids.max")
	}

	#[inline(always)]
	fn rdma_max_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "rdma.max")
	}
	
	#[inline(always)]
	fn rdma_current_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "rdma.current")
	}

	#[inline(always)]
	fn file_path(&self, mount_point: &CgroupMountPoint, file_name: &str) -> PathBuf
	{
		self.to_path(mount_point).to_path_buf().append(file_name)
	}
}
