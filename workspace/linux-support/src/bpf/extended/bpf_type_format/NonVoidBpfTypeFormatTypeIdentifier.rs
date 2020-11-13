// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Type identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NonVoidBpfTypeFormatTypeIdentifier(NonZeroU32);

impl NonVoidBpfTypeFormatTypeIdentifier
{
	/// Inclusive maximum.
	pub const InclusiveMinimum: Self = Self::new_from_u32(1);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self::new_from_u32(BTF_MAX_TYPE as u32);
	
	/// New instance.
	#[inline(always)]
	pub const fn new(value: NonZeroU32) -> Self
	{
		Self(value)
	}
	
	/// New instance.
	#[inline(always)]
	const fn new_from_u32(value: u32) -> Self
	{
		Self(new_non_zero_u32(value))
	}
}
