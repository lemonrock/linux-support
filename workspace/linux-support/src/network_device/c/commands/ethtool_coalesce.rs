// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Coalescing parameters for IRQs and statistics updates.
///
/// Each pair of (usecs, max_frames) fields specifies that interrupts should be coalesced until `(usecs > 0 && time_since_first_completion >= usecs) || (max_frames > 0 && completed_frames >= max_frames)`.
///
/// It is illegal to set both usecs and max_frames to zero as this would cause interrupts to never be generated.
/// To disable coalescing, set `usecs = 0` and `max_frames = 1`.
///
/// Some implementations ignore the value of `max_frames` and use the condition `time_since_first_completion >= usecs`.
/// This is deprecated.  Drivers for hardware that does not support counting completions should validate that `max_frames == !rx_usecs`.
///
/// Adaptive receive coalescing and transmit coalescing is an algorithm implemented by some drivers to improve latency under low packet rates and improve throughput under high packet rates.
/// Some drivers only implement one of receive or transmit adaptive coalescing.
/// Anything not implemented by the driver causes these values to be silently ignored.
///
/// When the packet rate is below `pkt_rate_high` but above `pkt_rate_low` (both measured in packets per second) the normal `{rx,tx}_*` coalescing parameters are used.
#[repr(C)]
pub(crate) struct ethtool_coalesce
{
	/// Either `ETHTOOL_GCOALESCE` or `ETHTOOL_SCOALESCE`.
	pub(crate) cmd: u32,

	/// How many microseconds to delay a receive interrupt after a packet arrives.
	///
	/// Driver must support `ETHTOOL_COALESCE_RX_USECS` if this is `Some`.
	pub(crate) rx_coalesce_usecs: Option<NonZeroU32>,

	/// Maximum number of packets to receive before a receive interrupt.
	///
	/// Driver must support `ETHTOOL_COALESCE_RX_MAX_FRAMES` if this is `Some`.
	pub(crate) rx_max_coalesced_frames: Option<NonZeroU32>,
	
	/// Same as `rx_coalesce_usecs`, except that this value applies while an IRQ is being serviced by the host.
	///
	/// Driver must support `ETHTOOL_COALESCE_RX_USECS_IRQ` if this is `Some`.
	pub(crate) rx_coalesce_usecs_irq: Option<NonZeroU32>,
	
	/// Same as `rx_max_coalesced_frames`, except that this value applies while an IRQ is being serviced by the host.
	///
	/// Driver must support `ETHTOOL_COALESCE_RX_MAX_FRAMES_IRQ` if this is `Some`.
	pub(crate) rx_max_coalesced_frames_irq: Option<NonZeroU32>,
	
	/// How many microseconds to delay a transmit interrupt after a packet is sent.
	///
	/// Driver must support `ETHTOOL_COALESCE_TX_USECS` if this is `Some`.
	pub(crate) tx_coalesce_usecs: Option<NonZeroU32>,

	/// Maximum number of packets to be sent before a transmit interrupt.
	///
	/// Driver must support `ETHTOOL_COALESCE_TX_MAX_FRAMES` if this is `Some`.
	pub(crate) tx_max_coalesced_frames: Option<NonZeroU32>,

	/// Same as `tx_coalesce_usecs`, except that this value applies while an IRQ is being serviced by the host.
	///
	/// Driver must support `ETHTOOL_COALESCE_TX_USECS_IRQ` if this is `Some`.
	pub(crate) tx_coalesce_usecs_irq: Option<NonZeroU32>,
	
	/// Same as `tx_max_coalesced_frames`, except that this value applies while an IRQ is being serviced by the host.
	///
	/// Driver must support `ETHTOOL_COALESCE_TX_MAX_FRAMES_IRQ` if this is `Some`.
	pub(crate) tx_max_coalesced_frames_irq: Option<NonZeroU32>,

	/// How many microseconds to delay in-memory statistics block updates.
	///
	/// Some drivers do not have an	in-memory statistic block, and in such cases this value is ignored.
	///
	/// Driver must support `ETHTOOL_COALESCE_STATS_BLOCK_USECS` if this is `Some`.
	pub(crate) stats_block_coalesce_usecs: Option<NonZeroU32>,

	/// Enable adaptive receive coalescing (boolean-like).
	///
	/// Driver must support `ETHTOOL_COALESCE_USE_ADAPTIVE_RX` if this is `1`.
	pub(crate) use_adaptive_rx_coalesce: u32,

	/// Enable adaptive transmit coalescing (boolean-like).
	///
	/// Driver must support `ETHTOOL_COALESCE_USE_ADAPTIVE_TX` if this is `1`.
	pub(crate) use_adaptive_tx_coalesce: u32,

	/// Threshold for low packet rate (packets per second).
	///
	/// Driver must support `ETHTOOL_COALESCE_PKT_RATE_LOW` if this is `Some`.
	pub(crate) pkt_rate_low: Option<NonZeroU32>,

	/// How many microseconds to delay a receive interrupt after a packet arrives, when the packet rate is below `pkt_rate_low`.
	///
	/// Driver must support `ETHTOOL_COALESCE_RX_USECS_LOW` if this is `Some`.
	pub(crate) rx_coalesce_usecs_low: Option<NonZeroU32>,

	/// Maximum number of packets to be received before a receive interrupt, when the packet rate is below `pkt_rate_low`.
	///
	/// Driver must support `ETHTOOL_COALESCE_RX_MAX_FRAMES_LOW` if this is `Some`.
	pub(crate) rx_max_coalesced_frames_low: Option<NonZeroU32>,

	/// How many microseconds to delay a transmit interrupt after a packet is sent, when the packet rate is below `pkt_rate_low`.
	///
	/// Driver must support `ETHTOOL_COALESCE_TX_USECS_LOW` if this is `Some`.
	pub(crate) tx_coalesce_usecs_low: Option<NonZeroU32>,

	/// Maximum nuumber of packets to be sent before a transmit interrupt, when the packet rate is below `pkt_rate_low`.
	///
	/// Driver must support `ETHTOOL_COALESCE_TX_MAX_FRAMES_LOW` if this is `Some`.
	pub(crate) tx_max_coalesced_frames_low: Option<NonZeroU32>,

	/// Threshold for high packet rate (packets per second).
	///
	/// Driver must support `ETHTOOL_COALESCE_PKT_RATE_HIGH` if this is `Some`.
	pub(crate) pkt_rate_high: Option<NonZeroU32>,

	/// How many microseconds to delay a receive interrupt after a packet arrives, when the packet rate is above `pkt_rate_high`.
	///
	/// Driver must support `ETHTOOL_COALESCE_RX_USECS_HIGH` if this is `Some`.
	pub(crate) rx_coalesce_usecs_high: Option<NonZeroU32>,

	/// Maximum number of packets to be received before a receive interrupt, when the packet rate is above `pkt_rate_high`.
	///
	/// Driver must support `ETHTOOL_COALESCE_RX_MAX_FRAMES_HIGH` if this is `Some`.
	pub(crate) rx_max_coalesced_frames_high: Option<NonZeroU32>,

	/// How many microseconds to delay a transmit interrupt after a packet is sent, when the packet rate is above `pkt_rate_high`.
	///
	/// Driver must support `ETHTOOL_COALESCE_TX_USECS_HIGH` if this is `Some`.
	pub(crate) tx_coalesce_usecs_high: Option<NonZeroU32>,

	/// Maximum number of packets to be sent before a transmit interrupt, when the packet rate is above `pkt_rate_high`.
	///
	/// Driver must support `ETHTOOL_COALESCE_TX_MAX_FRAMES_HIGH` if this is `Some`.
	pub(crate) tx_max_coalesced_frames_high: Option<NonZeroU32>,

	/// How often to do adaptive coalescing packet rate sampling, measured in seconds.
	///
	/// This value must not be zero.
	///
	/// Driver must support `ETHTOOL_COALESCE_RATE_SAMPLE_INTERVAL` if this is `Some`.
	pub(crate) rate_sample_interval: Option<NonZeroU32>,
}

impl EthtoolCommand for ethtool_coalesce
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}
