// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]
#![feature(thread_local)]


//! #message-dispatch-datadog
//!
//! This is a library to use message-dispatch and io_uring to send DataDog information.
//!
//! See <https://github.com/lemonrock/linux-support> for more details.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use arrayvec::Array;
use arrayvec::ArrayString;
use arrayvec::ArrayVec;
use context_allocator::GloballyAllocated;
use context_allocator::GlobalThreadAndCoroutineSwitchableAllocator;
use either::Either;
use either::Either::Left;
use either::Either::Right;
use lazy_static::lazy_static;
use likely::unlikely;
use linux_support::cpu::HyperThread;
use linux_support::linux_kernel_version::LinuxKernelHostName;
use linux_support::thread::ThreadName;
use magic_ring_buffer::memory_sizes::MemorySize;
use message_dispatch::Message;
use memchr::memchr;
use memchr::memchr2;
use std::borrow::Cow;
use std::cmp::min;
use std::fmt::Arguments;
use std::fmt::format;
use std::marker::PhantomData;
use std::ops::Deref;
use std::panic::AssertUnwindSafe;
use std::panic::UnwindSafe;
use std::ptr::NonNull;
use std::time::Duration;
use std::time::SystemTime;


/// DogStatsD support.
///
/// Datagram format is described in <https://docs.datadoghq.com/developers/dogstatsd/datagram_shell?tab=metrics>.
pub mod dogstatsd;
