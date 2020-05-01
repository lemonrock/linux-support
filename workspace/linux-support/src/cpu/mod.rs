// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::current_numa_node_and_hyper_thread;
use crate::bit_set::*;
use crate::configuration::checks::*;
use crate::linux_kernel_command_line::*;
use crate::memory::numa::*;
use crate::paths::*;
use crate::process::ProcessIdentifierChoice;
use crate::process::status::Status;
use crate::strings::*;
use crate::strings::parse_number::ParseNumberError;
use crate::user_and_groups::assert_effective_user_id_is_root;
use errno::errno;
use libc::*;
use likely::*;
#[cfg(target_arch = "x86_64")] use raw_cpuid::*;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error;
use std::io;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::size_of;
#[allow(deprecated)] use std::mem::uninitialized;
use std::path::PathBuf;
use strum_macros::*;


include!("BitSetHyperThread.rs");
include!("CompiledCpuFeatureCheck.rs");
include!("cpu_supports_1gb_pages.rs");
include!("HyperThread.rs");
include!("HyperThreadingStatus.rs");
include!("maximum_logical_processor_identifiers_per_package.rs");
include!("MandatoryCpuFeatureCheck.rs");
include!("OptionalCpuFeatureCheck.rs");
include!("ParseHyperThreadingStatusError.rs");
include!("PerBitSetAwareDataHyperThread.rs");
