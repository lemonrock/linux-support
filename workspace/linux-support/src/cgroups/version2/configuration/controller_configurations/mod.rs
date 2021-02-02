// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::memory::huge_pages::DefaultHugePageSizes;


include!("AllControllersConfiguration.rs");
include!("configure_controller.rs");
include!("ControllerConfiguration.rs");
include!("ControllersConfiguration.rs");
include!("CpuControllerConfiguration.rs");
include!("CpusetControllerConfiguration.rs");
include!("DebugControllerConfiguration.rs");
include!("DomainControllersConfiguration.rs");
include!("HugetlbControllerConfiguration.rs");
include!("IoControllerConfiguration.rs");
include!("MemoryControllerConfiguration.rs");
include!("PerfEventControllerConfiguration.rs");
include!("PerHugePageSizeHugetlbControllerConfiguration.rs");
include!("PidsControllerConfiguration.rs");
include!("RdmaControllerConfiguration.rs");
include!("ThreadedControllersConfiguration.rs");
