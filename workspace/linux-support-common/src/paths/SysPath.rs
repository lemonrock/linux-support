// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents `/sys`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct SysPath(PathBuf);

impl Default for SysPath
{
	#[inline(always)]
	fn default() -> Self
	{
		SysPath(PathBuf::from("/sys"))
	}
}

impl SysPath
{
	#[inline(always)]
	pub(crate) fn pci_bus_file_path(&self, file_name: &str) -> PathBuf
	{
		self.pci_bus_folder_path().append(file_name)
	}

	/// A path about all hyper threads.
	#[inline(always)]
	pub(crate) fn hyper_threads_folder_path(&self, file_name: &str) -> PathBuf
	{
		self.cpu_system_devices_folder_path().append(file_name)
	}

	/// A hyper thread file.
	#[inline(always)]
	pub(crate) fn hyper_thread_path(&self, hyper_thread: HyperThread, file_name: &str) -> PathBuf
	{
		self.hyper_thread_folder_path(hyper_thread).append(file_name)
	}

	/// Hyper thread folder path.
	///
	/// Look at `HyperThread` for functions that use this.
	#[inline(always)]
	pub(crate) fn hyper_thread_folder_path(&self, hyper_thread: HyperThread) -> PathBuf
	{
		let into: u16 = hyper_thread.into();
		self.hyper_threads_folder_path(&format!("cpu{}", into))
	}

	/// Workqueue file path.
	#[inline(always)]
	pub(crate) fn hyper_thread_workqueue_file_path(&self, file_name: &str) -> PathBuf
	{
		self.workqueue_virtual_devices_folder_path().append(file_name)
	}

	#[inline(always)]
	pub(crate) fn numa_hugepages_file_path(&self, huge_page_size: HugePageSize, numa_node: NumaNode, file_name: &str) -> PathBuf
	{
		Self::hugepages_file_path(self.numa_node_folder_path(numa_node), huge_page_size, file_name)
	}

	/// A NUMA node file.
	#[inline(always)]
	pub(crate) fn numa_node_path(&self, numa_node: NumaNode, file_name: &str) -> PathBuf
	{
		self.numa_node_folder_path(numa_node).append(file_name)
	}

	/// NUMA node folder path.
	///
	/// Look at `NumaNode` for functions that use this.
	#[inline(always)]
	pub(crate) fn numa_node_folder_path(&self, numa_node: NumaNode) -> PathBuf
	{
		let into: u8 = numa_node.into();
		self.numa_nodes_path(&format!("node{}", into))
	}

	/// A path about all NUMA nodes.
	#[inline(always)]
	pub(crate) fn numa_nodes_path(&self, file_name: &str) -> PathBuf
	{
		self.numa_nodes_folder_path().append(file_name)
	}

	#[inline(always)]
	pub(crate) fn numa_nodes_folder_path(&self) -> PathBuf
	{
		self.system_devices_folder_path().append("node")
	}

	#[inline(always)]
	pub(crate) fn khugepaged_file_path(&self, file_name: &str) -> PathBuf
	{
		self.global_transparent_huge_memory_file_path("khugepaged").append(file_name)
	}

	#[inline(always)]
	pub(crate) fn global_transparent_huge_memory_file_path(&self, file_name: &str) -> PathBuf
	{
		self.global_memory_folder_path().append("transparent_hugepage").append(file_name)
	}

	#[inline(always)]
	pub(crate) fn read_global_hugepages_value(&self, huge_page_size: HugePageSize, file_name: &str) -> io::Result<u64>
	{
		self.global_hugepages_file_path(huge_page_size, file_name).read_value()
	}

	#[inline(always)]
	pub(crate) fn global_hugepages_file_path(&self, huge_page_size: HugePageSize, file_name: &str) -> PathBuf
	{
		Self::hugepages_file_path(self.global_memory_folder_path(), huge_page_size, file_name)
	}

	#[inline(always)]
	fn hugepages_file_path(folder_path: PathBuf, huge_page_size: HugePageSize, file_name: &str) -> PathBuf
	{
		folder_path.append("hugepages").append(format!("hugepages-{}kB", huge_page_size.size_in_kilo_bytes())).append(file_name)
	}

	#[inline(always)]
	fn cpu_system_devices_folder_path(&self) -> PathBuf
	{
		self.system_devices_folder_path().append("cpu")
	}

	#[inline(always)]
	fn global_memory_folder_path(&self) -> PathBuf
	{
		self.kernel_folder_path().append("mm")
	}

	#[inline(always)]
	fn pci_bus_folder_path(&self) -> PathBuf
	{
		self.bus_folder_path().append("pci")
	}

	#[inline(always)]
	fn workqueue_virtual_devices_folder_path(&self) -> PathBuf
	{
		self.virtual_devices_folder_path().append("workqueue")
	}

	#[inline(always)]
	fn virtual_devices_folder_path(&self) -> PathBuf
	{
		self.devices_folder_path().append("virtual")
	}

	#[inline(always)]
	fn system_devices_folder_path(&self) -> PathBuf
	{
		self.devices_folder_path().append("system")
	}

	#[inline(always)]
	fn devices_folder_path(&self) -> PathBuf
	{
		self.path().append("devices")
	}

	#[inline(always)]
	fn bus_folder_path(&self) -> PathBuf
	{
		self.path().append("bus")
	}

	#[inline(always)]
	fn kernel_folder_path(&self) -> PathBuf
	{
		self.path().append("kernel")
	}

	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
