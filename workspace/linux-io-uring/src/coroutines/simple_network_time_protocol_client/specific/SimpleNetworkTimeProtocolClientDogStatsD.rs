// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct SimpleNetworkTimeProtocolClientDogStatsD<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	dog_stats_d_publisher: DogStatsDPublisher<CoroutineHeapSize, GTACSA>,
}

impl<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> SimpleNetworkTimeProtocolClientDogStatsD<CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn new(dog_stats_d_publisher: DogStatsDPublisher<CoroutineHeapSize, GTACSA>) -> Self
	{
		Self
		{
			dog_stats_d_publisher,
		}
	}
	
	#[inline(always)]
	fn log(&self, alert: &'static EventTemplate, message: Arguments)
	{
		let additional_tags = additional_dog_stats_d_tags!
		[
		];
		self.dog_stats_d_publisher.log(alert, additional_tags, message)
	}
}
