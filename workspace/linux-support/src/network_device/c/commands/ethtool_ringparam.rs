// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Ring parameters.
///
/// Mini-pending and jumbo-pending are not often supported, eg the very common Intel i40e driver does not support these.
#[repr(C)]
pub(crate) struct ethtool_ringparam
{
	/// `ETHTOOL_GRINGPARAM` or `ETHTOOL_SRINGPARAM`.
	pub(crate) cmd: u32,
	
	/// Maximum supported number of pending entries per	receive ring queue.
	///
	/// Read-only.
	pub(crate) rx_max_pending: Option<QueueDepth>,
	
	/// Maximum supported number of pending entries per receive mini ring queue.
	///
	/// Read-only.
	pub(crate) rx_mini_max_pending: Option<QueueDepth>,
	
	/// Maximum supported number of pending entries per receive jumbo ring queue.
	///
	/// Read-only.
	pub(crate) rx_jumbo_max_pending: Option<QueueDepth>,
	
	/// Maximum supported number of pending entries per transmit ring queue.
	///
	/// Read-only.
	pub(crate) tx_max_pending: Option<QueueDepth>,
	
	/// Current maximum number of pending entries per receive ring queue.
	pub(crate) rx_pending: Option<v>,
	
	/// Current maximum number of pending entries per receive mini ring queue.
	pub(crate) rx_mini_pending: Option<QueueDepth>,
	
	/// Current maximum number of pending entries per receive jumbo ring queue.
	pub(crate) rx_jumbo_pending: Option<QueueDepth>,
	
	/// Current maximum supported number of pending entries per transmit ring queue.
	///
	/// Seems to be different to `/sys/class/net/<network_interface_name>/tx_queue_len`.
	pub(crate) tx_pending: Option<QueueDepth>,
}

impl EthtoolCommand for ethtool_ringparam
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}
