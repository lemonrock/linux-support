// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Resolves the value of items of type `Immediate::Named`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffsetsMap<OV: OffsetValue>(UsageHashMap<OV>);

impl<OV: OffsetValue> Default for OffsetsMap<OV>
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::ProgramError::*;
		
		Self(UsageHashMap::new(CouldNotResolveOffset, NotAllOffsetsHaveBeenResolved))
	}
}

impl<OV: OffsetValue> OffsetsMap<OV>
{
	#[inline(always)]
	pub(crate) fn resolve<'de>(&self, offset: &impl AsRef<Offset<'de, OV>>) -> Result<OV, ProgramError>
	{
		use self::Offset::*;
		
		match offset.as_ref()
		{
			&Known(value) => Ok(value),
			
			&Named(Name(ref name)) => self.0.resolve(name.deref()),
		}
	}
	
	#[inline(always)]
	pub(crate) fn guard_all_values_have_been_resolved_at_least_once(self) -> Result<(), ProgramError>
	{
		self.0.guard_all_values_have_been_resolved_at_least_once()
	}
}
