// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![feature(allocator_api)]
#![feature(const_if_match)]
#![feature(core_intrinsics)]
#![feature(thread_local)]


//! #linux-io-uring
//!
//! This is a mid-level library to provide a coroutine based way to work with io-uring across many cores.
//!
//! See <https://github.com/lemonrock/linux-support> for more details.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use self::coroutines::*;
use self::coroutines::accept::required::*;
use self::coroutines::accept::specific::*;
use self::dogstatsd::*;
use self::registered_buffers::*;
use arrayvec::ArrayString;
use arrayvec::ArrayVec;
use context_allocator::GlobalThreadAndCoroutineSwitchableAllocator;
use context_allocator::LifetimeHint;
use context_allocator::LocalAllocator;
use context_allocator::allocators::Allocator;
use context_allocator::memory_sources::MemoryMapSource;
use context_coroutine::Coroutine;
use context_coroutine::CoroutineInstanceHandle;
use context_coroutine::CoroutineInstancePointer;
use context_coroutine::CoroutineManager;
use context_coroutine::CoroutineManagerIndex;
use context_coroutine::ResumeOutcome;
use context_coroutine::StartOutcome;
use context_coroutine::UserBits;
use context_coroutine::Yielder;
use context_coroutine::choose_coroutine_manager;
use either::Either;
use either::Left;
use either::Right;
use lazy_static::lazy_static;
use likely::likely;
use likely::unlikely;
use linux_support::cpu::HyperThread;
use linux_support::file_descriptors::CreationError;
use linux_support::file_descriptors::signalfd::SignalFileDescriptor;
use linux_support::file_descriptors::socket::AcceptedConnection;
use linux_support::file_descriptors::socket::BackLog;
use linux_support::file_descriptors::socket::Blocking;
use linux_support::file_descriptors::socket::BlockingDuration;
use linux_support::file_descriptors::socket::ConnectionFailedReason;
use linux_support::file_descriptors::socket::DatagramClientListenerSocketFileDescriptor;
use linux_support::file_descriptors::socket::InternetProtocolSocketSettings;
use linux_support::file_descriptors::socket::NewSocketServerListenerError;
use linux_support::file_descriptors::socket::PendingAcceptConnection;
use linux_support::file_descriptors::socket::SocketAcceptError;
use linux_support::file_descriptors::socket::SocketAddress;
use linux_support::file_descriptors::socket::SocketData;
use linux_support::file_descriptors::socket::StreamingServerListenerSocketFileDescriptor;
use linux_support::file_descriptors::socket::StreamingSocketFileDescriptor;
use linux_support::file_descriptors::socket::TransmissionControlProtocolSocketSettings;
use linux_support::file_descriptors::socket::UnixSocketAddress;
use linux_support::file_descriptors::socket::c::in_addr;
use linux_support::io_priority::CompressedIoPriority;
use linux_support::io_priority::IoPriority;
use linux_support::io_uring::CompletionResponse;
use linux_support::io_uring::FileDescriptorOrigin;
use linux_support::io_uring::IoUring;
use linux_support::io_uring::IoUringCreationError;
use linux_support::io_uring::LinuxKernelSubmissionQueuePollingThreadConfiguration;
use linux_support::io_uring::RegisteredBufferIndex;
use linux_support::io_uring::SubmitError;
use linux_support::io_uring::SubmissionQueueEntry;
use linux_support::io_uring::SubmissionQueueEntryOptions;
use linux_support::linux_kernel_version::LinuxKernelDomainName;
use linux_support::linux_kernel_version::LinuxKernelHostName;
use linux_support::logging::AdditionalLoggingConfiguration;
use linux_support::memory::huge_pages::DefaultPageSizeAndHugePageSizes;
use linux_support::memory::mapping::MemoryMapError;
use linux_support::process::ProcessName;
use linux_support::signals::Signals;
use linux_support::thread::PerThreadMemoryAllocatorInstantiator;
use linux_support::thread::ThreadFunction;
use linux_support::thread::ThreadLocalAllocatorConfiguration;
use linux_support::thread::ThreadLoopBodyFunction;
use magic_ring_buffer::LargeRingQueueCreationError;
use magic_ring_buffer::ReferenceCountedLargeRingQueue;
use magic_ring_buffer::ReferenceCountedLargeRingQueueElement;
use magic_ring_buffer::memory_sizes::MemorySize;
use magic_ring_buffer::memory_sizes::MemorySize1Gb;
use magic_ring_buffer::memory_sizes::MemorySize1Mb;
use magic_ring_buffer::memory_sizes::MemorySize4Kb;
use magic_ring_buffer::memory_sizes::MemorySize4Mb;
use magic_ring_buffer::memory_sizes::MemorySize16Kb;
use magic_ring_buffer::memory_sizes::MemorySize16Mb;
use magic_ring_buffer::memory_sizes::MemorySize64Kb;
use magic_ring_buffer::memory_sizes::MemorySize64Mb;
use magic_ring_buffer::memory_sizes::MemorySize256Kb;
use magic_ring_buffer::memory_sizes::MemorySize256Mb;
use message_dispatch::Message;
use message_dispatch::Publisher;
use message_dispatch::Queues;
use message_dispatch::RoundRobinPublisher;
use message_dispatch::Subscriber;
use message_dispatch_datadog::DequeuedMessageProcessingError;
use message_dispatch_datadog::additional_dog_stats_d_tags;
use message_dispatch_datadog::alert;
use message_dispatch_datadog::dogstatsd::AdditionalDogStatsDTag;
use message_dispatch_datadog::dogstatsd::AdditionalDogStatsDTags;
use message_dispatch_datadog::dogstatsd::DogStatsDMessage;
use message_dispatch_datadog::dogstatsd::DogStatsDTag;
use message_dispatch_datadog::dogstatsd::Label;
use message_dispatch_datadog::dogstatsd::ThreadLocalNumericAdditionalDogStatsDTagsCache;
use message_dispatch_datadog::dogstatsd::event::EventTemplate;
use message_dispatch_datadog::dogstatsd::metric::MetricTemplate;
use message_dispatch_datadog::dogstatsd::metric::MetricValue;
use phf::Map;
use phf::phf_map;
use serde::Deserialize;
use serde::Serialize;
use socket_access_control::AccessControl;
use socket_access_control::InternetProtocolVersion4AccessControl;
use socket_access_control::InternetProtocolVersion6AccessControl;
use socket_access_control::UnixDomainSocketAccessControl;
use std::alloc::AllocError;
use std::alloc::AllocRef;
use std::alloc::Layout;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::cmp::min;
use std::cmp::Ordering;
use std::error;
use std::fmt::Arguments;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::io;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::mem::align_of;
use std::mem::forget;
use std::mem::size_of;
use std::mem::transmute;
#[allow(deprecated)] use std::mem::uninitialized;
use std::mem::zeroed;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::net::SocketAddrV4;
use std::net::SocketAddrV6;
use std::num::NonZeroU8;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::ptr::NonNull;
use std::ptr::copy_nonoverlapping;
use std::ptr::write;
use std::ptr::write_bytes;
use std::rc::Rc;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use std::sync::Arc;
use std::time::SystemTime;
use swiss_army_knife::big_endian::BigEndianI16;
use swiss_army_knife::big_endian::BigEndianU128;
use swiss_army_knife::big_endian::BigEndianU16;
use swiss_army_knife::big_endian::BigEndianU32;
use swiss_army_knife::exponents_of_two::SignedExponentOfTwo8;
use swiss_army_knife::exponents_of_two::UnsignedExponentOfTwo8;
use swiss_army_knife::fixed_point_arithmetic::Signed1616FixedPoint;
use swiss_army_knife::fixed_point_arithmetic::Signed3232FixedPoint;
use swiss_army_knife::fixed_point_arithmetic::Unsigned1616FixedPoint;
use swiss_army_knife::fixed_point_arithmetic::Unsigned3232FixedPoint;
use swiss_army_knife::internet_protocol::InternetProtocolAddress;
use terminate::Terminate;
use time::Timespec;


/// Coroutines.
pub mod coroutines;


/// DogStatsD.
pub mod dogstatsd;


/// Registered buffers.
pub mod registered_buffers;


/// Thread local allocator.
pub mod thread_local_allocator;


include!("DispatchIoUringError.rs");include!("IoUringSettings.rs");include!("IoUringSetupError.rs");
include!("MessageHandlerArguments.rs");
include!("ThreadLocalCoroutineManagers.rs");
include!("ThreadLoopInitiation.rs");
include!("ThreadLoopInitializationError.rs");
include!("ThreadLoop.rs");
