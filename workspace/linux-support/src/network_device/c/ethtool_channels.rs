// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(crate) struct ethtool_channels
{
	/// `ETHTOOL_GCHANNELS` (to get channels) or `ETHTOOL_SCHANNELS` (to set channels).
	pub(crate) cmd: u32,
	
	/// Maximum number of receive channel the driver support.
	///
	/// Read only.
	pub(crate) max_rx: u32,
	
	/// Maximum number of transmit channel the driver support.
	///
	/// Read only.
	pub(crate) max_tx: u32,
	
	/// Maximum number of other channel the driver support.
	///
	/// Read only.
	pub(crate) max_other: u32,
	
	/// Maximum number of combined channel the driver support.
	/// Set of queues RX, TX or other.
	///
	/// Read only.
	pub(crate) max_combined: u32,
	
	/// Valid values are in the range 1 to `max_rx`.
	pub(crate) rx_count: Option<NonZeroU32>,
	
	/// Valid values are in the range 1 to `max_tx`.
	pub(crate) tx_count: Option<NonZeroU32>,
	
	/// Valid values are in the range 1 to `max_other`.
	pub(crate) other_count: Option<NonZeroU32>,
	
	/// Valid values are in the range 1 to `max_combined`.
	pub(crate) combined_count: Option<NonZeroU32>,
}
