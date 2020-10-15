// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct AcceptDogStatsD<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	dog_stats_d_publisher: DogStatsDPublisher<CoroutineHeapSize, GTACSA>,
	socket_hyper_thread_dog_stats_d_tag: AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>,
	processing_hyper_thread_thread_local_additional_dog_stats_d_cache: Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
	service_protocol_additional_dog_stats_d_tag: AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>,
}

impl<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> AcceptDogStatsD<CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn new((dog_stats_d_publisher, global_allocator, socket_hyper_thread_thread_local_additional_dog_stats_d_cache, processing_hyper_thread_thread_local_additional_dog_stats_d_cache): (DogStatsDPublisher<CoroutineHeapSize, GTACSA>, &'static GTACSA, Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>, Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>), service_protocol_identifier: ServiceProtocolIdentifier, socket_hyper_thread: HyperThread) -> Self
	{
		Self
		{
			dog_stats_d_publisher,
			socket_hyper_thread_dog_stats_d_tag: socket_hyper_thread_thread_local_additional_dog_stats_d_cache.get(socket_hyper_thread),
			processing_hyper_thread_thread_local_additional_dog_stats_d_cache,
			service_protocol_additional_dog_stats_d_tag: AdditionalDogStatsDTag::from(DogStatsDTag::from_u8("service_protocol", service_protocol_identifier).unwrap(), global_allocator),
		}
	}
	
	#[inline(always)]
	fn log(&self, alert: &'static EventTemplate, message: Arguments)
	{
		let additional_tags = additional_dog_stats_d_tags!
		[
			self.service_protocol_additional_dog_stats_d_tag()
		];
		self.dog_stats_d_publisher.log(alert, additional_tags, message)
	}
	
	#[inline(always)]
	fn increment_connection_count(&self, processing_hyper_thread: HyperThread)
	{
		let increment_connection_count = unsafe
		{
			#[thread_local] static mut IncrementConnectionCount: Option<MetricTemplate> = None;
			if unlikely!(IncrementConnectionCount.is_none())
			{
				IncrementConnectionCount = Some(MetricTemplate::new_with_common_tags("connection.count"))
			}
			IncrementConnectionCount.as_ref().unwrap()
		};
		
		self.dog_stats_d_publisher.publish
		(
			increment_connection_count.message
			(
				additional_dog_stats_d_tags!
				[
					self.service_protocol_additional_dog_stats_d_tag(),
					self.socket_hyper_thread_dog_stats_d_tag(),
					self.processing_hyper_thread_thread_local_additional_dog_stats_d_cache.get(processing_hyper_thread)
				],
				MetricValue::IncrementUnsampled
			)
		)
	}
	
	#[inline(always)]
	fn socket_hyper_thread_dog_stats_d_tag(&self) -> AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>
	{
		self.socket_hyper_thread_dog_stats_d_tag.clone()
	}
	
	#[inline(always)]
	fn service_protocol_additional_dog_stats_d_tag(&self) -> AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>
	{
		self.service_protocol_additional_dog_stats_d_tag.clone()
	}
}
