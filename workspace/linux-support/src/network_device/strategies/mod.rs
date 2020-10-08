// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::coalescing_preferences::*;
use self::coalescing_strategies::*;
use self::ConfigureDriverProfileError::*;
use self::DriverProfileError::*;
use self::FeatureGroupChoice::*;
use self::FindDriverProfileError::*;
use self::NETIF_F::*;
use super::*;
use crate::cpu::HyperThreads;
use crate::interrupt_request::*;
use crate::linux_kernel_version::LinuxKernelVersion;
use crate::network_device::tunables::TunableChoice::ReceiveCopyBreakPoint;
use crate::paths::ProcPath;
use crate::pci_express::*;
use crate::pci_express::classification::*;
use crate::pci_express::msi_x_interrupt_request_naming_strategy::*;
use crate::file_descriptors::netlink::NetlinkSocketFileDescriptor;
use crate::file_descriptors::netlink::route::RouteNetlinkProtocol;
use crate::file_descriptors::socket::SocketCreationOrBindError;


/// Preferences.
pub mod coalescing_preferences;


pub(crate) mod coalescing_strategies;


include!("AllocateInterruptRequests.rs");
include!("ConfigureDriverProfileError.rs");
include!("DevicePreferences.rs");
include!("DriverProfile.rs");
include!("DriverProfileChoice.rs");
include!("DriverProfileError.rs");
include!("DriverProfileKey.rs");
include!("DriverProfilesMap.rs");
include!("FindDriverProfileError.rs");
include!("FairWeightQueueStrategy.rs");
include!("InterruptRequestAffinities.rs");
include!("MsiXInterruptRequestNamingStrategy.rs");
include!("NetworkDeviceInputOutputControlDriverProfile.rs");
include!("SetToSpecificValueOrMaximize.rs");
