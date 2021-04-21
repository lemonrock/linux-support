// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::synchronization::*;
use super::*;
use crate::cpu::HyperThreads;
use crate::capabilities_and_privileges::*;
use crate::io_priority::IoPriority;
use crate::logging::LocalSyslogSocket;
use crate::memory::*;
use crate::memory::huge_pages::*;
use crate::memory::mapping::*;
use crate::memory::numa::*;
use crate::paths::*;
use crate::process::*;
use crate::process_control::error_number_to_io_error;
use crate::process_control::process_control_wrapper2;
use crate::process_control::result_must_be_zero;
use crate::process_control::SecureBits;
use crate::scheduling::Nice;
use crate::scheduling::PerThreadSchedulerPolicyAndFlags;
use crate::speculation_mitigation::*;
use crate::time::CurrentTimerSlackNanoseconds;
use crate::file_descriptors::socket::NewSocketClientError;


/// Thread synchronization.
pub mod synchronization;


include!("configure_global_panic_hook.rs");
include!("MainThreadConfigurationError.rs");
include!("PerThreadMemoryAllocatorInstantiator.rs");
include!("SpawnedThread.rs");
include!("SpawnedThreadError.rs");
include!("SpawnedThreads.rs");
include!("ThreadCapabilitiesConfiguration.rs");
include!("ThreadCapabilitiesConfigurationError.rs");
include!("ThreadConfiguration.rs");
include!("ThreadConfigurationError.rs");
include!("ThreadFunction.rs");
include!("ThreadIdentifier.rs");
include!("ThreadIdentifierChoice.rs");
include!("ThreadLocalAllocatorConfiguration.rs");
include!("ThreadLoopBodyFunction.rs");
include!("ThreadName.rs");
include!("ThreadSettings.rs");
