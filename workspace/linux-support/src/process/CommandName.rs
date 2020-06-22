// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Command (process or thread) name.
///
/// Also known as `comm`.
///
/// Deref excludes trailing ASCII NUL.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CommandName(pub(crate) ArrayVec<[u8; CommandName::MaximumCommandNameLengthIncludingAsciiNul]>);

impl<'de> Deserialize<'de> for CommandName
{
	#[inline(always)]
	fn deserialize<D:  Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct CommandNameVisitor;

		impl<'de> Visitor<'de> for CommandNameVisitor
		{
			type Value = CommandName;

			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("A C-like string with a length between 0 and 15 inclusive without a trailing NUL")
			}

			#[inline(always)]
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				self.visit_bytes(v.as_ref())
			}

			#[inline(always)]
			fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E>
			{
				CommandName::new_from_bytes_excluding_ascii_nul(v).map_err(|cause| match cause
				{
					CommandNameFromBytesError::TooLong(length) => E::invalid_length(length, &"bytes with a length between 0 and 15 inclusive without a trailing NUL"),
				})
			}
		}

		deserializer.deserialize_bytes(CommandNameVisitor)
	}
}

impl Serialize for CommandName
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_bytes(self.deref())
	}
}

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
		Self::new_from_bytes_excluding_ascii_nul(bytes)
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

impl AsRef<[u8]> for CommandName
{
	#[inline(always)]
	fn as_ref(&self) -> &[u8]
	{
		self.deref()
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
	pub fn new_from_bytes_excluding_ascii_nul(bytes: &[u8]) -> Result<Self, CommandNameFromBytesError>
	{
		let length = bytes.len();

		if unlikely!(length > Self::MaximumCommandNameLengthExcludingAsciiNul)
		{
			return Err(CommandNameFromBytesError::TooLong(length))
		}

		let mut array_vec = ArrayVec::new();

		unsafe
		{
			let pointer: *mut u8 = array_vec.as_mut_ptr();
			pointer.copy_from_nonoverlapping(bytes.as_ptr(), length);
			array_vec.set_len(length)
		}
		array_vec.push(b'\0');
		Ok(Self(array_vec))
	}
}
