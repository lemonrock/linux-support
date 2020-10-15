// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![feature(thread_local)]


use self::application_configuration::*;
use self::command_line::*;
use self::memory_allocator_settings::*;
use self::service_protocol_identifiers::*;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
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
use likely::unlikely;
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
use linux_support::configuration::GlobalComputedSchedulingConfiguration;
use linux_support::configuration::ProcessConfiguration;
use linux_support::configuration::ProcessConfigurationError;
use linux_support::configuration::ProcessExecutor;
use linux_support::configuration::ProcessExecutorError;
use linux_support::cpu::HyperThread;
use linux_support::cpu::HyperThreads;
use linux_support::file_descriptors::socket::UnixSocketAddress;
use linux_support::file_descriptors::socket::c::in_addr;
use linux_support::file_descriptors::socket::c::in6_addr;
use linux_support::memory::huge_pages::DefaultPageSizeAndHugePageSizes;
use linux_support::network_device::NetworkInterfaceName;
use linux_support::network_device::strategies::DevicePreferences;
use linux_support::network_device::strategies::DriverProfile;
use linux_support::network_device::strategies::DriverProfileError;
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
use std::convert::TryInto;
use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;
use std::error;
use std::ffi::OsString;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::marker::PhantomData;
use std::mem::size_of;
use std::mem::transmute;
use std::mem::zeroed;
use std::net::SocketAddrV4;
use std::net::SocketAddrV6;
use std::net::Ipv6Addr;
use std::net::Ipv4Addr;
use std::num::NonZeroU32;
use std::ops::Sub;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::ptr::NonNull;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use swiss_army_knife::big_endian::BigEndianI16;
use swiss_army_knife::big_endian::BigEndianU16;
use swiss_army_knife::big_endian::BigEndianU32;
use swiss_army_knife::bit_set::BitSet;
use swiss_army_knife::exponents_of_two::SignedExponentOfTwo8;
use swiss_army_knife::exponents_of_two::UnsignedExponentOfTwo8;
use swiss_army_knife::fixed_point_arithmetic::Signed1616FixedPoint;
use swiss_army_knife::fixed_point_arithmetic::Signed3232FixedPoint;
use swiss_army_knife::fixed_point_arithmetic::Unsigned1616FixedPoint;
use swiss_army_knife::fixed_point_arithmetic::Unsigned3232FixedPoint;
use swiss_army_knife::internet_protocol::InternetProtocolAddress;
use swiss_army_knife::internet_protocol::InternetProtocolAddressWithMask;
use terminate::Terminate;


mod application_configuration;


mod command_line;


mod memory_allocator_settings;


mod service_protocol_identifiers;


mod simple_network_time_protocol;


include!("configure_and_execute.rs");


#[global_allocator] static SwitchableGlobalAllocator: GTACSA = GlobalThreadAndCoroutineSwitchableAllocatorInstance::system(per_thread_state);


/// Main method.
pub fn main() -> Result<(), ConfigurationError>
{
	let (run_as_daemon, configuration) = parse_command_line(parse_matches)?;
	configure_and_execute(run_as_daemon, configuration);
	Ok(())
}
