// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Publisher.
///
/// Can be cheaply cloned as is internally reference counted.
#[derive(Debug)]
pub(crate) struct DogStatsDPublisher<CoroutineHeapSize: 'static + MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	publisher: Rc<RoundRobinPublisher<DogStatsDMessage<'static, CoroutineHeapSize, GTACSA>, MessageHandlerArguments, DequeuedMessageProcessingError>>,
	global_allocator: &'static GTACSA,
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Clone for DogStatsDPublisher<CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			publisher: self.publisher.clone(),
			global_allocator: self.global_allocator,
		}
	}
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> DogStatsDPublisher<CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	pub(crate) fn new(queues: &Queues<MessageHandlerArguments, DequeuedMessageProcessingError>, dog_stats_d_message_subscribers: DogStatsDMessageSubscribers, global_allocator: &'static GTACSA) -> Self
	{
		Self
		{
			publisher: Rc::new(queues.round_robin_publisher(dog_stats_d_message_subscribers)),
			global_allocator,
		}
	}
	
	#[inline(always)]
	pub(crate) fn log(&self, alert: &'static EventTemplate, additional_tags: AdditionalDogStatsDTags<CoroutineHeapSize, GTACSA>, message: Arguments)
	{
		self.publish
		(
			alert.message
			(
				additional_tags,
				message,
				self.global_allocator
			)
		)
	}
	
	#[inline(always)]
	pub(crate) fn publish(&self, message: DogStatsDMessage<'static, CoroutineHeapSize, GTACSA>)
	{
		let _published_to_hyper_thread = self.publisher.publish(message);
	}
}
