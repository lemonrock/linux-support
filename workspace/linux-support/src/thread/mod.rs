// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::bit_set::BitSet;
use crate::configuration::ProcessLoggingConfiguration;
use crate::cpu::HyperThread;
use crate::logging::panic_payload_to_cause;
use crate::memory::*;
use crate::memory::huge_pages::adjust_transparent_huge_pages;
use crate::paths::*;
use crate::process::*;
use crate::scheduling::PerThreadSchedulerPolicyAndFlags;
use crate::strings::parse_number::*;
use crate::syscall::SYS::gettid;
use arrayvec::ArrayVec;
use libc::c_char;
use libc::pid_t;
use libc::PR_GET_NAME;
use libc::PR_SET_NAME;
use libc::prctl;
use libc::pthread_self;
use libc::pthread_t;
use likely::likely;
use memchr::memchr;
use serde::Deserialize;
use serde::Serialize;
use std::backtrace::Backtrace;
use std::backtrace::BacktraceStatus;
use std::convert::TryFrom;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::process;
use std::num::NonZeroI32;
use std::ops::Deref;
use std::panic::*;
use std::slice::from_raw_parts;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Release;
use std::thread;
use std::thread::Builder;
use std::thread::JoinHandle;
use std::thread::park;
use std::thread::Thread;
use std::thread::ThreadId;
use std::mem::transmute;
#[allow(deprecated)] use std::mem::uninitialized;
use terminate::Terminate;


include!("configure_global_panic_hook.rs");
include!("JoinHandles.rs");
include!("SimpleBarrier.rs");
include!("ThreadConfiguration.rs");
include!("ThreadConfigurationError.rs");
include!("ThreadIdentifier.rs");
include!("ThreadIdentifierChoice.rs");
include!("ThreadIdentifiers.rs");
include!("ThreadLoopBodyFunction.rs");
include!("ThreadName.rs");
