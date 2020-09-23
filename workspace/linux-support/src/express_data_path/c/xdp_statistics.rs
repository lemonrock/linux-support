// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
#[repr(C)]
pub struct xdp_statistics
{
	/// Packets dropped for reasons other than an invalid receive descriptor.
	pub rx_dropped: u64,
	
	/// Packets dropped due to an invalid receive descriptor.
	pub rx_invalid_descs: u64,
	
	/// Packets dropped due to an invalid transmit descriptor.
	pub tx_invalid_descs: u64,
}
