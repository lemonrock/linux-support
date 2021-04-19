// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
pub struct BlockingThreadLoopBodyFunction<ERAD: EventsReaderAndDispatcher>
{
	blocking_user_fault_file_descriptor: BlockingUserFaultFileDescriptor<ERAD>,
}

impl<ERAD: EventsReaderAndDispatcher> ThreadLoopBodyFunction for BlockingThreadLoopBodyFunction<ERAD>
{
	#[inline(always)]
	fn invoke<T: Terminate>(&mut self, _terminate: &Arc<T>)
	{
		self.blocking_user_fault_file_descriptor.read_and_handle_events()
	}
}
