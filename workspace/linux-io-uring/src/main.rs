// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![feature(thread_local)]


use self::main_support::*;
use clap::App;
use clap::Arg;
use clap::crate_name;
use clap::crate_authors;
use clap::crate_version;
use context_allocator::GlobalThreadAndCoroutineSwitchableAllocatorInstance;
use context_allocator::PerThreadState;
use context_allocator::adaptors::GlobalAllocToAllocatorAdaptor;
use context_allocator::allocators::ContextAllocator;
use context_allocator::allocators::binary_search_trees::MultipleBinarySearchTreeAllocator;
use context_allocator::memory_sources::CoroutineHeapMemorySource;
use context_allocator::memory_sources::MemoryMapSource;
use linux_io_uring::IoUringSettings;
use linux_io_uring::ThreadLoopInitiation;
use linux_io_uring::coroutines::accept::AcceptConnectionsCoroutineSettings;
use linux_io_uring::coroutines::accept::AccessControlValue;
use linux_io_uring::coroutines::accept::ServiceProtocolIdentifier;
use linux_io_uring::coroutines::accept::TransmissionControlProtocolServerListenerSettings;
use linux_io_uring::dogstatsd::DogStatsDStaticInitialization;
use linux_io_uring::registered_buffers::RegisteredBufferSettings;
use linux_io_uring::registered_buffers::RegisteredBufferSetting;
use linux_io_uring::thread_local_allocator::SimplePerThreadMemoryAllocatorInstantiator;
use linux_support::configuration::{ProcessConfiguration, GlobalComputedSchedulingConfiguration};
use linux_support::configuration::ProcessExecutor;
use linux_support::cpu::HyperThread;
use linux_support::file_descriptors::socket::UnixSocketAddress;
use linux_support::file_descriptors::socket::c::in_addr;
use linux_support::file_descriptors::socket::c::in6_addr;
use linux_support::memory::huge_pages::DefaultPageSizeAndHugePageSizes;
use linux_support::paths::FileSystemLayout;
use linux_support::signals::Signals;
use linux_support::thread::ThreadConfiguration;
use linux_support::thread::ThreadFunction;
use linux_support::thread::ThreadSettings;
use linux_support::user_and_groups::GroupIdentifier;
use linux_support::user_and_groups::UserIdentifier;
use maplit::btreemap;
use maplit::hashmap;
use magic_ring_buffer::memory_sizes::MemorySize64Kb;
use message_dispatch::Queues;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_reader;
use socket_access_control::InternetProtocolVersion4AccessControl;
use socket_access_control::InternetProtocolVersion6AccessControl;
use socket_access_control::UnixDomainSocketAccessControl;
use std::alloc::System;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddrV4;
use std::net::SocketAddrV6;
use std::net::Ipv6Addr;
use std::net::Ipv4Addr;
use std::num::NonZeroU32;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::ptr::NonNull;
use std::sync::Arc;
use swiss_army_knife::bit_set::BitSet;
use swiss_army_knife::internet_protocol::InternetProtocolAddressWithMask;
use terminate::Terminate;


mod main_support;


#[global_allocator] static SwitchableGlobalAllocator: GTACSA = GlobalThreadAndCoroutineSwitchableAllocatorInstance::system(per_thread_state);


pub fn main()
{
	let default_configuration_file_path = format!("../../etc/{}.conf", crate_name!());
	
	const ConfigurationName: &'static str = "configuration";
	const ConfigurationValue: &'static str = "FILE";
	const RunAsDaemonName: &'static str = "runasdaemon";
	
	#[allow(deprecated)]
	let matches = App::new(crate_name!())
	.version(crate_version!())
	.author(crate_authors!("\n"))
	.about("Ultra-high performance socket server using Linux's io_uring")
	.before_help("")
	.after_help("")
	.arg
	(
		Arg::with_name(ConfigurationName)
		.required(false)
		.short("c")
		.long(ConfigurationName)
		.takes_value(true)
		.value_name(ConfigurationValue)
		.allow_hyphen_values(true)
		.empty_values(false)
		.help("Sets a JSON configuration file")
		.validator_os(|FILE|
		{
			if PathBuf::from(FILE).is_file()
			{
				Ok(())
			}
			else
			{
				Err(OsString::from(format!("The --{} {} {} is not a extant, readable and regular file", ConfigurationName, ConfigurationValue, FILE.to_string_lossy())))
			}
		})
	)
	.arg
	(
		Arg::with_name(RunAsDaemonName)
		.short("d")
		.long(RunAsDaemonName)
		.takes_value(false)
		.multiple(false)
		.help("Runs as a daemon")
	)
	.get_matches();
	
	
	let run_as_daemon = matches.is_present(RunAsDaemonName);
	
	let configuration = match matches.value_of_os(ConfigurationName)
	{
		Some(FILE) => match Configuration::from_json_file(FILE)
		{
			Ok(configuration) => configuration,
			Err(error) =>
			{
				eprintln!("The --{} {} {} is invalid: {}", ConfigurationName, ConfigurationValue, FILE.to_string_lossy(), error);
				exit(1);
			}
		},
		
		None => Configuration::defaultish
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
			},
			80
		),
	};
	
	configure_and_execute(run_as_daemon, configuration)
}

fn configure_and_execute(run_as_daemon: bool, configuration: Configuration)
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
	
	
	
	let terminate = configuration.configure(run_as_daemon, Some(&global_computed_scheduling_affinity), Some(&process_affinity));
	
	let queues = Queues::one_queue_for_each_hyper_thread(&queue_hyper_threads, message_handlers_and_preferred_maximum_number_of_elements_of_largest_possible_fixed_size_message_body_in_queue_for_hyper_thread, &defaults, inclusive_maximum_bytes_wasted);
	
	let defaults = DefaultPageSizeAndHugePageSizes::new(sys_path, proc_path);
	
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
