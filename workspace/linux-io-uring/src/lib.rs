// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]


//! #linux-io-uring
//!
//! This is a mid-level library to provide a coroutine based way to work with io-uring across many cores.
//!
//! See <https://github.com/lemonrock/linux-support> for more details.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use self::coroutines::*;
use self::coroutines::accept::*;
use self::dogstatsd::*;
use self::registered_buffers::*;
use arrayvec::Array;
use arrayvec::ArrayString;
use arrayvec::ArrayVec;
use context_allocator::allocators::global::*;
use context_allocator::GlobalThreadAndCoroutineSwitchableAllocator;
use context_allocator::LifetimeHint;
use context_allocator::memory_sources::MemoryMapSource;
use context_coroutine::CoroutineManager;
use context_coroutine::CoroutineManagerIndex;
use context_coroutine::UserBits;
use context_coroutine::CoroutineInstancePointer;
use context_coroutine::ResumeOutcome;
use context_coroutine::StartOutcome;
use context_coroutine::Coroutine;
use context_coroutine::CoroutineInstanceHandle;
use context_coroutine::Yielder;
use either::Either;
use either::Either::Left;
use either::Either::Right;
use lazy_static::lazy_static;
use likely::unlikely;
use linux_support::bit_set::*;
use linux_support::cpu::HyperThread;
use linux_support::file_descriptors::CreationError;
use linux_support::file_descriptors::signalfd::SignalFileDescriptor;
use linux_support::file_descriptors::socket::*;
use linux_support::file_descriptors::socket::c::*;
use linux_support::io_uring::*;
use linux_support::linux_kernel_version::LinuxKernelHostName;
use linux_support::logging::*;
use linux_support::memory::huge_pages::*;
use linux_support::memory::mapping::*;
use linux_support::memory::numa::*;
use linux_support::signals::Signal;
use linux_support::signals::Signals;
use linux_support::thread::*;
use magic_ring_buffer::*;
use magic_ring_buffer::memory_sizes::*;
use maplit::hashset;
use message_dispatch::Message;
use message_dispatch::Publisher;
use message_dispatch::RoundRobinPublisher;
use message_dispatch::Queues;
use message_dispatch::Subscriber;
use message_dispatch_datadog::additional_dog_stats_d_tags;
use message_dispatch_datadog::alert;
use message_dispatch_datadog::dog_stats_d_tags;
use message_dispatch_datadog::dogstatsd::AdditionalDogStatsDTag;
use message_dispatch_datadog::dogstatsd::AdditionalDogStatsDTags;
use message_dispatch_datadog::dogstatsd::DogStatsDMessage;
use message_dispatch_datadog::dogstatsd::DogStatsDTag;
use message_dispatch_datadog::dogstatsd::DogStatsDTags;
use message_dispatch_datadog::dogstatsd::Label;
use message_dispatch_datadog::dogstatsd::Name;
use message_dispatch_datadog::dogstatsd::ThreadLocalNumericAdditionalDogStatsDTagsCache;
use message_dispatch_datadog::dogstatsd::event::Event;
use message_dispatch_datadog::dogstatsd::event::EventAlertType;
use message_dispatch_datadog::dogstatsd::event::EventPriority;
use message_dispatch_datadog::dogstatsd::event::EventTemplate;
use message_dispatch_datadog::dogstatsd::event::SourceTypeName;
use message_dispatch_datadog::dogstatsd::metric::Metric;
use message_dispatch_datadog::dogstatsd::metric::MetricTemplate;
use message_dispatch_datadog::dogstatsd::metric::MetricValue;
use serde::Deserialize;
use serde::Serialize;
use std::alloc::AllocErr;
use std::borrow::Cow;
use std::cmp::min;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Arguments;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::marker::PhantomData;
use std::mem::forget;
use std::net::SocketAddrV4;
use std::num::NonZeroUsize;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::ptr::NonNull;
use std::sync::Arc;
use std::time::Duration;
use std::rc::Rc;
use socket_access_control::*;
use terminate::Terminate;


/// Coroutines
pub mod coroutines;


mod dogstatsd;


mod registered_buffers;


mod thread_local_allocator;


include!("CoroutineManagers.rs");
include!("DequeuedMessageProcessingError.rs");
include!("DispatchIoUringError.rs");
include!("IoUringSettings.rs");
include!("IoUringSetupError.rs");
include!("MessageHandlerArguments.rs");
include!("ThreadLoopInitiation.rs");
include!("ThreadLoopInitializationError.rs");
include!("ThreadLoop.rs");
