// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use super::*;
use crate::file_descriptors::*;
use crate::file_descriptors::network_device::*;
use crate::network_device::coalescing::CoalesceConfiguration;
use crate::network_device::features::FeatureGroupChoice;
use crate::network_device::tunables::Tunable;


/// C.
pub mod c;


/// Coalescing.
pub mod coalescing;


/// Features.
pub mod features;


/// Tunables.
pub mod tunables;


include!("BusDeviceAddress.rs");
include!("Channels.rs");
include!("GlobalNetworkDeviceConfiguration.rs");
include!("GlobalNetworkDeviceConfigurationError.rs");
include!("HardwareAddress.rs");
include!("MaximumTransmissionUnit.rs");
include!("NetworkDeviceInputOutputControl.rs");
include!("NetworkDeviceInputOutputControlError.rs");
include!("NetworkInterfaceAlias.rs");
include!("NetworkInterfaceIndex.rs");
include!("NetworkInterfaceIndexToNetworkInterfaceNameError.rs");
include!("NetworkInterfaceName.rs");
include!("NetworkInterfaceNameToSomethingError.rs");
include!("PendingQueueDepths.rs");
include!("PhysicalIdentifier.rs");
include!("PhysicalIdentifierFromBytesError.rs");
include!("QueueCount.rs");
