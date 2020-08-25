// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::c::*;
use self::c::commands::*;
use self::coalescing::*;
use self::energy_efficient_ethernet::*;
use self::features::*;
use self::pause::*;
use self::receive_side_scaling::*;
use self::string_sets::*;
use self::tunables::*;
use self::wake_on_lan::WakeOnLanInformation;
use crate::bpf::extended::express_data_path::QueueIdentifier;
use crate::bpf::extended::maps::express_data_path_redirect::QueueDepth;
use crate::file_descriptors::*;
use crate::file_descriptors::network_device::*;
use crate::paths::SysPath;
use crate::user_and_groups::assert_effective_user_id_is_root;
use crate::network_device::eeprom::{PluginModuleEepromBinaryData, ExpansionEepromBinaryData, BinaryData256, BinaryData640};
use crate::diagnostics::UndocumentedError;


/// C.
pub mod c;


/// Coalescing.
pub mod coalescing;


/// Energy Efficient Ethernet (EEE).
pub mod energy_efficient_ethernet;


/// Plug-in EEPROM module.
pub mod eeprom;


/// Features.
pub mod features;


/// Queuing discipline.
///
/// Only used for transmit.
pub mod queuing_discipline;


/// Pause (ethernet Layer 2 flow control).
pub mod pause;


/// Receive side scaling (RSS).
pub mod receive_side_scaling;


/// String sets.
pub mod string_sets;


/// Tunables.
pub mod tunables;


/// Wake-on-LAN.
pub mod wake_on_lan;


include!("BusDeviceAddress.rs");
include!("Channels.rs");
include!("DeviceFeatures.rs");
include!("DriverAndDeviceInformation.rs");
include!("EnergyEfficientEthernetConfiguration.rs");
include!("GlobalNetworkDeviceConfiguration.rs");
include!("GlobalNetworkDeviceConfigurationError.rs");
include!("HardwareAddress.rs");
include!("MaximumTransmissionUnit.rs");
include!("MaximumTransmissionUnitOutRangeError.rs");
include!("NetworkDeviceFirmware.rs");
include!("NetworkDeviceGroup.rs");
include!("NetworkDeviceInputOutputControl.rs");
include!("NetworkDeviceInputOutputControlError.rs");
include!("NetworkDeviceRegisters.rs");
include!("NetworkDeviceTimestampingInformation.rs");
include!("NetworkInterfaceAlias.rs");
include!("NetworkInterfaceIndex.rs");
include!("NetworkInterfaceIndexToNetworkInterfaceNameError.rs");
include!("NetworkInterfaceName.rs");
include!("NetworkInterfaceNameToSomethingError.rs");
include!("PendingQueueDepths.rs");
include!("PhysicalIdentifier.rs");
include!("PhysicalIdentifierFromBytesError.rs");
include!("QueueCount.rs");
include!("TransmissionQueueLengthOutRangeError.rs");
