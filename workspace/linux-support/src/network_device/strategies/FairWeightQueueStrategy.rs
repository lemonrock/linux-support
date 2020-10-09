// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A weight queue strategy that is fair to all queues.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FairWeightQueueStrategy;

impl WeightQueueStrategy for FairWeightQueueStrategy
{
	fn weight(&self, queue_index: QueueIdentifier, number_of_receive_queues: QueueCount, denominator: NonZeroU32) -> u32
	{
		let queue_count: u32 = number_of_receive_queues.into();
		let denominator = denominator.get();
		
		if denominator < queue_count
		{
			1
		}
		else
		{
			let most_weight = denominator / queue_count;
			
			// Distribute the remainder to earlier numbered queues.
			let queue_index: u32 = queue_index.into();
			let remainder = denominator % queue_count;
			if queue_index < remainder
			{
				most_weight + 1
			}
			else
			{
				most_weight
			}
		}
	}
}
