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
#[derive(Debug, Copy, Clone)]
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

	/// Maximum number of packets to be sent before a transmit interrupt, when the packet rate is below `pkt_rate_low`.
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

impl ethtool_coalesce
{
	#[inline(always)]
	pub(crate) fn get() -> Self
	{
		Self
		{
			cmd: ETHTOOL_GCOALESCE,
			rx_coalesce_usecs: None,
			rx_max_coalesced_frames: None,
			rx_coalesce_usecs_irq: None,
			rx_max_coalesced_frames_irq: None,
			tx_coalesce_usecs: None,
			tx_max_coalesced_frames: None,
			tx_coalesce_usecs_irq: None,
			tx_max_coalesced_frames_irq: None,
			stats_block_coalesce_usecs: None,
			use_adaptive_rx_coalesce: 0,
			use_adaptive_tx_coalesce: 0,
			pkt_rate_low: None,
			rx_coalesce_usecs_low: None,
			rx_max_coalesced_frames_low: None,
			tx_coalesce_usecs_low: None,
			tx_max_coalesced_frames_low: None,
			pkt_rate_high: None,
			rx_coalesce_usecs_high: None,
			rx_max_coalesced_frames_high: None,
			tx_coalesce_usecs_high: None,
			tx_max_coalesced_frames_high: None,
			rate_sample_interval: None
		}
	}
	
	#[inline(always)]
	pub(crate) fn as_coalesce_configuration(&self) -> Result<CoalesceConfiguration, AdaptiveCoalescingError>
	{
		use self::AdaptiveCoalescingConfiguration::*;
		
		Ok
		(
			CoalesceConfiguration
			{
				adaptive_coalescing: match (self.use_adaptive_rx_coalesce, self.use_adaptive_tx_coalesce)
				{
					(0, 0) => None,
					(1, 0) => Some(TransmitOnly(AdaptiveCoalescingRateSampling::new_from_ethtool(self))),
					(0, 1) => Some(ReceiveOnly(AdaptiveCoalescingRateSampling::new_from_ethtool(self))),
					(1, 1) => Some(ReceiveAndTransmit(AdaptiveCoalescingRateSampling::new_from_ethtool(self))),
					(receive @ _, transmit @ _) => return Err(AdaptiveCoalescingError(format!("Invalid combination ({}, {}) for adaptive coalescing", receive, transmit))),
				},
				
				receive_transmit: ReceiveTransmitCoalescing
				{
					receive: CoalescePair
					{
						microseconds: self.rx_coalesce_usecs,
						maximum_frames: self.rx_max_coalesced_frames,
					},
					
					transmit: CoalescePair
					{
						microseconds: self.tx_coalesce_usecs,
						maximum_frames: self.tx_max_coalesced_frames,
					},
				},
				
				receive_transmit_whilst_irq_is_being_serviced_by_the_host: ReceiveTransmitCoalescing
				{
					receive: CoalescePair
					{
						microseconds: self.rx_coalesce_usecs_irq,
						maximum_frames: self.rx_max_coalesced_frames_irq,
					},
					
					transmit: CoalescePair
					{
						microseconds: self.tx_coalesce_usecs_irq,
						maximum_frames: self.tx_max_coalesced_frames_irq,
					},
				},
				
				low_packet_rate_packets_per_second_threshold: self.pkt_rate_low,
				
				receive_transmit_at_low_packet_rate: ReceiveTransmitCoalescing
				{
					receive: CoalescePair
					{
						microseconds: self.rx_coalesce_usecs_low,
						maximum_frames: self.rx_max_coalesced_frames_low,
					},
					
					transmit: CoalescePair
					{
						microseconds: self.tx_coalesce_usecs_low,
						maximum_frames: self.tx_max_coalesced_frames_low,
					},
				},
				
				high_packet_rate_packets_per_second_threshold: self.pkt_rate_high,
				
				receive_transmit_at_high_packet_rate: ReceiveTransmitCoalescing
				{
					receive: CoalescePair
					{
						microseconds: self.rx_coalesce_usecs_high,
						maximum_frames: self.rx_max_coalesced_frames_high,
					},
					
					transmit: CoalescePair
					{
						microseconds: self.tx_coalesce_usecs_high,
						maximum_frames: self.tx_max_coalesced_frames_high,
					},
				},
				
				statistics_block_coalesce_microseconds: self.stats_block_coalesce_usecs,
			}
		)
	}
}
