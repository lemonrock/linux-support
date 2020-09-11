// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Diagnostics.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Diagnostics
{
	/// File system layout used.
	pub file_system_layout: FileSystemLayout,
	
	/// Page and Huge Page Sizes.
	pub defaults: DiagnosticUnobtainableResult<DefaultPageSizeAndHugePageSizes>,
	
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

	/// Environment.
	pub environment: EnvironmentDiagnostic,

	/// Linux kernel.
	pub linux_kernel: LinuxKernelDiagnostic,
	
	/// Interrupt requests.
	pub interrupt_requests: InterruptRequestDiagnostics,

	/// Inode.
	pub inode: DiagnosticUnobtainableResult<InodeDiagnostics>,
	
	/// eBPF.
	pub extended_berkeley_packet_filter: ExtendedBerkeleyPacketFilterDiagnostics,
	
	/// XDP.
	pub express_data_path_diagnostics: DiagnosticUnobtainableResult<Vec<GetExpressDataPathDiagnosticsMessageData>>,
	
	/// Cgroup2.
	pub cgroup_version2: Option<RootCgroupVersion2Diagnostics>,

	/// File Systems.
	pub file_systems: FileSystemsDiagnostics,

	/// File handles.
	pub file_handle: FileHandleDiagnostics,

	/// Memory.
	pub memory: MemoryDiagnostics,
	
	/// Hyper Threads (CPUs).
	pub hyper_threads: HyperThreadsDiagnostics,
	
	/// CPU information.
	#[cfg(target_arch = "x86_64")] pub cpu_information: CpuInformationDiagnostics,
	
	/// `/proc/sys`.
	pub proc_sys: BTreeMap<PathBuf, ProcSysFileDiagnostic>,
}

impl Diagnostics
{
	/// Gather diagnostics or panic.
	pub fn gather(file_system_layout: &FileSystemLayout) -> DiagnosticUnobtainableResult<Self>
	{
		catch_unwind(AssertUnwindSafe(||
		{
			let process_identifier = ProcessIdentifierChoice::Current;
		
			let (sys_path, proc_path, _dev_path, etc_path) = file_system_layout.paths();
			
			let defaults = file_system_layout.defaults().map_err(DiagnosticUnobtainable::from);
			
			let file_systems = FileSystemsDiagnostics::gather(proc_path, process_identifier);
			
			let supported_huge_page_sizes = match defaults
			{
				Err(_) => Cow::Owned(BTreeSet::new()),
				
				Ok(ref defaults) => Cow::Borrowed(defaults.supported_huge_page_sizes())
			};
			
			let process_diagnostics = ProcessDiagnostics::gather(sys_path, proc_path, process_identifier);
			
			let process_group_identifier = process_diagnostics.process_group_identifier.as_ref().map(|process_group_identifier| ProcessGroupIdentifierChoice::Other(*process_group_identifier)).unwrap_or(ProcessGroupIdentifierChoice::Current);
			Self
			{
				file_system_layout: file_system_layout.clone(),
				
				users_and_groups: UsersAndGroupsDiagnostics::gather(proc_path, etc_path, process_identifier),
				
				current_thread: CurrentThreadDiagnostic::gather(),
				
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
				
				process_diagnostics,
				
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
				
				network_devices: NetworkDeviceDiagnostics::gather(sys_path),
				
				internet_protocol_version_4_addresses: InternetProtocolAddressesDiagnostic::<in_addr>::gather(),
				
				internet_protocol_version_6_addresses: InternetProtocolAddressesDiagnostic::<in6_addr>::gather(),
			
				environment: EnvironmentDiagnostic::gather(),
			
				linux_kernel: LinuxKernelDiagnostic::gather(sys_path, proc_path),
			
				interrupt_requests: InterruptRequestDiagnostics::gather(sys_path, proc_path),
			
				inode: InodeDiagnostics::gather(proc_path),
			
				extended_berkeley_packet_filter: ExtendedBerkeleyPacketFilterDiagnostics::gather(proc_path, &file_systems),
				
				express_data_path_diagnostics: Self::express_data_path_diagnostics(),
				
				cgroup_version2: RootCgroupVersion2Diagnostics::gather(&file_systems, &supported_huge_page_sizes),
				
				file_handle: FileHandleDiagnostics::gather(proc_path),
			
				memory: MemoryDiagnostics::gather(sys_path, proc_path, &supported_huge_page_sizes),
			
				hyper_threads: HyperThreadsDiagnostics::gather(sys_path, proc_path),
				
				#[cfg(target_arch = "x86_64")] cpu_information: CpuInformationDiagnostics::gather(),
				
				proc_sys: ProcSysFileDiagnostic::gather(proc_path),
				
				defaults,
				
				file_systems,
			}
		})).map_err(|_| DiagnosticUnobtainable(format!("Panicked")))
	}
	
	#[inline(always)]
	fn express_data_path_diagnostics() -> DiagnosticUnobtainableResult<Vec<GetExpressDataPathDiagnosticsMessageData>>
	{
		let mut netlink_socket_file_descriptor = NetlinkSocketFileDescriptor::open().map_err(DiagnosticUnobtainable::from)?;
		RouteNetlinkProtocol::get_express_data_path_diagnostics(&mut netlink_socket_file_descriptor).map_err(DiagnosticUnobtainable)
	}
}
