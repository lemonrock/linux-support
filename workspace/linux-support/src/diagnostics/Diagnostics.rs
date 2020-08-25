// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Diagnostics.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Diagnostics
{
	/// File system layout used.
	pub file_system_layout: FileSystemLayout,
	
	/// Users and groups.
	pub users_and_groups: UsersAndGroupsDiagnostics,

	/// Thread.
	pub current_thread: CurrentThreadDiagnostic,

	/// Thread.
	pub threads: DiagnosticUnobtainableResult<HashMap<ThreadIdentifier, ThreadDiagnostic>>,

	/// Swap.
	pub swap: SwapDiagnostics,

	/// Scheduling.
	pub scheduling: SchedulingDiagnostics,

	/// Process.
	pub current_process_diagnostics: CurrentProcessDiagnostics,

	/// Process.
	pub process_diagnostics: ProcessDiagnostics,

	/// PCI devices.
	pub pci_devices: DiagnosticUnobtainableResult<HashMap<PciDeviceAddress, Option<PciDeviceDiagnostics>>>,

	/// Network devices.
	pub network_devices: DiagnosticUnobtainableResult<NetworkDeviceDiagnostics>,

	/// Internet protocol version 4 addresses.
	pub internet_protocol_version_4_addresses: DiagnosticUnobtainableResult<InternetProtocolAddressesDiagnostic<in_addr>>,

	/// Internet protocol version 6 addresses.
	pub internet_protocol_version_6_addresses: DiagnosticUnobtainableResult<InternetProtocolAddressesDiagnostic<in6_addr>>,
}

impl Diagnostics
{
	fn gather(file_system_layout: &FileSystemLayout) -> Self
	{
		let (sys_path, proc_path, dev_path, etc_path) = file_system_layout.paths();
		
		let process_group_identifier = ProcessGroupIdentifierChoice::Current;
		let process_identifier = ProcessIdentifierChoice::Current;
		Self
		{
			file_system_layout: file_system_layout.clone(),
			
			users_and_groups: UsersAndGroupsDiagnostics::gather(proc_path, etc_path, process_identifier),
			
			current_thread: CurrentThreadDiagnostic::gather(proc_path, process_identifier),
			
			threads: match ThreadIdentifier::for_process(proc_path, process_identifier)
			{
				Err(error) => Err(DiagnosticUnobtainable::from(error)),
				Ok(thread_identifiers) =>
				{
					let mut thread_diagnostics = HashMap::new();
					for thread_identifier in thread_identifiers
					{
						thread_diagnostics.insert(thread_identifier, ThreadDiagnostic::gather(proc_path, process_identifier, thread_identifier));
					}
					Ok(thread_diagnostics)
				}
			},
			
			swap: SwapDiagnostics::gather(proc_path),
			
			scheduling: SchedulingDiagnostics::gather(sys_path, proc_path, process_group_identifier, process_identifier),
			
			current_process_diagnostics: CurrentProcessDiagnostics::gather(),
			
			process_diagnostics: ProcessDiagnostics::gather(proc_path, process_group_identifier, process_identifier),
			
			pci_devices: match PciDeviceAddress::all(sys_path)
			{
				Err(error) => Err(DiagnosticUnobtainable::from(error)),
				Ok(pci_device_addresses) =>
				{
					let mut pci_devices = HashMap::new();
					for pci_device_address in pci_device_addresses
					{
						pci_devices.insert(pci_device_address, PciDeviceDiagnostics::gather(sys_path, pci_device_address));
					}
					Ok(pci_devices)
				}
			},
			
			network_devices: NetworkDeviceDiagnostics::gather(),
			
			internet_protocol_version_4_addresses: InternetProtocolAddressesDiagnostic::<in_addr>::gather(),
			
			internet_protocol_version_6_addresses: InternetProtocolAddressesDiagnostic::<in6_addr>::gather(),
		}
	}
}
