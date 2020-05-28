// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics.
///
/// Documentation is as per Linux header `if_link.h`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct rtnl_link_stats64
{
	/// total packets received.
	pub rx_packets: u64,
	
	/// total packets transmitted.
	pub tx_packets: u64,
	
	/// total bytes received .
	pub rx_bytes: u64,
	
	/// total bytes transmitted.
	pub tx_bytes: u64,
	
	/// bad packets received.
	pub rx_errors: u64,
	
	/// packet transmit problems.
	pub tx_errors: u64,
	
	/// no space in Linux buffers.
	pub rx_dropped: u64,
	
	/// no space available in Linux.
	pub tx_dropped: u64,
	
	/// multicast packets received.
	pub multicast: u64,
	
	/// Collisions.
	pub collisions: u64,
	
	/// Detailed receive error: receive length errors.
	pub rx_length_errors: u64,
	
	/// Detailed receive error: receiver ring buff overflow.
	pub rx_over_errors: u64,
	
	/// Detailed receive error: recved pkt with crc error.
	pub rx_crc_errors: u64,
	
	/// Detailed receive error: recv'd frame alignment error .
	pub rx_frame_errors: u64,
	
	/// Detailed receive error: recv'r fifo overrun.
	pub rx_fifo_errors: u64,
	
	/// Detailed receive error: receiver missed packet.
	pub rx_missed_errors: u64,

	/// Detailed transmission error.
	pub tx_aborted_errors: u64,
	
	/// Detailed transmission error.
	pub tx_carrier_errors: u64,
	
	/// Detailed transmission error.
	pub tx_fifo_errors: u64,
	
	/// Detailed transmission error.
	pub tx_heartbeat_errors: u64,
	
	/// Detailed transmission error.
	pub tx_window_errors: u64,
	
	/// For cslip, etc.
	pub rx_compressed: u64,
	
	/// For cslip, etc.
	pub tx_compressed: u64,

	/// dropped, no handler found.
	pub rx_nohandler: u64,
}

