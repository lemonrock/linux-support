// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// PCI bus.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PciBus
{
	#[allow(missing_docs)]
	pci_bus_address: PciBusAddress,
	
	#[allow(missing_docs)]
	canonical_parent_folder_path_of_pci_bus_folder_path: PathBuf,
}

impl Into<PciBusAddress> for PciBus
{
	#[inline(always)]
	fn into(self) -> PciBusAddress
	{
		self.pci_bus_address
	}
}

impl Deref for PciBus
{
	type Target = PciBusAddress;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.pci_bus_address
	}
}

impl PciBus
{
	/// Details.
	#[inline(always)]
	pub fn details(&self) -> io::Result<PciBusDetails>
	{
		Ok
		(
			PciBusDetails
			{
				associated_hyper_threads_bit_set: self.associated_hyper_threads_bit_set()?,
				associated_hyper_threads_bitmask: self.associated_hyper_threads_bitmask()?
			}
		)
	}
	
	/// Address.
	#[inline(always)]
	pub fn address(&self) -> PciBusAddress
	{
		self.pci_bus_address
	}
	
	/// Rescan PCI bus.
	///
	/// Root only.
	#[inline(always)]
	pub fn rescan(&self) -> io::Result<()>
	{
		assert_effective_user_id_is_root("Rescan of PCI bus");
		
		self.file_path("bus_rescan").write_value(true)
	}
	
	/// PCI bus associated hyper threads.
	///
	/// May report CPUs that don't actually exist; refine list against that known for a NUMA node.
	#[inline(always)]
	fn associated_hyper_threads_bit_set(&self) -> io::Result<HyperThreads>
	{
		let file_path = self.file_path("cpuaffinity");
		file_path.read_hyper_thread_or_numa_node_list().map(HyperThreads)
	}

	/// PCI bus associated hyper threads.
	///
	/// May report CPUs that don't actually exist; refine list against that known for a NUMA node.
	#[inline(always)]
	fn associated_hyper_threads_bitmask(&self) -> io::Result<HyperThreads>
	{
		let file_path = self.file_path("cpulistaffinity");
		file_path.parse_comma_separated_bit_set().map(HyperThreads)
	}
	
	#[inline(always)]
	fn file_path(&self, file_name: &str) -> PathBuf
	{
		let string_address: String = self.pci_bus_address.into();
		self.canonical_parent_folder_path_of_pci_bus_folder_path.clone().append("pci_bus").append(&string_address).append(file_name)
	}
}
