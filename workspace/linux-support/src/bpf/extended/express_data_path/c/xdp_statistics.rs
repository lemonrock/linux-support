// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub(super) struct xdp_statistics
{
	/// Packets dropped for reasons other than invalid receive descriptor.
	rx_dropped: u64,
	
	/// Packets dropped due to invalid receive descriptor.
	rx_invalid_descs: u64,
	
	/// Packets dropped due to invalid transmit descriptor.
	tx_invalid_descs: u64,
}
