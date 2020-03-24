// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::bridge_programming_interfaces::*;
use self::communication_controller_programming_interfaces::*;
use self::display_controller_programming_interfaces::*;
use self::generic_system_peripheral_programming_interfaces::*;
use self::input_device_controller_programming_interfaces::*;
use self::intelligent_input_output_controller_programming_interfaces::*;
use self::mass_storage_controller_programming_interfaces::*;
use self::multimedia_controller_programming_interfaces::*;
use self::serial_bus_controller_programming_interfaces::*;
use self::wireless_controller_programming_interfaces::*;
use likely::likely;
use serde::Deserialize;
use serde::Serialize;


include!("programming_interface.rs");
include!("zero_programming_interface.rs");


include!("Bridge.rs");
include!("CommunicationController.rs");
include!("DisplayController.rs");
include!("DockingStation.rs");
include!("EncryptionController.rs");
include!("GenericSystemPeripheral.rs");
include!("InputDeviceController.rs");
include!("IntelligentInputOutputController.rs");
include!("MassStorageController.rs");
include!("MemoryController.rs");
include!("MultimediaController.rs");
include!("NetworkController.rs");
include!("NonEssentialInstrumentation.rs");
include!("ProcessingAccelerators.rs");
include!("Processor.rs");
include!("SatelliteCommunicationsController.rs");
include!("SerialBusController.rs");
include!("SignalProcessingController.rs");
include!("Unassigned.rs");
include!("Legacy.rs");
include!("WirelessController.rs");


/// Bridge Programming Interfaces.
pub mod bridge_programming_interfaces;

/// Communication Controller Programming Interfaces.
pub mod communication_controller_programming_interfaces;

/// Display Controller Programming Interfaces.
pub mod display_controller_programming_interfaces;

/// Generic System Peripheral Programming Interfaces.
pub mod generic_system_peripheral_programming_interfaces;

/// Input Device Controller Programming Interfaces.
pub mod input_device_controller_programming_interfaces;

/// Intelligent Input Output (I²0) Controller Programming Interfaces.
pub mod intelligent_input_output_controller_programming_interfaces;

/// Mass Storage Controller Programming Interfaces.
pub mod mass_storage_controller_programming_interfaces;

/// Multimedia Controller Programming Interfaces.
pub mod multimedia_controller_programming_interfaces;

/// Serial Bus Controller Programming Interfaces.
pub mod serial_bus_controller_programming_interfaces;

/// Wireless Controller Programming Interfaces.
pub mod wireless_controller_programming_interfaces;
