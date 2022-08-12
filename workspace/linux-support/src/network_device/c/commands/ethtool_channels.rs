// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A channel is an IRQ and the set of queues (or, in XDP world, the queue) that can trigger that IRQ.
#[repr(C)]
pub(crate) struct ethtool_channels
{
	/// `ETHTOOL_GCHANNELS` (to get channels) or `ETHTOOL_SCHANNELS` (to set channels).
	pub(crate) cmd: u32,
	
	/// Maximum number of receive channels the driver supports.
	///
	/// These have only receive queues.
	///
	/// Read only.
	pub(crate) max_rx: Option<QueueCount>,
	
	/// Maximum number of transmit channels the driver supports.
	///
	/// These have only transmit queues.
	///
	/// Read only.
	pub(crate) max_tx: Option<QueueCount>,
	
	/// Maximum number of other channels the driver supports.
	///
	/// Used for other purposes such as link interrupts or PCI SR-IOV co-ordination.
	///
	/// Read only.
	pub(crate) max_other: Option<QueueCount>,
	
	/// Multi-purpose channels.
	///
	/// These have simultaneously receive and transmit queues.
	///
	/// Read only.
	pub(crate) max_combined: Option<QueueCount>,
	
	/// Valid values are in the range 1 to `max_rx`.
	pub(crate) rx_count: Option<QueueCount>,
	
	/// Valid values are in the range 1 to `max_tx`.
	pub(crate) tx_count: Option<QueueCount>,
	
	/// Valid values are in the range 1 to `max_other`.
	pub(crate) other_count: Option<QueueCount>,
	
	/// Valid values are in the range 1 to `max_combined`.
	pub(crate) combined_count: Option<QueueCount>,
}

impl EthtoolCommand for ethtool_channels
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}
