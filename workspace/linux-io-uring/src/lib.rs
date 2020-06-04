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


use self::registered_buffers::*;
use context_allocator::allocators::global::*;
use context_allocator::memory_sources::MemoryMapSource;
use context_coroutine::CoroutineManager;
use context_coroutine::Coroutine;
use context_coroutine::CoroutineInstanceHandle;
use context_coroutine::Yielder;
use likely::unlikely;
use linux_support::bit_set::*;
use linux_support::cpu::HyperThread;
use linux_support::file_descriptors::CreationError;
use linux_support::file_descriptors::signalfd::SignalFileDescriptor;
use linux_support::file_descriptors::socket::*;
use linux_support::io_uring::*;
use linux_support::logging::*;
use linux_support::memory::huge_pages::*;
use linux_support::memory::mapping::*;
use linux_support::memory::numa::*;
use linux_support::signals::Signal;
use linux_support::thread::*;
use magic_ring_buffer::*;
use magic_ring_buffer::memory_sizes::*;
use maplit::hashset;
use message_dispatch::Queues;
use serde::Deserialize;
use serde::Serialize;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::marker::PhantomData;
use std::net::SocketAddrV4;
use std::num::NonZeroUsize;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::rc::Rc;
use socket_access_control::*;
use terminate::Terminate;
use linux_support::file_descriptors::socket::c::{sockaddr_in, sockaddr_in6};
use std::path::PathBuf;


/// Coroutines
pub mod coroutines;


mod registered_buffers;


include!("DequeuedMessageProcessingError.rs");
include!("IoUringSettings.rs");
include!("IoUringSetupError.rs");
include!("ThreadLoopInitiation.rs");
include!("ThreadLoopInitializationError.rs");
include!("ThreadLocalAllocatorSettings.rs");
include!("ThreadLocalAllocatorSettingsDropGuard.rs");
include!("ThreadLoop.rs");
