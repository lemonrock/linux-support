// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use linux_support::configuration::ProcessConfiguration;
use linux_support::configuration::ProcessExecutor;
use linux_support::linux_kernel_version::{LinuxKernelVersionNumber, LinuxKernelHostName, LinuxKernelDomainName};
use linux_support::paths::FileSystemLayout;
use linux_support::thread::{ThreadConfiguration, ThreadFunction, ThreadSettings};
use std::sync::Arc;
use context_allocator::{GlobalThreadAndCoroutineSwitchableAllocatorInstance, LocalAllocator};
use context_allocator::PerThreadState;
use std::alloc::System;
use context_allocator::adaptors::AllocRefToAllocatorAdaptor;
use std::ptr::NonNull;
use context_allocator::memory_sources::{MemoryMapSource, CoroutineHeapMemorySource};
use context_allocator::allocators::ContextAllocator;
use context_allocator::allocators::binary_search_trees::MultipleBinarySearchTreeAllocator;
use linux_io_uring::{SimplePerThreadMemoryAllocatorInstantiator, ThreadLoopInitiation, IoUringSettings};
use linux_support::signals::Signals;
use message_dispatch::Queues;
use linux_io_uring::coroutines::accept::ServiceProtocolIdentifier;
use linux_io_uring::coroutines::accept::TransmissionControlProtocolServerListenerSettings;
use linux_io_uring::coroutines::accept::AcceptConnectionsCoroutineSettings;
use linux_io_uring::coroutines::accept::RemotePeerAddressBasedAccessControlValue;
use linux_support::file_descriptors::socket::c::sockaddr_in;
use linux_support::file_descriptors::socket::c::sockaddr_in6;
use std::path::PathBuf;
use socket_access_control::RemotePeerAddressBasedAccessControl;
use message_dispatch_datadog::dogstatsd::{DogStatsDTag, Label};
use linux_support::logging::AdditionalLoggingConfiguration;
use std::net::IpAddr;
use linux_support::process::ProcessName;
use std::error;
use std::marker::PhantomData;


type CoroutineHeapSize = MemorySize64Kb;

type CoroutineLocalAllocator = ContextAllocator<CoroutineHeapMemorySource<CoroutineHeapSize>>;

type ThreadLocalAllocator = MultipleBinarySearchTreeAllocator<MemoryMapSource>;

type GTACSA = GlobalThreadAndCoroutineSwitchableAllocatorInstance<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator, AllocRefToAllocatorAdaptor<System>>;

#[inline(always)]
fn per_thread_state() -> NonNull<PerThreadState<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator>>
{
	#[thread_local] static mut PerThreadState: PerThreadState<CoroutineHeapSize, CoroutineLocalAllocator, ThreadLocalAllocator> = PerThreadState::empty();
	unsafe { NonNull::new_unchecked(&mut PerThreadState) }
}

#[global_allocator] static SwitchableGlobalAllocator: GTACSA = GlobalThreadAndCoroutineSwitchableAllocatorInstance::system(per_thread_state);

pub fn main()
{
	// These are things that should be controlled by arguments passed to main or as a JSON configuration file to deserialize.
	let file_system_layout = FileSystemLayout::default();
	// TODO: This needs to have its 'affinity' overridden
		// Best option: Use all isolated CPUs
		// Second-best option: Use all CPUs
		// Alternative option: Use isolated CPUs for our process; force all other processes to run solely one one or two non-isolated cores; run our main thread on a non-isolated core
	// We can use sched_setaffinity to move processes (or threads, usings tid) but we need to identify processes we can safely 'touch'
	let process_configuration = ProcessConfiguration::default();
	let process_executor = ProcessExecutor::default();
	let run_as_daemon = false;
	
	let main_thread_configuration = ThreadConfiguration::default();
	let accept_child_thread_configuration = ThreadConfiguration::default();
	
	let main_thread_affinity = xxx;
	let accept_child_thread_affinity = xxx;
	
	struct ConfigureDataDog
	{
	}
	
	impl AdditionalLoggingConfiguration for ConfigureDataDog
	{
		fn configure(&mut self, host_name: Option<&LinuxKernelHostName>, domain_name: Option<&LinuxKernelDomainName>, internet_protocol_addresses: &[IpAddr], process_name: &ProcessName) -> Result<(), dyn error::Error + 'static>
		{
			Label::initialize_host_name(linux_kernel_host_name);
			DogStatsDTag::initialize_environment(linux_kernel_domain_name);
		}
	}
	
	let (terminate, defaults) = process_configuration.configure(run_as_daemon, &file_system_layout, &mut configure_data_dog).expect("Could not configure process");
	
	{
		let queues = Queues::one_queue_for_each_hyper_thread(&hyper_threads, message_handlers_and_preferred_maximum_number_of_elements_of_largest_possible_fixed_size_message_body_in_queue_for_hyper_thread, &defaults, inclusive_maximum_bytes_wasted);
		
		const SipOverTls: ServiceProtocolIdentifier = ServiceProtocolIdentifier(0);
		
		let accept_child_thread_function = ThreadLoopInitiation
		{
			defaults,
			global_allocator: &SwitchableGlobalAllocator,
			queues: queues.clone(),
			io_uring_settings: IoUringSettings,
			signal_mask: Signals(),
			
			dog_stats_d_message_subscribers: Box::new([]),
			
			transmission_control_protocol_over_internet_protocol_version_4_server_listeners: vec!
			[
				AcceptConnectionsCoroutineSettings
				{
					transmission_control_protocol_service_listener_settings: TransmissionControlProtocolServerListenerSettings
					{
						socket_address: (),
						send_buffer_size_in_bytes: (),
						receive_buffer_size_in_bytes: (),
						idles_before_keep_alive_seconds: Default::default(),
						keep_alive_interval_seconds: Default::default(),
						maximum_keep_alive_probes: Default::default(),
						socket_linger_seconds: Default::default(),
						finish_timeout_seconds: Default::default(),
						maximum_syn_retransmits: Default::default(),
						back_log: Default::default()
					},
					
					remote_peer_address_based_access_control: RemotePeerAddressBasedAccessControl::new(),
					service_protocol_identifier: SipOverTls,
				}
			],
			
			transmission_control_protocol_over_internet_protocol_version_6_server_listeners: vec!
			[
				AcceptConnectionsCoroutineSettings
				{
					transmission_control_protocol_service_listener_settings: TransmissionControlProtocolServerListenerSettings
					{
						socket_address: (),
						send_buffer_size_in_bytes: (),
						receive_buffer_size_in_bytes: (),
						idles_before_keep_alive_seconds: Default::default(),
						keep_alive_interval_seconds: Default::default(),
						maximum_keep_alive_probes: Default::default(),
						socket_linger_seconds: Default::default(),
						finish_timeout_seconds: Default::default(),
						maximum_syn_retransmits: Default::default(),
						back_log: Default::default()
					},
					
					remote_peer_address_based_access_control: RemotePeerAddressBasedAccessControl::new(),
					service_protocol_identifier: SipOverTls,
				}
			],
			
			streaming_unix_domain_socket_server_listener_server_listeners: vec!
			[
				AcceptConnectionsCoroutineSettings
				{
					transmission_control_protocol_service_listener_settings: TransmissionControlProtocolServerListenerSettings
					{
						socket_address: (),
						send_buffer_size_in_bytes: (),
						receive_buffer_size_in_bytes: (),
						idles_before_keep_alive_seconds: Default::default(),
						keep_alive_interval_seconds: Default::default(),
						maximum_keep_alive_probes: Default::default(),
						socket_linger_seconds: Default::default(),
						finish_timeout_seconds: Default::default(),
						maximum_syn_retransmits: Default::default(),
						back_log: Default::default()
					},
					
					remote_peer_address_based_access_control: RemotePeerAddressBasedAccessControl::new(),
					service_protocol_identifier: SipOverTls,
				}
			],
			
			marker: PhantomData,
		};
		
		let main_thread = ThreadSettings
		{
			thread_configuration: &main_thread_configuration,
			affinity: main_thread_affinity,
			thread_function: main_thread_function,
		};
		
		let child_threads= vec!
		[
			ThreadSettings
			{
				thread_configuration: &accept_child_thread_configuration,
				affinity: accept_child_thread_affinity,
				thread_function: accept_child_thread_function.clone(),
			}
		];
		
		let instantiation_arguments = Arc::new((defaults, &SwitchableGlobalAllocator));
		
		process_executor.execute_securely(&file_system_layout, terminate, main_thread, child_threads, instantiation_arguments).expect("Could not execute process")
	}
}
