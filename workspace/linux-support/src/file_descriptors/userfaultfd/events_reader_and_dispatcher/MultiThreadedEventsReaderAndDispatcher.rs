// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) struct MultiThreadedEventsReaderAndDispatcher<UFEH: UserFaultEventHandler>
{
	event_reader: EventReader,
	event_dispatcher: EventDispatcher<UFEH>,
	buffer: Vec<uffd_msg>,
}

impl<UFEH: UserFaultEventHandler> EventsReaderAndDispatcher for MultiThreadedEventsReaderAndDispatcher<UFEH>
{
	#[inline(always)]
	fn read_and_dispatch_events_blocking(&mut self)
	{
		let number_of_messages = self.event_reader.blocking_read_events(self.buffer());
		self.dispatch_events(number_of_messages.get());
	}
	
	#[inline(always)]
	fn read_and_dispatch_events_non_blocking(&mut self) -> bool
	{
		let number_of_messages = self.event_reader.non_blocking_read_events(self.buffer());
		self.dispatch_events(number_of_messages)
	}
}

impl<UFEH: UserFaultEventHandler> MultiThreadedEventsReaderAndDispatcher<UFEH>
{
	#[inline(always)]
	pub(super) fn new(file_descriptor: &Arc<UserFaultFileDescriptor>, user_fault_event_handler: UFEH, initial_number_of_events_to_read_at_once: NonZeroUsize) -> Self
	{
		Self
		{
			event_reader: EventReader::new(file_descriptor),
			event_dispatcher: EventDispatcher::new(user_fault_event_handler),
			buffer:
			{
				let length = initial_number_of_events_to_read_at_once.get();
				let mut events = Vec::with_capacity(length);
				unsafe { events.set_len(length) };
				events
			},
		}
	}
	
	#[inline(always)]
	fn dispatch_events(&mut self, number_of_messages: usize) -> bool
	{
		for index in 0 .. number_of_messages
		{
			let event = self.buffer.get_unchecked_safe(index);
			self.event_dispatcher.dispatch_event(event)
		}
		
		if unlikely!(number_of_messages == self.buffer.capacity())
		{
			self.increase_buffer_capacity();
			true
		}
		else
		{
			false
		}
	}
	
	#[inline(always)]
	fn buffer(&mut self) -> NonNull<[uffd_msg]>
	{
		NonNull::slice_from_raw_parts(new_non_null(self.buffer.as_mut_ptr()), self.buffer.capacity())
	}
	
	#[inline(always)]
	fn increase_buffer_capacity(&mut self)
	{
		self.buffer.reserve_exact(self.buffer.capacity());
		unsafe { self.buffer.set_len(self.buffer.capacity()) }
	}
}
