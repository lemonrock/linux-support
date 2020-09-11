// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ContextIdentifierOrCreate
{
	identifier: ContextIdentifier,
	
	create: NonZeroU32,
}

impl Debug for ContextIdentifierOrCreate
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "ContextIdentifierOrCreate({:?})", unsafe { self.create.get() })
	}
}

impl ContextIdentifierOrCreate
{
	pub(crate) const Create: Self = Self
	{
		create: ContextIdentifier::ExclusiveMaximum.0,
	};
	
	pub(crate) const fn identifier(identifier: ContextIdentifier) -> Self
	{
		Self
		{
			identifier
		}
	}
}
