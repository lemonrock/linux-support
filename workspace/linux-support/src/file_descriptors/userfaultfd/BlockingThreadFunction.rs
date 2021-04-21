// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Pass this to `ThreadConfiguration`.
pub struct BlockingThreadFunction<UFEH: UserFaultEventHandler + Send + Sync + 'static, ERAD: EventsReaderAndDispatcher + Send + Sync + 'static, UFFDW: AsRef<Arc<UserFaultFileDescriptor>> + Send + Sync + 'static, UFEHC: FnOnce(Arc<UFFDW>) -> UFEH + Send + Sync + 'static, ERADC: FnOnce(&Arc<UserFaultFileDescriptor>, UFEH) -> ERAD + Send + Sync + 'static>
{
	user_fault_file_descriptor: Arc<UserFaultFileDescriptor>,
	user_fault_file_descriptor_wrapper: Arc<UFFDW>,
	user_fault_event_handler_constructor: UFEHC,
	events_reader_and_dispatcher_constructor: ERADC,
	marker: PhantomData<(ERAD, UFEH)>
}

impl<UFEH: UserFaultEventHandler + Send + Sync + 'static, ERAD: EventsReaderAndDispatcher + Send + Sync + 'static, UFFDW: AsRef<Arc<UserFaultFileDescriptor>> + Send + Sync + 'static, UFEHC: FnOnce(Arc<UFFDW>) -> UFEH + Send + Sync + 'static, ERADC: FnOnce(&Arc<UserFaultFileDescriptor>, UFEH) -> ERAD + Send + Sync + 'static> ThreadFunction for BlockingThreadFunction<UFEH, ERAD, UFFDW, UFEHC, ERADC>
{
	type TLBF = BlockingThreadLoopBodyFunction<ERAD>;
	
	#[inline(always)]
	fn initialize(self) -> Result<Self::TLBF, Box<dyn error::Error + 'static>>
	{
		Ok
		(
			BlockingThreadLoopBodyFunction
			{
				blocking_user_fault_file_descriptor:
				{
					let user_fault_event_handler = (self.user_fault_event_handler_constructor)(self.user_fault_file_descriptor_wrapper);
					let event_reader_and_dispatcher = (self.events_reader_and_dispatcher_constructor)(&self.user_fault_file_descriptor, user_fault_event_handler);
					BlockingUserFaultFileDescriptor(event_reader_and_dispatcher)
				}
			}
		)
	}
}
