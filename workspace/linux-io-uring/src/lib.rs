// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]


//! #linux-io-uring
//!
//! This is a mid-level library to provide a coroutine based way to work with io-uring across many cores.
//!
//! See <https://github.com/lemonrock/linux-support> for more details.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use self::queues::*;
use self::registered_buffers::*;
use linux_support::file_descriptors::CreationError;
use linux_support::io_uring::*;
use linux_support::logging::ProcessLoggingConfiguration;
use linux_support::logging::SyslogPriority;
use linux_support::memory::mapping::*;
use linux_support::memory::huge_pages::{DefaultPageSizeAndHugePageSizes, HugePageSize};
use linux_support::thread::ThreadFunction;
use linux_support::thread::ThreadLoopBodyFunction;
use hashset::hashset;
use message_dispatch::{PerThreadQueueSubscriber, Queue, MessageHandlersRegistration};
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::mem::size_of;
use std::mem::transmute;
use std::mem::zeroed;
use std::marker::PhantomData;
use std::sync::Arc;
use terminate::Terminate;
use std::collections::{BTreeSet, BTreeMap};
use linux_support::file_descriptors::signalfd::SignalFileDescriptor;
use linux_support::bit_set::{BitSet, BitSetAware};
use linux_support::signals::Signal;
use context_coroutine::CoroutineMemoryWarehouse;
use context_allocator::allocators::global::{GlobalThreadAndCoroutineSwitchableAllocator, LocalAllocator, LifetimeHint};
use context_allocator::memory_sources::MemoryMapSource;
use linux_support::memory::numa::{SetMemoryPolicy, SetMemoryPolicyStrictness, NumaNode};
use linux_support::memory::mapping::Sharing::Shared;
use magic_ring_buffer::LargeRingQueueCreationError;
use linux_support::cpu::HyperThread;
use message_dispatch::erased_boxed_functions::Register;


mod registered_buffers;


mod queues;


include!("CoroutineParallelOperationSlots.rs");
include!("CoroutineUserData.rs");
include!("DequeuedMessageProcessingError.rs");
include!("IoUringSettings.rs");
include!("IoUringSetupError.rs");
include!("Operation.rs");
include!("OriginalRequestCancelationKind.rs");
include!("ThreadLoopInitializationError.rs");
include!("ThreadLocalAllocatorSettings.rs");
include!("ThreadLoop.rs");
