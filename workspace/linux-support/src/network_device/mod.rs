// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::c::*;
use self::c::commands::*;
use self::coalescing::*;
use self::eeprom::*;
use self::energy_efficient_ethernet::*;
use self::features::*;
use self::link_settings::*;
use self::pause::*;
use self::queues::*;
use self::receive_side_scaling::*;
use self::receive_side_scaling::hash_function_fields::*;
use self::seg6::*;
use self::strategies::*;
use self::string_sets::*;
use self::tunables::*;
use self::wake_on_lan::WakeOnLanInformation;
use crate::bpf::extended::maps::express_data_path_redirect::QueueDepth;
use crate::diagnostics::UndocumentedError;
use crate::file_descriptors::*;
use crate::file_descriptors::network_device::*;
use crate::paths::SysPath;
use crate::user_and_groups::assert_effective_user_id_is_root;
use crate::configuration::Milliseconds;
use crate::file_descriptors::socket::c::in6_addr;
use crate::pci_express::{PciDeviceAddress, PciDevice, PciBusAddress, PciBusDetails};
use crate::cpu::HyperThreads;
use crate::file_descriptors::netlink::route::RouteNetlinkProtocol;
use crate::file_descriptors::netlink::NetlinkSocketFileDescriptor;


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


/// Link settings.
pub mod link_settings;


/// Network flow classifier (`nfc`) for received packets.
///
/// Used in conjunction with driver support for accelerated Receive Flow Steering (aRFS).
/// A driver supports aRFS if it implements the `.ndo_rx_flow_steer` operation.
pub mod network_flow_classifier;


/// Queuing discipline.
///
/// Only used for transmit.
pub mod queuing_discipline;


/// Pause (ethernet Layer 2 flow control).
pub mod pause;


/// Queues.
pub mod queues;


/// Receive side scaling (RSS).
pub mod receive_side_scaling;


/// 'seg6'.
pub mod seg6;


/// Strategies for working with multi-core, multi-queue NICs with RSS on NUMA machines.
pub mod strategies;


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
include!("HardwareAddressType.rs");
include!("InternetProtocolAddressLifetime.rs");
include!("InternetProtocolVersion4AddressResolutionProtocolAnnounce.rs");
include!("InternetProtocolVersion4AddressResolutionProtocolIgnore.rs");
include!("InternetProtocolVersion4Details.rs");
include!("InternetProtocolVersion4DeviceConfiguration.rs");
include!("InternetProtocolVersion4ForceInternetGroupManagementProtocolVersion.rs");
include!("InternetProtocolVersion4MediumIdentifier.rs");
include!("InternetProtocolVersion4ReversePathFilter.rs");
include!("InternetProtocolVersion6AcceptDuplicateAddressDetection.rs");
include!("InternetProtocolVersion6AcceptRouterAdvertisement.rs");
include!("InternetProtocolVersion6Details.rs");
include!("InternetProtocolVersion6DeviceConfiguration.rs");
include!("InternetProtocolVersion6ForceMulticastListenerDiscoverVersion.rs");
include!("InternetProtocolVersion6KeepAddressOnDown.rs");
include!("InternetProtocolVersion6PrivacyExtensions.rs");
include!("MaximumTransmissionUnitPayloadSize.rs");
include!("MaximumTransmissionUnitPayloadSizeOutOfRangeError.rs");
include!("NetNamespaceIdentifier.rs");
include!("NetworkDeviceFirmware.rs");
include!("NetworkDeviceGroup.rs");
include!("NetworkDeviceInputOutputControl.rs");
include!("NetworkDeviceInputOutputControlError.rs");
include!("NetworkDeviceRegisters.rs");
include!("NetworkDeviceTimestampingInformation.rs");
include!("NetworkInterfaceAlias.rs");
include!("NetworkInterfaceAlternativeName.rs");
include!("NetworkInterfaceIndex.rs");
include!("NetworkInterfaceIndexToNetworkInterfaceNameError.rs");
include!("NetworkInterfaceName.rs");
include!("NetworkInterfaceNameToSomethingError.rs");
include!("PendingQueueDepths.rs");
include!("PhysicalIdentifier.rs");
include!("PhysicalIdentifierFromBytesError.rs");
include!("SettableLinkFlags.rs");
include!("TransmissionQueueLengthOutOfRangeError.rs");
include!("VirtualFunctionIndex.rs");
