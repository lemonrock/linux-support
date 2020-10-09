// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) fn configure_and_execute(run_as_daemon: bool, configuration: Configuration)
{
	let (transmission_control_protocol_over_internet_protocol_version_4_server_listeners, transmission_control_protocol_over_internet_protocol_version_6_server_listeners, streaming_unix_domain_socket_server_listener_server_listeners) = configuration.server_listeners();
	
	let global_computed_scheduling_affinity = GlobalComputedSchedulingConfiguration
	{
		software_and_hardware_watchdog_runs_on_which_kernel_cpus: XXX,
		work_queue_runs_on_which_kernel_cpus: XXX,
		default_interrupt_request_affinity: XXX,
		interrupt_request_affinity: Default::default(),
		receive_packet_steering_flow_limit_tables: XXX,
	};
	
	let process_affinity = xxx;
	// TODO: Most be subsets of process configuration affinity.
	let main_thread_affinity = xxx;
	let accept_child_thread_affinity = xxx;
	let queue_hyper_threads = xxx;
	let dog_stats_d_message_subscribers = vec!
	[
		HyperThread::_1(),
		HyperThread::_2(),
	];
	
	let message_handlers_and_preferred_maximum_number_of_elements_of_largest_possible_fixed_size_message_body_in_queue_for_hyper_thread;
	let inclusive_maximum_bytes_wasted;
	
	
	let (terminate, defaults) = configuration.configure(run_as_daemon, Some(&global_computed_scheduling_affinity), Some(&process_affinity));

// TODO: Amazon ENA: If the NETIF_F_RXHASH flag is set, the 32-bit result of the hash function delivered in the Rx CQ descriptor is set in the received SKB.
// TODO: Use configure_all_multiqueue_pci_ethernet_devices().
	xxxxx;
	
	let sys_path = &configuration.file_system_layout.sys_path;
	let proc_path = &configuration.file_system_layout.proc_path;
	let admin_queue_thread_and_queues_threads: HashMap<NetworkInterfaceName, (HyperThread, HyperThreads)> = DriverProfile::configure_all_multiqueue_pci_ethernet_devices(sys_path, proc_path, &configuration.device_preferences).xxxxxx_error;
	
	let queues = Queues::one_queue_for_each_hyper_thread(&queue_hyper_threads, message_handlers_and_preferred_maximum_number_of_elements_of_largest_possible_fixed_size_message_body_in_queue_for_hyper_thread, &defaults, inclusive_maximum_bytes_wasted);
	
	let accept_child_thread_function = ThreadLoopInitiation::new
	(
		defaults.clone(),
		&SwitchableGlobalAllocator,
		queues,
		Signals(BitSet::empty()),
		dog_stats_d_message_subscribers.into_boxed_slice(),
		
		configuration.io_uring_settings.clone(),
		transmission_control_protocol_over_internet_protocol_version_4_server_listeners,
		transmission_control_protocol_over_internet_protocol_version_6_server_listeners,
		streaming_unix_domain_socket_server_listener_server_listeners,
	);
	
	let main_thread = ThreadSettings
	{
		thread_configuration: &configuration.main_thread_configuration,
		affinity: main_thread_affinity,
		thread_function: main_thread_function,
	};
	
	let child_threads= vec!
	[
		ThreadSettings
		{
			thread_configuration: &configuration.accept_child_thread_configuration,
			affinity: accept_child_thread_affinity,
			thread_function: accept_child_thread_function.clone(),
		}
	];
	
	configuration.execute(terminate, main_thread, child_threads, defaults)
}
