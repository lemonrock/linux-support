// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::network_device::*;
use crate::network_device::c::*;
use crate::network_device::queues::*;
use crate::bpf::extended::identifiers::MultipleProgramIdentifiers;
use crate::network_device::queuing_discipline::QueuingDisciplineAlgorithm;
use crate::configuration::Milliseconds;
use crate::network_device::seg6::HmacPolicyForSrEnabledPackets;


include!("GetLinkMessageData.rs");
include!("GetLinkProcessingMessageState.rs");
include!("InternetProtocolVersion4DeviceConfigurationGetLinkProcessMessageState.rs");
include!("InternetProtocolVersion6DetailsGetLinkProcessMessageState.rs");
include!("InternetProtocolVersion6DeviceConfigurationGetLinkProcessMessageState.rs");
