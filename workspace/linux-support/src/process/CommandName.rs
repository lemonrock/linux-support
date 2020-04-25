// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Command (process or thread) name.
///
/// Also known as `comm`.
///
/// Deref includes trailing ASCII NUL.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandName(pub(crate) ArrayVec<[u8; Self::MaximumCommandNameLengthIncludingAsciiNul]>);

impl Deref for CommandName
{
	type Target = [u8];

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		let bytes = self.0.as_slice();
		&bytes[0 .. bytes.len() - 1]
	}
}

impl FromBytes for CommandName
{
	type Error = CommandNameFromBytesError;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		use self::CommandNameFromBytesError::*;

		if unlikely!(bytes.len() > Self::MaximumCommandNameLengthIncludingAsciiNul)
		{
			return Err(TooLong)
		}

		let length = bytes.len();
		if unlikely!(* bytes.get_unchecked(length - 1) != b'\0')
		{
			return Err(NoTrailingNul)
		}

		let mut array_vec = ArrayVec::new();
		unsafe
		{
			let pointer: *mut u8 = array_vec.as_mut_ptr();
			pointer.copy_from_nonoverlapping(bytes.as_ptr(), length);
			array_vec.set_len(length)
		}
		Ok(Self(array_vec))
	}
}

impl AsRef<CStr> for CommandName
{
	#[inline(always)]
	fn as_ref(&self) -> &CStr
	{
		unsafe { CStr::from_bytes_with_nul_unchecked(self.0.as_slice()) }
	}
}

impl ToString for CommandName
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		unsafe { String::from_utf8_unchecked(self.deref().to_vec()) }
	}
}

impl CommandName
{
	/// Maximum command name length.
	pub const MaximumCommandNameLengthExcludingAsciiNul: usize = 15;

	/// Maximum command name length.
	///
	/// This is the same as `TASK_COMM_LEN`.
	pub const MaximumCommandNameLengthIncludingAsciiNul: usize = Self::MaximumCommandNameLengthExcludingAsciiNul + 1;

	/// New instance.
	#[inline(always)]
	pub fn new_from_bytes_excluding_ascii_nul(bytes: &[u8]) -> Self
	{
		let mut array_vec = ArrayVec::new();
		let length = bytes.len();
		debug_assert!(length <= Self::MaximumCommandNameLengthExcludingAsciiNul);
		unsafe
		{
			let pointer: *mut u8 = array_vec.as_mut_ptr();
			pointer.copy_from_nonoverlapping(bytes.as_ptr(), length);
			array_vec.set_len(length)
		}
		array_vec.push(b'\0');
		Self(array_vec)
	}
}
