// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A message to send between threads.
pub enum DogStatsDMessage<'a, HeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>
{
	/// Metric.
	Metric(Metric<'a>),
	
	/// Event.
	Event(Event<'a, HeapSize, GTACSA>),
	
	/// Service check.
	ServiceCheck(ServiceCheck<'a, HeapSize, GTACSA>),
}

impl<'a, HeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> Message for DogStatsDMessage<'a, HeapSize, GTACSA>
{
	type ConstructMessageArguments = Self;
	
	#[inline(always)]
	unsafe fn construct_message(uninitialized_memory: NonNull<Self>, construct_message_arguments: Self)
	{
		let pointer = uninitialized_memory.as_ptr();
		pointer.write(construct_message_arguments);
	}
	
	type MessageHandlerArguments = ();
	
	type DequeuedMessageProcessingError = std::io::Error;
	
	#[inline(always)]
	fn handle_message(&mut self, message_handler_arguments: &Self::MessageHandlerArguments) -> Result<(), Self::DequeuedMessageProcessingError>
	{
		unimplemented!("TODO: Finish me");
	}
}
