// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use crate::file_descriptors::socket::c::*;


include!("cisco_proto.rs");
include!("ethtool_channels.rs");
include!("fr_proto.rs");
include!("fr_proto_pvc.rs");
include!("fr_proto_pvc_info.rs");
include!("if_settings.rs");
include!("if_settings_ifsu.rs");
include!("ifmap.rs");
include!("ifreq.rs");
include!("ifreq_ifrn.rs");
include!("ifreq_ifru.rs");
include!("raw_hdlc_proto.rs");
include!("sync_serial_settings.rs");
include!("te1_settings.rs");
include!("x25_hdlc_proto.rs");
