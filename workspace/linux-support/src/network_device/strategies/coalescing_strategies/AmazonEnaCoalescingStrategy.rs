// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct AmazonEnaCoalescingStrategy;

impl CoalescingStrategy for AmazonEnaCoalescingStrategy
{
	#[inline(always)]
	fn coalesce_configuration(&self, _paired_transmit_receive_queues: bool, receive_coalescing_preference: &ReceiveCoalescingPreference, transmit_coalescing_preference: &TransmitCoalescingPreference) -> CoalesceConfiguration
	{
		CoalesceConfiguration
		{
			adaptive_coalescing: if receive_coalescing_preference.prefer_adaptive_coalescing
			{
				Some(AdaptiveCoalescingConfiguration::ReceiveOnly(AdaptiveCoalescingRateSampling { interval_in_seconds: None }))
			}
			else
			{
				None
			},
			
			receive_transmit: ReceiveTransmitCoalescing
			{
				receive: CoalescePair
				{
					microseconds: unsafe { transmute(receive_coalescing_preference.preferred_micoseconds_interval) },
					
					maximum_frames: None
				},
				
				transmit: CoalescePair
				{
					microseconds: unsafe { transmute(transmit_coalescing_preference.preferred_micoseconds_interval) },
					
					maximum_frames: None,
				},
			},
			
			receive_transmit_whilst_irq_is_being_serviced_by_the_host: ReceiveTransmitCoalescing::DisabledWhereMaximumFramesUnsupported,
			
			low_packet_rate_packets_per_second_threshold: None,
			
			receive_transmit_at_low_packet_rate: ReceiveTransmitCoalescing::DisabledWhereMaximumFramesUnsupported,
			
			high_packet_rate_packets_per_second_threshold: None,
			
			receive_transmit_at_high_packet_rate: ReceiveTransmitCoalescing::DisabledWhereMaximumFramesUnsupported,
			
			statistics_block_coalesce_microseconds: None
		}
	}
}
