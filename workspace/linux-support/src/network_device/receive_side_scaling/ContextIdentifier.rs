// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A non-default receive-side scaling context identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ContextIdentifier(NonZeroU32);

impl TryFrom<NonZeroU32> for ContextIdentifier
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU32) -> Result<Self, Self::Error>
	{
		if likely!(value < Self::ExclusiveMaximum.0)
		{
			Ok(Self(value))
		}
		else
		{
			Err(ParseNumberError::WasMaximum)
		}
	}
}

impl ContextIdentifier
{
	/// Exclusive maximum.
	const ExclusiveMaximum: Self = Self(unsafe { NonZeroU32::new_unchecked(ETH_RXFH_CONTEXT_ALLOC) });
	
	/// Default.
	pub const Default: Option<Self> = None;
}
