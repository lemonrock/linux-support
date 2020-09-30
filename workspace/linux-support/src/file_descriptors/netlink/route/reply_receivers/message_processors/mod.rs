// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::bpf::extended::identifiers::ExtendedBpfProgramIdentifier;
use crate::network_device::HardwareAddress;
use crate::network_device::NetworkInterfaceAlternativeName;
use crate::network_device::InternetProtocolVersion4DeviceConfiguration;
use crate::network_device::InternetProtocolVersion6DeviceConfiguration;
use crate::network_device::InternetProtocolVersion6Details;
use crate::network_device::InternetProtocolVersion4Details;
use crate::network_device::c::in6_addr_gen_mode;
use crate::express_data_path::c::xdp_diag_info;
use crate::express_data_path::c::XDP_DIAG;
use crate::express_data_path::c::xdp_diag_msg;
use crate::express_data_path::c::xdp_diag_req;
use crate::express_data_path::c::xdp_diag_umem;
use crate::express_data_path::c::xdp_diag_stats;


include!("GetExpressDataPathDiagnosticsMessageProcessor.rs");
include!("GetInternetProtocolVersion4AddressMessageProcessor.rs");
include!("GetInternetProtocolVersion6AddressMessageProcessor.rs");
include!("GetInternetProtocolVersion6AnycastAddressMessageProcessor.rs");
include!("GetInternetProtocolVersion6MulticastAddressMessageProcessor.rs");
include!("GetLinkMessageProcessor.rs");
include!("MessageProcessor.rs");
include!("set_address_field.rs");
include!("set_field_error.rs");

