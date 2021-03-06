// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Value of ethtool_rxnfc.data field.

// Layer 3 ot Layer 4 network traffic flow hash options.

pub(crate) const RXH_L2DA: u64 = 1 << 1;

pub(crate) const RXH_VLAN: u64 = 1 << 2;

pub(crate) const RXH_L3_PROTO: u64 = 1 << 3;

pub(crate) const RXH_IP_SRC: u64 = 1 << 4;

pub(crate) const RXH_IP_DST: u64 = 1 << 5;

/// Source port in case of TCP, UDP or SCTP.
pub(crate) const RXH_L4_B_0_1: u64 = 1 << 6;

/// Destination port in case of TCP, UDP or SCTP.
pub(crate) const RXH_L4_B_2_3: u64 = 1 << 7;

pub(crate) const RXH_DISCARD: u64 = 1 << 31;
