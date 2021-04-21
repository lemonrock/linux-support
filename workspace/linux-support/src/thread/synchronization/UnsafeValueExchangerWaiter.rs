// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(transparent)]
pub(super) struct UnsafeValueExchangerWaiter<'v, V>(&'v UnsafeValueExchanger<V>);

impl<'v, V> UnsafeValueExchangerWaiter<'v, V>
{
	#[inline(always)]
	pub(super) fn wait(self) -> V
	{
		self.0.debug_assert_has_been_split();
		
		while self.0.lock_is_not_yet_reached()
		{
			park()
		}
		
		self.0.take_value()
	}
}
