// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_coalesce
{
	pub cmd: u32,
	pub rx_coalesce_usecs: u32,
	pub rx_max_coalesced_frames: u32,
	pub rx_coalesce_usecs_irq: u32,
	pub rx_max_coalesced_frames_irq: u32,
	pub tx_coalesce_usecs: u32,
	pub tx_max_coalesced_frames: u32,
	pub tx_coalesce_usecs_irq: u32,
	pub tx_max_coalesced_frames_irq: u32,
	pub stats_block_coalesce_usecs: u32,
	pub use_adaptive_rx_coalesce: u32,
	pub use_adaptive_tx_coalesce: u32,
	pub pkt_rate_low: u32,
	pub rx_coalesce_usecs_low: u32,
	pub rx_max_coalesced_frames_low: u32,
	pub tx_coalesce_usecs_low: u32,
	pub tx_max_coalesced_frames_low: u32,
	pub pkt_rate_high: u32,
	pub rx_coalesce_usecs_high: u32,
	pub rx_max_coalesced_frames_high: u32,
	pub tx_coalesce_usecs_high: u32,
	pub tx_max_coalesced_frames_high: u32,
	pub rate_sample_interval: u32,
}

impl Default for ethtool_coalesce
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
