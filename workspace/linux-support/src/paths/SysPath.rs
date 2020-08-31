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
	/// `/sys/devices/system/cpu/cpu<hyper_thread>/cache/<file_name>`.
	#[inline(always)]
	pub(crate) fn hyper_thread_cache_file_path(&self, hyper_thread: HyperThread, file_name: &str) -> PathBuf
	{
		self.hyper_thread_file_path(hyper_thread, "cache").append(file_name)
	}

	/// `/sys/devices/system/cpu/cpu<hyper_thread>/topology/<file_name>`.
	#[inline(always)]
	pub(crate) fn hyper_thread_topology_file_path(&self, hyper_thread: HyperThread, file_name: &str) -> PathBuf
	{
		self.hyper_thread_file_path(hyper_thread, "topology").append(file_name)
	}

	/// `/sys/devices/system/cpu/cpu<hyper_thread>/<file_name>`.
	#[inline(always)]
	pub(crate) fn hyper_thread_file_path(&self, hyper_thread: HyperThread, file_name: &str) -> PathBuf
	{
		self.hyper_thread_folder_path(hyper_thread).append(file_name)
	}

	/// `/sys/devices/system/cpu/cpu<hyper_thread>`.
	#[inline(always)]
	pub(crate) fn hyper_thread_folder_path(&self, hyper_thread: HyperThread) -> PathBuf
	{
		let into: u16 = hyper_thread.into();
		self.hyper_threads_folder_path(&format!("cpu{}", into))
	}

	/// `/sys/devices/system/cpu/smt/<file_name>`.
	#[inline(always)]
	pub(crate) fn hyper_thread_smt_file_path(&self, file_name: &str) -> PathBuf
	{
		self.hyper_threads_folder_path("smt").append(file_name)
	}

	/// `/sys/devices/system/cpu/<file_name>`.
	#[inline(always)]
	pub(crate) fn hyper_threads_folder_path(&self, file_name: &str) -> PathBuf
	{
		self.cpu_system_devices_folder_path().append(file_name)
	}

	/// `/sys/devices/virtual/workqueue/<file_name>`.
	#[inline(always)]
	pub(crate) fn hyper_thread_work_queue_file_path(&self, file_name: &str) -> PathBuf
	{
		self.workqueue_virtual_devices_folder_path().append(file_name)
	}

	/// `/sys/devices/system/node/node<numa_node>/hugepages/hugepages-<huge_page_size>kB`
	#[inline(always)]
	pub(crate) fn numa_node_hugepages_folder_path(&self, huge_page_size: HugePageSize, numa_node: NumaNode) -> PathBuf
	{
		Self::hugepages_folder_path(self.numa_node_folder_path(numa_node), huge_page_size)
	}

	/// `/sys/devices/system/node/node<numa_node>/<file_name>`
	#[inline(always)]
	pub(crate) fn numa_node_file_path(&self, numa_node: NumaNode, file_name: &str) -> PathBuf
	{
		self.numa_node_folder_path(numa_node).append(file_name)
	}

	/// `/sys/devices/system/node/node<numa_node>`
	#[inline(always)]
	pub(crate) fn numa_node_folder_path(&self, numa_node: NumaNode) -> PathBuf
	{
		let into: u16 = numa_node.into();
		self.numa_nodes_path(&format!("node{}", into))
	}

	/// `/sys/devices/system/node/<file_name>`
	#[inline(always)]
	pub(crate) fn numa_nodes_path(&self, file_name: &str) -> PathBuf
	{
		self.numa_nodes_folder_path().append(file_name)
	}

	/// `/sys/kernel/mm/ksm/<file_name>`
	#[inline(always)]
	pub(crate) fn ksm_file_path(&self, file_name: &str) -> PathBuf
	{
		self.transparent_huge_memory_file_path("ksm").append(file_name)
	}
	
	/// `/sys/kernel/mm/khugepaged/<file_name>`
	#[inline(always)]
	pub(crate) fn khugepaged_file_path(&self, file_name: &str) -> PathBuf
	{
		self.transparent_huge_memory_file_path("khugepaged").append(file_name)
	}

	/// `/sys/kernel/mm/transparent_hugepage/<file_name>`
	#[inline(always)]
	pub(crate) fn transparent_huge_memory_file_path(&self, file_name: &str) -> PathBuf
	{
		self.global_memory_folder_path().append("transparent_hugepage").append(file_name)
	}

	/// `/sys/kernel/mm/hugepages/hugepages-<huge_page_size>kB`
	#[inline(always)]
	pub(crate) fn global_hugepages_folder_path(&self, huge_page_size: HugePageSize) -> PathBuf
	{
		Self::hugepages_folder_path(self.global_memory_folder_path(), huge_page_size)
	}

	/// `/sys/devices/system/node`.
	#[inline(always)]
	pub(crate) fn numa_nodes_folder_path(&self) -> PathBuf
	{
		self.system_devices_folder_path().append("node")
	}

	/// `/sys/devices/system/cpu`.
	#[inline(always)]
	pub(crate) fn cpu_system_devices_folder_path(&self) -> PathBuf
	{
		self.system_devices_folder_path().append("cpu")
	}

	/// `/sys/devices/system`.
	#[inline(always)]
	fn system_devices_folder_path(&self) -> PathBuf
	{
		self.devices_folder_path().append("system")
	}

	/// `/sys/kernel/mm`.
	#[inline(always)]
	fn global_memory_folder_path(&self) -> PathBuf
	{
		self.kernel_folder_path().append("mm")
	}
	
	/// `/sys/kernel/security`.
	#[inline(always)]
	pub(crate) fn kernel_security_file_path(&self, file_name: &str) -> PathBuf
	{
		self.kernel_security_folder_path().append(file_name)
	}
	
	/// `/sys/kernel/security`.
	#[inline(always)]
	fn kernel_security_folder_path(&self) -> PathBuf
	{
		self.kernel_folder_path().append("security")
	}

	/// `/sys/kernel/irq/<interrupt_request>/<file_name>`.
	#[inline(always)]
	pub(crate) fn global_irq_file_path(&self, interrupt_request: InterruptRequest, file_name: &str) -> PathBuf
	{
		self.global_irq_folder_path(interrupt_request).append(file_name)
	}
	
	/// `/sys/kernel/irq/<interrupt_request>`.
	#[inline(always)]
	pub(crate) fn global_irq_folder_path(&self, interrupt_request: InterruptRequest) -> PathBuf
	{
		self.kernel_irq_folder_path().append(interrupt_request.file_name())
	}
	
	/// `/sys/kernel/irq`.
	#[inline(always)]
	pub(crate) fn kernel_irq_folder_path(&self) -> PathBuf
	{
		self.kernel_folder_path().append("irq")
	}

	/// `/sys/kernel/<file_name>`.
	#[inline(always)]
	pub(crate) fn kernel_file_path(&self, file_name: &str) -> PathBuf
	{
		self.kernel_folder_path().append(file_name)
	}

	/// `/sys/kernel`.
	#[inline(always)]
	fn kernel_folder_path(&self) -> PathBuf
	{
		self.path().append("kernel")
	}

	/// `/sys/devices/virtual/workqueue`.
	#[inline(always)]
	fn workqueue_virtual_devices_folder_path(&self) -> PathBuf
	{
		self.virtual_devices_folder_path().append("workqueue")
	}

	/// `/sys/devices/virtual`.
	#[inline(always)]
	fn virtual_devices_folder_path(&self) -> PathBuf
	{
		self.devices_folder_path().append("virtual")
	}

	/// `/sys/fs/<file_system>`.
	#[inline(always)]
	pub fn file_system_fs_folder_path(&self, file_system: FileSystemType) -> PathBuf
	{
		self.fs_folder_path().append(&file_system)
	}

	/// `/sys/fs`.
	#[inline(always)]
	fn fs_folder_path(&self) -> PathBuf
	{
		self.path().append("fs")
	}
	
	/// `/sys/devices`.
	#[inline(always)]
	fn devices_folder_path(&self) -> PathBuf
	{
		self.path().append("devices")
	}
	
	/// `/sys/class/net/<network_interface_name>`.
	#[inline(always)]
	pub fn network_interface_class_net_folder_path(&self, network_interface_name: &NetworkInterfaceName) -> PathBuf
	{
		self.class_net_folder_path().append(network_interface_name)
	}
	
	/// `/sys/class/net`.
	#[inline(always)]
	fn class_net_folder_path(&self) -> PathBuf
	{
		self.class_folder_path().append("net")
	}
	
	/// `/sys/class`.
	#[inline(always)]
	fn class_folder_path(&self) -> PathBuf
	{
		self.path().append("class")
	}

	/// `/sys/bus/pci/drivers/<driver_name>`.
	#[inline(always)]
	pub(crate) fn pci_driver_folder_path(&self, driver_name: impl AsRef<Path>) -> PathBuf
	{
		self.drivers_pci_bus_folder_path().append(driver_name)
	}

	/// `/sys/bus/pci/drivers`.
	#[inline(always)]
	fn drivers_pci_bus_folder_path(&self) -> PathBuf
	{
		self.pci_bus_file_path("drivers")
	}

	/// `/sys/bus/pci/devices/<pci_device_address>`.
	#[inline(always)]
	pub(crate) fn pci_device_folder_path(&self, pci_device_address: PciDeviceAddress) -> PathBuf
	{
		let string_address: String = pci_device_address.into();
		self.devices_pci_bus_folder_path().append(&string_address)
	}

	/// `/sys/bus/pci/devices`.
	#[inline(always)]
	pub(crate) fn devices_pci_bus_folder_path(&self) -> PathBuf
	{
		self.pci_bus_file_path("devices")
	}

	/// `/sys/bus/pci/<file_name>`.
	#[inline(always)]
	pub(crate) fn pci_bus_file_path(&self, file_name: &str) -> PathBuf
	{
		self.pci_bus_folder_path().append(file_name)
	}

	/// `/sys/bus/pci`.
	#[inline(always)]
	fn pci_bus_folder_path(&self) -> PathBuf
	{
		self.bus_folder_path().append("pci")
	}

	/// `/sys/bus`.
	#[inline(always)]
	fn bus_folder_path(&self) -> PathBuf
	{
		self.path().append("bus")
	}

	/// `/sys/module/<file_name>`.
	#[inline(always)]
	pub(crate) fn module_file_or_folder_path(&self, file_name: impl AsRef<Path>) -> PathBuf
	{
		self.path().append("module").append(file_name)
	}

	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}

	/// `hugepages/hugepages-<huge_page_size>kB`
	#[inline(always)]
	fn hugepages_folder_path(folder_path: PathBuf, huge_page_size: HugePageSize) -> PathBuf
	{
		folder_path.append("hugepages").append(format!("hugepages-{}kB", huge_page_size.size_in_kilobytes()))
	}
}
