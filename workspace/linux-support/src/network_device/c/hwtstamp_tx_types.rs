// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Strings are in the `ethtool_stringset::ETH_SS_TS_TX_TYPES` string set.
///
/// Used in `hwtstamp_config.tx_type`.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum hwtstamp_tx_types
{
	/// No outgoing packet will need hardware time stamping; should a packet arrive which asks for it, no hardware time stamping will be done.
	///
	/// String set value is `off`.
	HWTSTAMP_TX_OFF = 0,
	
	/// Enables hardware time stamping for outgoing packets; the sender of the packet decides which are to be time stamped by setting `SOF_TIMESTAMPING_TX_SOFTWARE` before sending the packet.
	///
	/// String set value is `on`.
	HWTSTAMP_TX_ON = 1,
	
	/// Enables time stamping for outgoing packets just as `HWTSTAMP_TX_ON` does, but also enables time stamp insertion directly into Sync packets.
	/// In this case, transmitted Sync packets will not received a time stamp via the socket error queue.
	///
	/// String set value is `onestep-sync`.
	HWTSTAMP_TX_ONESTEP_SYNC = 2,
	
	/// Same as `HWTSTAMP_TX_ONESTEP_SYNC`, but also enables time stamp insertion directly into PDelay_Resp packets.
	/// In this case, neither transmitted Sync nor PDelay_Resp packets will receive a time stamp via the socket error queue.
	///
	/// String set value is `onestep-p2p`.
	HWTSTAMP_TX_ONESTEP_P2P = 3,
}

impl hwtstamp_tx_types
{
	const __HWTSTAMP_TX_CNT: usize = 4;
}
