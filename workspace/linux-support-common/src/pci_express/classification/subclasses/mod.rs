// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


include!("Bridge.rs");
include!("CommunicationController.rs");
include!("DisplayController.rs");
include!("DockingStation.rs");
include!("EncryptionController.rs");
include!("GenericSystemPeripheral.rs");
include!("InputDeviceController.rs");
include!("IntelligentController.rs");
include!("MassStorageController.rs");
include!("MemoryController.rs");
include!("MultimediaController.rs");
include!("NetworkController.rs");
include!("Processor.rs");
include!("SatelliteCommunicationsController.rs");
include!("SerialBusController.rs");
include!("SignalProcessingController.rs");
include!("Unassigned.rs");
include!("UnassignedSubClass.rs");
include!("UnclassifiedDevice.rs");
include!("WirelessController.rs");


use serde::Deserialize;
use serde::Serialize;
