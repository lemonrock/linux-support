// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Is not permitted to be empty.
///
/// Should be qualifier to `Facility` which is scoped by `ApplicationName`.
#[repr(transparent)]
pub struct MessageIdentifier(ArrayVec<PrintableAsciiCharacter, 32>);

impl Deref for MessageIdentifier
{
	type Target = [PrintableAsciiCharacter];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl MessageIdentifier
{
	/// ***SLOW***.
	#[inline(always)]
	pub fn new(raw_slice: &[u8]) -> Result<Self, PrintableAsciiCharacterPushError>
	{
		let mut inner = ArrayVec::new();
		PrintableAsciiCharacter::push_raw_slice_into_array_vec(raw_slice, &mut inner)?;
		Ok(Self(inner))
	}
}
