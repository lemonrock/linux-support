// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::attributes::*;
use super::route::get_address::*;
use crate::bpf::extended::identifiers::ExtendedBpfProgramIdentifier;
use crate::network_device::*;
use crate::network_device::c::*;


include!("ARPHRD.rs");
include!("ControlNetlinkMessageType.rs");
include!("IF_LINK_MODE.rs");
include!("IF_OPER.rs");
include!("IFA.rs");
include!("ifa_cacheinfo.rs");
include!("IFA_F_.rs");
include!("ifaddrmsg.rs");
include!("ifinfomsg.rs");
include!("IFLA.rs");
include!("IFLA_EVENT.rs");
include!("IFLA_XDP.rs");
include!("NetlinkMessageType.rs");
include!("nlattr.rs");
include!("NLM_F_.rs");
include!("nlmsgerr.rs");
include!("nlmsghdr.rs");
include!("rt_scope.rs");
include!("rtattr.rs");
include!("RTM_.rs");
include!("rtnl_link_ifmap.rs");
include!("rtnl_link_stats64.rs");
include!("XDP_ATTACHED.rs");
include!("XDP_FLAGS_.rs");
