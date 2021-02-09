// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) trait EventsReaderAndDispatcher
{
	fn read_and_dispatch_events_blocking(&mut self);
	
	/// Returns `true` if it is believed there are more events to read immediately.
	///
	/// Not necessarily implemented.
	#[inline(always)]
	fn read_and_dispatch_events_non_blocking(&mut self) -> bool
	{
		panic!("Not supported")
	}
}
