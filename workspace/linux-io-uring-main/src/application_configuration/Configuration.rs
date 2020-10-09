// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Deserialize, Serialize)]
pub struct Configuration
{
	#[serde(default)] pub file_system_layout: FileSystemLayout,
	
	#[serde(default)] pub process_executor: ProcessExecutor,
	
	pub io_uring_settings: IoUringSettings,
	
	#[serde(default)] pub device_preferences: DevicePreferences,
	
	#[serde(default)] pub process_configuration: ProcessConfiguration,
	
	#[serde(default)] pub main_thread_configuration: ThreadConfiguration,
	
	#[serde(default)] pub accept_child_thread_configuration: ThreadConfiguration,

	#[serde(default)] pub accept_services: HashMap<ServiceProtocolIdentifier, AcceptServiceConfiguration>,
}

impl Default for Configuration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::defaultish(443)
	}
}

impl Configuration
{
	/// From JSON file.
	pub fn from_json_file(configuration_file_path: impl AsRef<Path>) -> Result<Self, Box<dyn error::Error>>
	{
    	let reader = BufReader::new(File::open(configuration_file_path)?);
    	let configuration = from_reader(reader)?;
		Ok(configuration)
	}
	
	/// Default-ish
	pub fn defaultish(port_number: u16) -> Self
	{
		Self
		{
			file_system_layout: FileSystemLayout::default(),
			
			process_executor: ProcessExecutor::default(),
			
			io_uring_settings: IoUringSettings::defaultish
			(
				RegisteredBufferSettings
				{
					_4Kb: RegisteredBufferSetting::maximum_registered_buffer_size(OneMegabyte),
					_16Kb: RegisteredBufferSetting::maximum_registered_buffer_size(OneMegabyte),
					_64Kb: RegisteredBufferSetting::maximum_registered_buffer_size(OneMegabyte),
					_256Kb: RegisteredBufferSetting::maximum_registered_buffer_size(OneMegabyte),
					_1Mb: RegisteredBufferSetting::maximum_registered_buffer_size(OneMegabyte),
					_4Mb: RegisteredBufferSetting::none(),
					_16Mb: RegisteredBufferSetting::none(),
					_64Mb: RegisteredBufferSetting::none(),
					_256Mb: RegisteredBufferSetting::none(),
					_1Gb: RegisteredBufferSetting::none(),
				}
			),
			
			device_preferences: DevicePreferences::default(),
			
			process_configuration: ProcessConfiguration::default(),
			
			main_thread_configuration: ThreadConfiguration::default(),
			
			accept_child_thread_configuration: ThreadConfiguration::default(),
			
			accept_services: hashmap!
			{
				SipOverTls => AcceptServiceConfiguration
				{
					internet_protocol_version_4: vec!
					[
						TransmissionControlProtocolServerListenerSettings::defaultish(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port_number)),
					],
					
					permitted_internet_protocol_version_4_subnets: btreemap!
					[
						InternetProtocolAddressWithMask::<in_addr>::local_host() => Arc::new(AccessControlValue),
					],
					
					internet_protocol_version_6: vec!
					[
						TransmissionControlProtocolServerListenerSettings::defaultish(SocketAddrV6::new(Ipv6Addr::LOCALHOST, port_number, 0, 0)),
					],
					
					permitted_internet_protocol_version_6_subnets: btreemap!
					{
						InternetProtocolAddressWithMask::<in6_addr>::local_host() => Arc::new(AccessControlValue),
					},
					
					unix_domain_socket: vec!
					[
						TransmissionControlProtocolServerListenerSettings::defaultish(UnixSocketAddress::from_abstract_name(port_number.to_string().as_bytes()).unwrap())
					],
					
					permitted_unix_domain_group_identifiers: hashmap!
					[
						GroupIdentifier::default() => Arc::new(AccessControlValue),
					],
					
					permitted_unix_domain_user_identifiers: hashmap!
					[
						UserIdentifier::default() => Arc::new(AccessControlValue),
					],
				}
			}
		}
	}
	
	/// Server listeners.
	pub fn server_listeners(&mut self) ->
	(
		Vec<AcceptConnectionsCoroutineSettings<SocketAddrV4, InternetProtocolVersion4AccessControl<AccessControlValue>>>,
		Vec<AcceptConnectionsCoroutineSettings<SocketAddrV6, InternetProtocolVersion6AccessControl<AccessControlValue>>>,
		Vec<AcceptConnectionsCoroutineSettings<UnixSocketAddress<PathBuf>, UnixDomainSocketAccessControl<AccessControlValue>>>,
	)
	{
		let mut transmission_control_protocol_over_internet_protocol_version_4_server_listeners = Vec::new();
		let mut transmission_control_protocol_over_internet_protocol_version_6_server_listeners = Vec::new();
		let mut streaming_unix_domain_socket_server_listener_server_listeners = Vec::new();
		
		for (service_protocol_identifier, accept_service_configuration) in self.accept_services.iter_mut()
		{
			accept_service_configuration.server_listeners
			(
				*service_protocol_identifier,
				&mut transmission_control_protocol_over_internet_protocol_version_4_server_listeners,
				&mut transmission_control_protocol_over_internet_protocol_version_6_server_listeners,
				&mut streaming_unix_domain_socket_server_listener_server_listeners)
		}
		
		(
			transmission_control_protocol_over_internet_protocol_version_4_server_listeners,
			transmission_control_protocol_over_internet_protocol_version_6_server_listeners,
			streaming_unix_domain_socket_server_listener_server_listeners,
		)
	}
	
	/// Configure.
	#[inline(always)]
	pub fn configure(&self, run_as_daemon: bool, global_computed_scheduling_affinity: Option<&GlobalComputedSchedulingConfiguration>, process_affinity: Option<&HyperThreads>) -> (Arc<impl Terminate>, DefaultPageSizeAndHugePageSizes)
	{
		let defaults = self.file_system_layout.defaults().unwrap();
		let terminate = self.process_configuration.configure(run_as_daemon, &self.file_system_layout, &defaults, &mut DogStatsDStaticInitialization, global_computed_scheduling_affinity, process_affinity).expect("Could not configure process");
		(terminate, defaults)
	}
	
	/// Configure PCI ethernet devices.
	#[inline(always)]
	pub fn configure_pci_ethernet_devices(&self) -> HashMap<NetworkInterfaceName, (HyperThread, HyperThreads)>
	{
		let sys_path = &self.file_system_layout.sys_path;
		let proc_path = &self.file_system_layout.proc_path;
		DriverProfile::configure_all_multiqueue_pci_ethernet_devices(sys_path, proc_path, &self.device_preferences).expect("Could not configure PCI ethernet devices")
	}
	
	/// Execute.
	#[inline(always)]
	pub fn execute<T: Terminate + 'static, MainThreadFunction: ThreadFunction, ChildThreadFunction: ThreadFunction>(self, terminate: Arc<T>, main_thread: ThreadSettings<MainThreadFunction>, child_threads: Vec<ThreadSettings<ChildThreadFunction>>, defaults: DefaultPageSizeAndHugePageSizes)
	{
		let instantiation_arguments = Arc::new((defaults, &SwitchableGlobalAllocator));
		self.process_executor.execute_securely::<T, MainThreadFunction, ChildThreadFunction, SimplePerThreadMemoryAllocatorInstantiator<CoroutineHeapSize, GTACSA>>(&self.file_system_layout, terminate, main_thread, child_threads, instantiation_arguments).expect("Could not execute process")
	}
}
