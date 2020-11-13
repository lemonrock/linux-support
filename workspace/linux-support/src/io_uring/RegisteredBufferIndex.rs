// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Registered buffer index.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct RegisteredBufferIndex(pub u16);

impl RegisteredBufferIndex
{
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(Self::ExclusiveMaximum.get() - 1);
	
	/// Exclusive maximum.
	pub const ExclusiveMaximum: NonZeroU16 = new_non_zero_u16(1_024);
}
