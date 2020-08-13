// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Properties common to a root and a non-root cgroup.
pub trait Cgroup<'name>
{
	/// To a path.
	fn to_path<'b>(&self, mount_point: &'b CgroupMountPoint) -> Cow<'b, Path>;
	
	/// Does not check if the child exists.
	fn child(self: Rc<Self>, name: impl Into<Cow<'name, CgroupName>>) -> Rc<NonRootCgroup<'name>>;
	
	/// Adjusts subtree controllers to `desired_controllers` but working so that controllers not available to the cgroup aren't enabled (trying to enable or disable controllers not available in the version of Linux causes a write error).
	#[inline(always)]
	fn change_subtree_controllers_taking_account_of_those_available(&self, mount_point: &CgroupMountPoint, desired_controllers: &Controllers) -> io::Result<Controllers>
	{
		let available_controllers = self.read_available_controllers(mount_point)?;
		let (enable, disable) = desired_controllers.to_enable_and_disable(&available_controllers);
		self.change_subtree_controllers(mount_point, &enable, &disable)?;
		Ok(available_controllers)
	}
	
	/// The set of controllers match the set of controllers in the parents' cgroup's `subtree_control()`.
	#[inline(always)]
	fn read_available_controllers(&self, mount_point: &CgroupMountPoint) -> io::Result<Controllers>
	{
		self.cgroup_controllers_file_path(mount_point).read_value()
	}

	/// Reads maximum depth.
	///
	/// Since Linux version 4.14.
	#[inline(always)]
	fn read_maximum_depth(&self, mount_point: &CgroupMountPoint) -> io::Result<MaximumNumber<usize>>
	{
		self.cgroup_max_depth_file_path(mount_point).read_value()
	}
	
	/// Writes maximum depth.
	///
	/// Since Linux version 4.14.
	#[inline(always)]
	fn write_maximum_depth(&self, mount_point: &CgroupMountPoint, maximum_number: MaximumNumber<usize>) -> io::Result<()>
	{
		self.cgroup_max_depth_file_path(mount_point).write_value(maximum_number)
	}

	/// Reads maximum descendants, ie the number of cgroup entries in a directory.
	///
	/// Since Linux version 4.14.
	#[inline(always)]
	fn read_maximum_descendants(&self, mount_point: &CgroupMountPoint) -> io::Result<MaximumNumber<usize>>
	{
		self.cgroup_max_descendants_file_path(mount_point).read_value()
	}
	
	/// Writes maximum descendants.
	///
	/// Since Linux version 4.14.
	#[inline(always)]
	fn write_maximum_descendants(&self, mount_point: &CgroupMountPoint, maximum_number: MaximumNumber<usize>) -> io::Result<()>
	{
		self.cgroup_max_descendants_file_path(mount_point).write_value(maximum_number)
	}

	/// List of process identifiers.
	#[inline(always)]
	fn get_process_identifiers(&self, mount_point: &CgroupMountPoint) -> io::Result<Vec<ProcessIdentifier>>
	{
		read_process_or_thread_identifiers(self.cgroup_procs_file_path(mount_point))
	}

	/// Migrate process to this cgroup.
	#[inline(always)]
	fn migrate_process_to_this_cgroup(&self, mount_point: &CgroupMountPoint, process_identifier: ProcessIdentifierChoice) -> io::Result<()>
	{
		self.cgroup_procs_file_path(mount_point).write_value(process_identifier)
	}

	/// Statistics.
	///
	/// Since Linux version 4.14.
	#[inline(always)]
	fn read_statistics(&self, mount_point: &CgroupMountPoint) -> Result<Statistics, StatisticsParseError>
	{
		let path = self.cgroup_stat_file_path(mount_point);
		Statistics::from_file(&path)
	}
	
	/// Always exists even if the `cpu` controller is not enabled.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	fn read_cpu_pressure_stall_information(&self, mount_point: &CgroupMountPoint) -> io::Result<CpuTimeStalled>
	{
		let path = self.cpu_pressure_file_path(mount_point);
		CpuTimeStalled::from_file(&path)
	}
	
	/// Always exists even if the `cpu` controller is not enabled.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	fn monitor_some_cpu_pressure_stall_information(&self, mount_point: &CgroupMountPoint, maximum_total_stall_time_in_window: U64Microseconds, window: U64Microseconds) -> io::Result<File>
	{
		let path = self.cpu_pressure_file_path(mount_point);
		CpuTimeStalled::monitor_some(&path, maximum_total_stall_time_in_window, window)
	}
	
	/// Statistics always exist even if the `cpu` controller is not enabled (although they contain fewer entries).
	#[inline(always)]
	fn read_cpu_statistics(&self, mount_point: &CgroupMountPoint) -> Result<CpuStatistics, StatisticsParseError>
	{
		let path = self.cpu_stat_file_path(mount_point);
		CpuStatistics::from_file(&path)
	}
	
	/// Always exists even if the `io` controller is not enabled.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	fn read_input_output_pressure_stall_information(&self, mount_point: &CgroupMountPoint) -> io::Result<MemoryOrInputOutputTimeStalled>
	{
		let path = self.io_pressure_file_path(mount_point);
		MemoryOrInputOutputTimeStalled::from_file(&path)
	}
	
	/// Always exists even if the `io` controller is not enabled.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	fn monitor_some_input_output_pressure_stall_information(&self, mount_point: &CgroupMountPoint, maximum_total_stall_time_in_window: U64Microseconds, window: U64Microseconds) -> io::Result<File>
	{
		let path = self.input_output_pressure_file_path(mount_point);
		MemoryOrInputOutputTimeStalled::monitor_some(&path, maximum_total_stall_time_in_window, window)
	}
	
	/// Always exists even if the `io` controller is not enabled.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	fn monitor_all_input_output_pressure_stall_information(&self, mount_point: &CgroupMountPoint, maximum_total_stall_time_in_window: U64Microseconds, window: U64Microseconds) -> io::Result<File>
	{
		let path = self.input_output_pressure_file_path(mount_point);
		MemoryOrInputOutputTimeStalled::monitor_all(&path, maximum_total_stall_time_in_window, window)
	}
	
	/// Always exists even if the `memory` controller is not enabled.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	fn read_memory_pressure_stall_information(&self, mount_point: &CgroupMountPoint) -> io::Result<MemoryOrInputOutputTimeStalled>
	{
		let path = self.memory_pressure_file_path(mount_point);
		MemoryOrInputOutputTimeStalled::from_file(&path)
	}
	
	/// Always exists even if the `memory` controller is not enabled.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	fn monitor_some_memory_pressure_stall_informatmemoryn(&self, mount_point: &CgroupMountPoint, maximum_total_stall_time_in_window: U64Microseconds, window: U64Microseconds) -> io::Result<File>
	{
		let path = self.memory_pressure_file_path(mount_point);
		MemoryOrInputOutputTimeStalled::monitor_some(&path, maximum_total_stall_time_in_window, window)
	}
	
	/// Always exists even if the `memory` controller is not enabled.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	fn monitor_all_memory_pressure_stall_informatmemoryn(&self, mount_point: &CgroupMountPoint, maximum_total_stall_time_in_window: U64Microseconds, window: U64Microseconds) -> io::Result<File>
	{
		let path = self.memory_pressure_file_path(mount_point);
		MemoryOrInputOutputTimeStalled::monitor_all(&path, maximum_total_stall_time_in_window, window)
	}

	/// Reads enabled controllers.
	#[inline(always)]
	fn read_subtree_controllers(&self, mount_point: &CgroupMountPoint) -> io::Result<Controllers>
	{
		self.cgroup_subtree_control_file_path(mount_point).read_value()
	}

	/// Changes the enabled controllers.
	///
	/// *Panics* in debug mode if `enable` and `disable` sets intersect.
	///
	/// Only domain controller cgroups are allowed to have enabled controllers.
	#[inline(always)]
	fn change_subtree_controllers(&self, mount_point: &CgroupMountPoint, enable: &Controllers, disable: &Controllers) -> io::Result<()>
	{
		debug_assert_eq!(enable.intersection(disable).count(), 0, "There are controllers in both the `enable` and `disable` sets");

		let line = Controllers::create_change_line(enable, disable);
		self.cgroup_subtree_control_file_path(mount_point).write_value(line)
	}

	/// List of thread identifiers.
	///
	/// Succeeds even if the cgroup's type is `Domain` or `InvalidDomain`.
	#[inline(always)]
	fn get_thread_identifiers(&self, mount_point: &CgroupMountPoint) -> io::Result<Vec<ThreadIdentifier>>
	{
		read_process_or_thread_identifiers(self.cgroup_threads_file_path(mount_point))
	}

	/// Migrate thread to this cgroup.
	///
	/// Succeeds only if:-
	///
	/// * This is a `RootCgroup`;
	/// * This is a `NonRootCgroup` which is a leaf and which has a `type` of `Threaded`.
	#[inline(always)]
	fn migrate_thread_to_this_cgroup(&self, mount_point: &CgroupMountPoint, thread_identifier: ThreadIdentifierChoice) -> io::Result<()>
	{
		self.cgroup_threads_file_path(mount_point).write_value(thread_identifier)
	}
	
	/// Effective HyperThreads.
	///
	/// Only works if the `cpuset` controller is enabled.
	///
	/// Can be empty.
	#[inline(always)]
	fn cpuset_hyper_threads_effective(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<HyperThreads>>
	{
		self.cpuset_cpus_effective_file_path(mount_point).read_hyper_thread_or_numa_node_list_if_exists().map(|option| option.map(HyperThreads))
	}
	
	/// Effective NUMA nodes.
	///
	/// Only works if the `cpuset` controller is enabled.
	///
	/// Can be empty.
	#[inline(always)]
	fn cpuset_numa_nodes_effective(&self, mount_point: &CgroupMountPoint) -> io::Result<Option<NumaNodes>>
	{
		self.cpuset_mems_effective_file_path(mount_point).read_hyper_thread_or_numa_node_list_if_exists().map(|option| option.map(NumaNodes))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cgroup_controllers_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.controllers")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cgroup_max_depth_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.max.depth")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cgroup_max_descendants_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.max.descendants")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cgroup_procs_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.procs")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cgroup_subtree_control_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.subtree_control")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cgroup_stat_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.stat")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cgroup_threads_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cgroup.threads")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cpu_pressure_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cpu.pressure")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cpu_stat_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cpu.stat")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cpuset_cpus_effective_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cpuset.cpus.effective")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn cpuset_mems_effective_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "cpuset.mems.effective")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn io_pressure_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "io.pressure")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn memory_pressure_file_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.file_path(mount_point, "memory.pressure")
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn file_path(&self, mount_point: &CgroupMountPoint, file_name: impl AsRef<Path>) -> PathBuf
	{
		self.to_owned_path(mount_point).append(file_name)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn to_owned_path(&self, mount_point: &CgroupMountPoint) -> PathBuf
	{
		self.to_path(mount_point).into_owned()
	}
}
