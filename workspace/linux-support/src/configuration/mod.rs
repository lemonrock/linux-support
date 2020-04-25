// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::capabilities_and_privileges::*;
use crate::cpu::*;
use crate::thread::ThreadName;
use crate::file_descriptors::epoll::set_maximum_number_of_watched_file_descriptors_per_user;
use crate::file_descriptors::file::leasing::*;
use crate::file_descriptors::inotify::*;
use crate::file_descriptors::pipes_and_fifos::*;
use crate::file_descriptors::posix_message_queues::*;
use crate::linux_kernel_asynchronous_io::set_maximum_number_of_kernel_asynchronous_io_events_per_user;
use crate::linux_kernel_version::*;
use crate::memory::*;
use crate::memory::huge_pages::PageSizeOrHugePageSize::PageSize;
use crate::memory::system_v_shared_memory::message_queues::*;
use crate::paths::*;
use crate::process::*;
use crate::scheduling::*;
use crate::user_and_groups::assert_effective_user_id_is_root;
use raw_cpuid::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::thread::Builder;
use std::thread::JoinHandle;
use crate::file_handles::NumberOfFileHandles;


include!("FailedChecks.rs");
include!("GlobalConfiguration.rs");
include!("GlobalConfigurationError.rs");
include!("GlobalEPollConfiguration.rs");
include!("GlobalEPollConfigurationError.rs");
include!("GlobalFileHandleConfiguration.rs");
include!("GlobalFileHandleConfigurationError.rs");
include!("GlobalFileLeasingConfiguration.rs");
include!("GlobalFileLeasingConfigurationError.rs");
include!("GlobalInotifyConfiguration.rs");
include!("GlobalInotifyConfigurationError.rs");
include!("GlobalLinuxKernelAsynchronousIoConfiguration.rs");
include!("GlobalLinuxKernelAsynchronousIoConfigurationError.rs");
include!("GlobalPipeConfiguration.rs");
include!("GlobalPipeConfigurationError.rs");
include!("GlobalPosixMessageQueueConfiguration.rs");
include!("GlobalPosixMessageQueueConfigurationError.rs");
include!("GlobalSchedulingConfiguration.rs");
include!("GlobalSchedulingConfigurationError.rs");
include!("GlobalSystemVMessageQueueConfiguration.rs");
include!("GlobalSystemVMessageQueueConfigurationError.rs");
include!("ProcessNiceConfiguration.rs");
include!("ProcessNiceConfigurationError.rs");
include!("ProcessConfiguration.rs");
include!("ProcessConfigurationError.rs");
include!("ProcessSchedulingConfiguration.rs");
include!("ProcessSchedulingConfigurationError.rs");
include!("ThreadConfiguration.rs");
include!("ThreadConfigurationError.rs");
