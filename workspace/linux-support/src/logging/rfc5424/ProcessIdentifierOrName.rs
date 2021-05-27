// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Is not permitted to be empty.
///
/// This should ordinarily be the `ProcessIdentifier`.
#[repr(transparent)]
pub struct ProcessIdentifierOrName(ArrayVec<PrintableAsciiCharacter, 128>);

impl Deref for ProcessIdentifierOrName
{
	type Target = [PrintableAsciiCharacter];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl ProcessIdentifierOrName
{
	/// ***SLOW***.
	///
	/// Makes system call.
	///
	/// Should be cached only after a daemon has double-forked.
	#[inline(always)]
	fn new() -> Result<Self, PrintableAsciiCharacterPushError>
	{
		let process_identifier = ProcessIdentifier::default();
		let process_identifier = &process_identifier.into_line_feed_terminated_byte_string()[..];
		let raw_slice = &process_identifier[..process_identifier.len() - 1];
		
		let mut inner = ArrayVec::new();
		PrintableAsciiCharacter::push_raw_slice_into_array_vec(raw_slice, &mut inner)?;
		Ok(Self(inner))
	}
}
