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
	/// Rescans all PCI buses and devices.
	#[inline(always)]
	pub fn rescan_all_pci_buses_and_devices(&self) -> io::Result<()>
	{
		let mut path = self.path();
		path.push("bus/pci/rescan");
		path.write_value(1)
	}

	/// A path about all hyper threads.
	#[inline(always)]
	pub(crate) fn hyper_threads_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.hyper_threads_parent_path();
		path.push(file_name);
		path
	}
	/// A hyper thread file.
	#[inline(always)]
	pub(crate) fn hyper_thread_path(&self, hyper_thread: HyperThread, file_name: &str) -> PathBuf
	{
		let mut path = self.hyper_thread_folder_path(hyper_thread);
		path.push(file_name);
		path
	}

	/// Hyper thread folder path.
	///
	/// Look at `HyperThread` for functions that use this.
	#[inline(always)]
	pub(crate) fn hyper_thread_folder_path(&self, hyper_thread: HyperThread) -> PathBuf
	{
		let into: u16 = hyper_thread.into();
		self.hyper_threads_path(&format!("cpu{}", into))
	}

	#[inline(always)]
	fn hyper_threads_parent_path(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("devices/system/cpu");
		path
	}

	/// Workqueue file path.
	#[inline(always)]
	pub(crate) fn hyper_thread_workqueue_file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.path();
		path.push("devices/virtual/workqueue");
		path.push(file_name);
		path
	}

	/// A PCI device file.
	#[inline(always)]
	pub(crate) fn pci_device_path(&self, pci_device: (u32, u8, u8, u8), file_name: &str) -> PathBuf
	{
		let mut path = self.pci_device_folder_path(pci_device);
		path.push(file_name);
		path
	}

	/// PCI device folder path.
	#[inline(always)]
	pub(crate) fn pci_device_folder_path(&self, pci_device: (u32, u8, u8, u8)) -> PathBuf
	{
		self.pci_devices_path(&format!("{:04x}:{:02x}:{:02x}.{:01x}", pci_device.0, pci_device.1, pci_device.2, pci_device.3))
	}

	/// A path about all PCI devices.
	#[inline(always)]
	pub(crate) fn pci_devices_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.pci_devices_parent_path();
		path.push(file_name);
		path
	}

	#[inline(always)]
	fn pci_devices_parent_path(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("bus/pci/devices");
		path
	}

	#[inline(always)]
	pub(crate) fn read_numa_hugepages_value(&self, huge_page_size: HugePageSize, numa_node: NumaNode, file_name: &str) -> io::Result<u64>
	{
		self.numa_hugepages_file_path(huge_page_size, numa_node, file_name).read_value()
	}

	#[inline(always)]
	pub(crate) fn numa_hugepages_file_path(&self, huge_page_size: HugePageSize, numa_node: NumaNode, file_name: &str) -> PathBuf
	{
		let mut file_path = self.numa_node_folder_path(numa_node);
		file_path.push(format!("hugepages/hugepages-{}kB", huge_page_size.size_in_kilo_bytes()));
		file_path.push(file_name);
		file_path
	}

	/// A NUMA node file.
	#[inline(always)]
	pub(crate) fn numa_node_path(&self, numa_node: NumaNode, file_name: &str) -> PathBuf
	{
		let mut path = self.numa_node_folder_path(numa_node);
		path.push(file_name);
		path
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
		let mut path = self.numa_nodes_parent_path();
		path.push(file_name);
		path
	}

	#[inline(always)]
	pub(crate) fn numa_nodes_parent_path(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("devices/system/node");
		path
	}

	#[inline(always)]
	pub(crate) fn khugepaged_file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.global_transparent_huge_memory_file_path("khugepaged");
		path.push(file_name);
		path
	}

	#[inline(always)]
	pub(crate) fn global_transparent_huge_memory_file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.global_memory_folder_path();
		path.push("transparent_hugepage");
		path.push(file_name);
		path
	}

	#[inline(always)]
	pub(crate) fn read_global_hugepages_value(&self, huge_page_size: HugePageSize, file_name: &str) -> io::Result<u64>
	{
		self.global_hugepages_file_path(huge_page_size, file_name).read_value()
	}

	#[inline(always)]
	pub(crate) fn global_hugepages_file_path(&self, huge_page_size: HugePageSize, file_name: &str) -> PathBuf
	{
		let mut file_path = self.global_memory_folder_path();
		file_path.push(format!("hugepages/hugepages-{}kB", huge_page_size.size_in_kilo_bytes()));
		file_path.push(file_name);
		file_path
	}

	#[inline(always)]
	fn global_memory_folder_path(&self) -> PathBuf
	{
		let mut path = self.path();
		path.push("kernel/mm");
		path
	}

	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
