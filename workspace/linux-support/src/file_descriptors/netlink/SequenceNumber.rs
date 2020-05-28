// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A sequence number should increment - but does not have to.
///
/// Sequence number `0` is commonly used for messages from the Linux kernel to user space that are not correlated to any request made from user space to the Linux kernel.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequenceNumber(u32);

impl SequenceNumber
{
	/// Sequence number `Zero` is commonly used for messages from the Linux kernel to user space that are not correlated to any request made from user space to the Linux kernel.
	///
	/// It is valid for user space to use it in a request to the Linux kernel, but unwise.
	pub const Zero: Self = Self(0);
	
	const One: Self = Self(1);
	
	/// Increment.
	///
	/// Wraps around to one, not zero.
	#[inline(always)]
	fn get_then_increment(&mut self) -> Self
	{
		let current_value = self.0;
		
		self.0 = if unlikely!(current_value == u32::MAX)
		{
			1
		}
		else
		{
			current_value + 1
		};
		
		Self(current_value)
	}
	
}
