// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::classification::*;
use self::classification::subclasses::NetworkController;
use self::configuration_space::MemoryMappedConfigurationSpace;
use self::link::*;
use self::resources::Resources;
use crate::cpu::*;
use crate::network_device::*;
use crate::linux_kernel_modules::*;
use crate::memory::huge_pages::PageSizeOrHugePageSizeSettings;
use crate::memory::mapping::*;
use crate::memory::numa::NumaNode;
use crate::paths::*;
use crate::user_and_groups::assert_effective_user_id_is_root;
use crate::interrupt_request::InterruptRequest;


/// Classifications.
pub mod classification;


/// Configuration Space.
pub mod configuration_space;


/// Definitions.
pub mod definitions;


/// Link.
pub mod link;


pub(crate) mod msi_x_interrupt_request_naming_strategy;


/// Registers.
pub mod registers;


/// Resources.
pub mod resources;


include!("BusNumber.rs");
include!("LinuxPciUserspaceKernelDriverModule.rs");
include!("MsiInterruptMode.rs");
include!("PciBridgeDeviceDetails.rs");
include!("PciBusAddress.rs");
include!("PciBus.rs");
include!("PciBusDetails.rs");
include!("PciDevice.rs");
include!("PciDeviceAddress.rs");
include!("PciDeviceAddressFromNetworkInterfaceNameError.rs");
include!("PciDeviceAddressStringParseError.rs");
include!("PciDeviceDetails.rs");
include!("PciDevicePhysicalOrVirtualFunction.rs");
include!("PrimaryPciBusAddress.rs");
include!("ResourceFile.rs");
