// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A structure to be used on a dedicated thread for cooperative monitoring of the current process.
pub struct BlockingUserFaultFileDescriptor<UFEH: UserFaultEventHandler>
{
	event_reader_and_handler: EventReaderAndDispatcher<UFEH>,
}

impl<UFEH: UserFaultEventHandler> BlockingUserFaultFileDescriptor<UFEH>
{
	#[inline(always)]
	fn new(file_descriptor: &Arc<UserFaultFileDescriptor>, initial_number_of_events_to_read_at_once: NonZeroUsize, user_fault_event_handler: UFEH) -> Self
	{
		Self
		{
			event_reader_and_handler: EventReaderAndDispatcher::new(file_descriptor, initial_number_of_events_to_read_at_once, user_fault_event_handler),
		}
	}
	
	#[inline(always)]
	fn read_and_handle_events(&mut self)
	{
		loop
		{
			self.event_reader_and_handler.read_and_dispatch_events_blocking()
		}
	}
}
