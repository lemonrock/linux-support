// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics.
///
/// Documentation is as per Linux header `if_link.h`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(C)]
pub struct rtnl_link_stats64
{
	/// total packets received.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_packets`.
	pub rx_packets: u64,
	
	/// total packets transmitted.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_packets`.
	pub tx_packets: u64,
	
	/// total bytes received.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_packets`.
	pub rx_bytes: u64,
	
	/// total bytes transmitted.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_bytes`.
	pub tx_bytes: u64,
	
	/// bad packets received.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_errors`.
	pub rx_errors: u64,
	
	/// packet transmit problems.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_errors`.
	pub tx_errors: u64,
	
	/// no space in Linux buffers.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_dropped`.
	pub rx_dropped: u64,
	
	/// no space available in Linux.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_dropped`.
	pub tx_dropped: u64,
	
	/// multicast packets received.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/multicast`.
	pub multicast: u64,
	
	/// Collisions.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/collisions`.
	pub collisions: u64,
	
	/// Detailed receive error: receive length errors.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_length_errors`.
	pub rx_length_errors: u64,
	
	/// Detailed receive error: receiver ring buff overflow.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_over_errors`.
	pub rx_over_errors: u64,
	
	/// Detailed receive error: recved pkt with crc error.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_crc_errors`.
	pub rx_crc_errors: u64,
	
	/// Detailed receive error: recveived frame alignment error.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_frame_errors`.
	pub rx_frame_errors: u64,
	
	/// Detailed receive error: receiver fifo overrun.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_fifo_errors`.
	pub rx_fifo_errors: u64,
	
	/// Detailed receive error: receiver missed packet.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_missed_errors`.
	pub rx_missed_errors: u64,

	/// Detailed transmission error.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_aborted_errors`.
	pub tx_aborted_errors: u64,
	
	/// Detailed transmission error.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_carrier_errors`.
	pub tx_carrier_errors: u64,
	
	/// Detailed transmission error.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_fifo_errors`.
	pub tx_fifo_errors: u64,
	
	/// Detailed transmission error.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_heartbeat_errors`.
	pub tx_heartbeat_errors: u64,
	
	/// Detailed transmission error.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_window_errors`.
	pub tx_window_errors: u64,
	
	/// For cslip, etc.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_compressed`.
	pub rx_compressed: u64,
	
	/// For cslip, etc.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/tx_compressed`.
	pub tx_compressed: u64,

	/// dropped, no handler found.
	///
	/// Also available at `/sys/class/net/<network_interface_name>/statistics/rx_nohandler`.
	pub rx_nohandler: u64,
}

