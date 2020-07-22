// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Defaults to `Void`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BpfTypeFormatTypeIdentifier(Option<NonVoidBpfTypeFormatTypeIdentifier>);

impl BpfTypeFormatTypeIdentifier
{
	/// `void`.
	pub const Void: Self = Self(None);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(Some(NonVoidBpfTypeFormatTypeIdentifier::InclusiveMaximum));
	
	/// New instance.
	#[inline(always)]
	pub const fn new(value: NonVoidBpfTypeFormatTypeIdentifier) -> Self
	{
		Self(Some(value))
	}
	
	#[inline(always)]
	pub(crate) fn next(&mut self) -> Result<Self, BpfTypeFormatError>
	{
		if unlikely!(self == &mut Self::InclusiveMaximum)
		{
			return Err(BpfTypeFormatError::TooManyTypes)
		}
		
		let inner: u32 = unsafe { transmute(self.0) };
		*self = Self(Some(NonVoidBpfTypeFormatTypeIdentifier::new_from_u32(inner + 1)));
		
		Ok(*self)
	}
}
