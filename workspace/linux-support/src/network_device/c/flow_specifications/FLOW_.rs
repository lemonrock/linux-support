// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Flag to enable additional fields in `ethtool_rx_flow_spec`.
pub(crate) const FLOW_EXT: u32 = 0x80000000;

pub(crate) const FLOW_MAC_EXT: u32 = 0x40000000;

/// Flag to enable RSS spreading of traffic matching rule (nfc only).
pub(crate) const FLOW_RSS: u32 = 0x20000000;
