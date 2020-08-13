// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Adaptive coalescing configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdaptiveCoalescingConfiguration
{
	/// Driver must support `ETHTOOL_COALESCE_USE_ADAPTIVE_RX`.
	#[allow(missing_docs)]
	ReceiveOnly(AdaptiveCoalescingRateSampling),
	
	/// Driver must support `ETHTOOL_COALESCE_USE_ADAPTIVE_TX`.
	#[allow(missing_docs)]
	TransmitOnly(AdaptiveCoalescingRateSampling),
	
	/// Driver must support `ETHTOOL_COALESCE_USE_ADAPTIVE_RX` and `ETHTOOL_COALESCE_USE_ADAPTIVE_TX`.
	#[allow(missing_docs)]
	ReceiveAndTransmit(AdaptiveCoalescingRateSampling),
}

impl AdaptiveCoalescingConfiguration
{
	fn to_values(this: &Option<Self>) -> (Option<NonZeroU32>, u32, u32)
	{
		use self::AdaptiveCoalescingConfiguration::*;
		
		let (adaptive_coalescing_rate_sampling, use_adaptive_receive_coalesce, use_adaptive_transmit_coalesce) = match this
		{
			&None => (AdaptiveCoalescingRateSampling::One, false, false),
			
			&Some(this) => match this
			{
				ReceiveOnly(adaptive_coalescing_rate_sampling) => (adaptive_coalescing_rate_sampling, true, false),
				TransmitOnly(adaptive_coalescing_rate_sampling) => (adaptive_coalescing_rate_sampling, false, true),
				ReceiveAndTransmit(adaptive_coalescing_rate_sampling) => (adaptive_coalescing_rate_sampling, true, true),
			},
		};
		(adaptive_coalescing_rate_sampling.interval_in_seconds, use_adaptive_receive_coalesce as u32, use_adaptive_transmit_coalesce as u32)
	}
}
