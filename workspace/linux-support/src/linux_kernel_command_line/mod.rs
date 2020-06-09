// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::configuration::checks::Check;
use crate::cpu::HyperThread;
use crate::cpu::HyperThreads;
use crate::file_systems::FileSystemType;
use crate::paths::*;
include!("IsolatedCpuFlags.rs");
include!("LinuxKernelCommandLineParameters.rs");
include!("OptionalKernelCommandLineSettingCheck.rs");
include!("validate_huge_page_sizes.rs");
include!("incompatible_settings.rs");
