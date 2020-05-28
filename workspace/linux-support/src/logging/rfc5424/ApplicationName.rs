// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Is not permitted to be empty.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ApplicationName(ArrayVec<[PrintableAsciiCharacter; 48]>);

impl Deref for ApplicationName
{
	type Target = [PrintableAsciiCharacter];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl ApplicationName
{
	/// This process name should be the one used in `ProcessConfiguration`.
	///
	/// ***SLOW***.
	#[inline(always)]
	pub fn new_from_process_name(process_name: &ProcessName) -> Result<Self, PrintableAsciiCharacterPushError>
	{
		Self::new(&process_name[..])
	}
	
	/// ***SLOW***.
	#[inline(always)]
	pub fn new_from_program_invocation_short_name() -> Result<Self, PrintableAsciiCharacterPushError>
	{
		let slice = get_program_invocation_short_name().to_bytes();
		Self::new(slice)
	}
	
	#[inline(always)]
	fn new(raw_slice: &[u8]) -> Result<Self, PrintableAsciiCharacterPushError>
	{
		let mut inner = ArrayVec::new();
		PrintableAsciiCharacter::push_raw_slice_into_array_vec(raw_slice, &mut inner)?;
		Ok(Self(inner))
	}
}
