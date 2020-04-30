// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::bit_set::*;
use crate::configuration::checks::Check;
use super::cpu::HyperThread;
use super::file_systems::FileSystemType;
use super::paths::PathExt;
use super::paths::ProcPath;
use crate::strings::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::OsString;
use std::fmt::Debug;
use std::io;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use strum_macros::*;


include!("IsolatedCpuFlags.rs");
include!("LinuxKernelCommandLineParameters.rs");
include!("OptionalKernelCommandLineSettingCheck.rs");
include!("validate_huge_page_sizes.rs");
include!("incompatible_settings.rs");
