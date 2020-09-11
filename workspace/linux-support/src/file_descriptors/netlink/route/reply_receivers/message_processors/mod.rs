// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::bpf::extended::identifiers::ExtendedBpfProgramIdentifier;
use crate::network_device::HardwareAddress;
use crate::bpf::extended::express_data_path::c::xdp_diag_info;
use crate::bpf::extended::express_data_path::c::XDP_DIAG;
use crate::bpf::extended::express_data_path::c::xdp_diag_msg;
use crate::bpf::extended::express_data_path::c::xdp_diag_req;
use crate::bpf::extended::express_data_path::c::xdp_diag_umem;
use crate::bpf::extended::express_data_path::c::xdp_diag_stats;


include!("GetAddressMessageProcessor.rs");include!("GetExpressDataPathDiagnosticsMessageProcessor.rs");
include!("GetLinkMessageProcessor.rs");
include!("MessageProcessor.rs");
