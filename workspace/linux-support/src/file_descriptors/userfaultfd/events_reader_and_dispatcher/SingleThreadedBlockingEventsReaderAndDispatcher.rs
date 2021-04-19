// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
pub struct SingleThreadedBlockingEventsReaderAndDispatcher<UFEH: UserFaultEventHandler>
{
	event_reader: EventReader,
	event_dispatcher: EventDispatcher<UFEH>,
}

impl<UFEH: UserFaultEventHandler> EventsReaderAndDispatcher for SingleThreadedBlockingEventsReaderAndDispatcher<UFEH>
{
	#[inline(always)]
	fn read_and_dispatch_events_blocking(&mut self)
	{
		const NumberOfMessages: usize = 1;
		
		let mut buffer: MaybeUninit<uffd_msg> = MaybeUninit::uninit();
		let buffer_slice = NonNull::slice_from_raw_parts(new_non_null(buffer.as_mut_ptr()), NumberOfMessages);
		
		let number_of_messages = self.event_reader.blocking_read_events(buffer_slice);
		debug_assert_eq!(number_of_messages.get(), NumberOfMessages);
		
		unsafe { self.event_dispatcher.dispatch_event(buffer.assume_init_ref()); }
	}
}

impl<UFEH: UserFaultEventHandler> SingleThreadedBlockingEventsReaderAndDispatcher<UFEH>
{
	#[inline(always)]
	pub(super) fn new(file_descriptor: &Arc<UserFaultFileDescriptor>, user_fault_event_handler: UFEH) -> Self
	{
		Self
		{
			event_reader: EventReader::new(file_descriptor),
			event_dispatcher: EventDispatcher::new(user_fault_event_handler),
		}
	}
}
