// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Coalesce configuration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoalesceConfiguration
{
	/// Adaptive coalescing.
	///
	/// If `Some` then the driver must support either `ETHTOOL_COALESCE_USE_ADAPTIVE_RX` or `ETHTOOL_COALESCE_USE_ADAPTIVE_TX` or both.
	pub adaptive_coalescing: Option<AdaptiveCoalescingConfiguration>,
	
	/// Normal settings.
	///
	/// Driver must support:-
	///
	/// * `ETHTOOL_COALESCE_RX_USECS` if `ReceiveTransmitCoalescing.Receive.microseconds` is `Some`.
	/// * `ETHTOOL_COALESCE_RX_MAX_FRAMES` to change `ReceiveTransmitCoalescing.Receive.maximum_frames` is `Some`.
	/// * `ETHTOOL_COALESCE_TX_USECS` if `ReceiveTransmitCoalescing.Receive.microseconds` is `Some`.
	/// * `ETHTOOL_COALESCE_TX_MAX_FRAMES` to change `ReceiveTransmitCoalescing.Receive.maximum_frames` is `Some`.
	pub receive_transmit: ReceiveTransmitCoalescing,
	
	/// Settings to used if an IRQ (interrupt request) is being serviced by the host.
	///
	/// Driver must support:-
	///
	/// * `ETHTOOL_COALESCE_RX_USECS_IRQ` if `ReceiveTransmitCoalescing.Receive.microseconds` is `Some`.
	/// * `ETHTOOL_COALESCE_RX_MAX_FRAMES_IRQ` to change `ReceiveTransmitCoalescing.Receive.maximum_frames` is `Some`.
	/// * `ETHTOOL_COALESCE_TX_USECS_IRQ` if `ReceiveTransmitCoalescing.Receive.microseconds` is `Some`.
	/// * `ETHTOOL_COALESCE_TX_MAX_FRAMES_IRQ` to change `ReceiveTransmitCoalescing.Receive.maximum_frames` is `Some`.
	pub receive_transmit_whilst_irq_is_being_serviced_by_the_host: ReceiveTransmitCoalescing,
	
	/// Threshold.
	///
	/// Driver must support `ETHTOOL_COALESCE_PKT_RATE_LOW` if is `Some`.
	pub low_packet_rate_packets_per_second_threshold: Option<NonZeroU32>,
	
	/// Settings to apply if the `low_packet_rate_packets_per_second_threshold` is met.
	///
	/// Driver must support:-
	///
	/// * `ETHTOOL_COALESCE_RX_USECS_LOW` if `ReceiveTransmitCoalescing.Receive.microseconds` is `Some`.
	/// * `ETHTOOL_COALESCE_RX_MAX_FRAMES_LOW` to change `ReceiveTransmitCoalescing.Receive.maximum_frames` is `Some`.
	/// * `ETHTOOL_COALESCE_TX_USECS_LOW` if `ReceiveTransmitCoalescing.Receive.microseconds` is `Some`.
	/// * `ETHTOOL_COALESCE_TX_MAX_FRAMES_LOW` to change `ReceiveTransmitCoalescing.Receive.maximum_frames` is `Some`.
	pub receive_transmit_at_low_packet_rate: ReceiveTransmitCoalescing,
	
	/// Threshold.
	///
	/// Driver must support `ETHTOOL_COALESCE_PKT_RATE_HIGH` if is `Some`.
	pub high_packet_rate_packets_per_second_threshold: Option<NonZeroU32>,
	
	/// Settings to apply if the `high_packet_rate_packets_per_second_threshold` is met.
	///
	/// Driver must support:-
	///
	/// * `ETHTOOL_COALESCE_RX_USECS_HIGH` if `ReceiveTransmitCoalescing.Receive.microseconds` is `Some`.
	/// * `ETHTOOL_COALESCE_RX_MAX_FRAMES_HIGH` to change `ReceiveTransmitCoalescing.Receive.maximum_frames` is `Some`.
	/// * `ETHTOOL_COALESCE_TX_USECS_HIGH` if `ReceiveTransmitCoalescing.Receive.microseconds` is `Some`.
	/// * `ETHTOOL_COALESCE_TX_MAX_FRAMES_HIGH` to change `ReceiveTransmitCoalescing.Receive.maximum_frames` is `Some`.
	pub receive_transmit_at_high_packet_rate: ReceiveTransmitCoalescing,
	
	/// How many microseconds to delay in-memory statistics block updates.
	///
	/// Some drivers do not have an	in-memory statistic block, and in such cases this value is ignored.
	///
	/// This value must not be zero.
	///
	/// Driver must support `ETHTOOL_COALESCE_STATS_BLOCK_USECS`.
	pub statistics_block_coalesce_microseconds: Option<NonZeroU32>,
}

impl CoalesceConfiguration
{
	#[inline(always)]
	pub(crate) fn as_ethtool_coalesce(&self) -> ethtool_coalesce
	{
		let (rate_sample_interval, use_adaptive_rx_coalesce, use_adaptive_tx_coalesce) = AdaptiveCoalescingConfiguration::to_values(&self.adaptive_coalescing);;
		
		let (rx_coalesce_usecs, rx_max_coalesced_frames, tx_coalesce_usecs, tx_max_coalesced_frames) = self.receive_transmit.destructure();
		
		let (rx_coalesce_usecs_irq, rx_max_coalesced_frames_irq, tx_coalesce_usecs_irq, tx_max_coalesced_frames_irq) = self.receive_transmit_whilst_irq_is_being_serviced_by_the_host.destructure();
		
		let (rx_coalesce_usecs_low, rx_max_coalesced_frames_low, tx_coalesce_usecs_low, tx_max_coalesced_frames_low) = self.receive_transmit_at_low_packet_rate.destructure();
		
		let (rx_coalesce_usecs_high, rx_max_coalesced_frames_high, tx_coalesce_usecs_high, tx_max_coalesced_frames_high) = self.receive_transmit_at_high_packet_rate.destructure();
		
		ethtool_coalesce
		{
			cmd: ETHTOOL_SCOALESCE,
			rx_coalesce_usecs,
			rx_max_coalesced_frames,
			rx_coalesce_usecs_irq,
			rx_max_coalesced_frames_irq,
			tx_coalesce_usecs,
			tx_max_coalesced_frames,
			tx_coalesce_usecs_irq,
			tx_max_coalesced_frames_irq,
			stats_block_coalesce_usecs: self.statistics_block_coalesce_microseconds,
			use_adaptive_rx_coalesce,
			use_adaptive_tx_coalesce,
			pkt_rate_low: self.low_packet_rate_packets_per_second_threshold,
			rx_coalesce_usecs_low,
			rx_max_coalesced_frames_low,
			tx_coalesce_usecs_low,
			tx_max_coalesced_frames_low,
			pkt_rate_high: self.high_packet_rate_packets_per_second_threshold,
			rx_coalesce_usecs_high,
			rx_max_coalesced_frames_high,
			tx_coalesce_usecs_high,
			tx_max_coalesced_frames_high,
			rate_sample_interval,
		}
	}
}
