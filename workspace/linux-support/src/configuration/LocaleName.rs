// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A locale name, such as `en_US.UTF-8`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LocaleName(Either<&'static CStr, CString>);

impl<'de> Deserialize<'de> for LocaleName
{
	#[inline(always)]
	fn deserialize<D:  Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		Ok(Self(Right(CString::deserialize(deserializer)?)))
	}
}

impl Serialize for LocaleName
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		match self.0
		{
			Left(c_str) => c_str.to_owned().serialize(serializer),
			Right(ref c_string) => c_string.serialize(serializer),
		}
	}
}

impl Default for LocaleName
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(b"en_US.UTF-8")
	}
}

impl LocaleName
{
	/// A locale name is typically of the form `language[_territory][.codeset][@modifier]`, where `language` is an ISO 639 language code, `territory` is an ISO 3166 country code, and `codeset` is a character set or encoding identifier like `ISO-8859-1` or `UTF-8`.
	#[inline(always)]
	pub fn new(bytes_without_ascii_null: &[u8]) -> Self
	{
		Self(Right(unsafe { CString::from_vec_unchecked(bytes_without_ascii_null.to_vec()) }))
	}

	/// Sets for `LC_ALL`.
	///
	/// Returns a statically allocated instance for the previous setting, which may be overwritten if `set()` is called again.
	#[inline(always)]
	pub fn set_all(&self) -> Result<Self, ()>
	{
		self.set_for_category(LC_ALL)
	}

	#[inline(always)]
	fn set_for_category(&self, category: i32) -> Result<Self, ()>
	{
		let pointer = match self.0
		{
			Left(c_str) => c_str.as_ptr(),
			Right(ref c_string) => c_string.as_ptr(),
		};

		let result = unsafe { setlocale(LC_ALL, pointer) };
		if unlikely!(result.is_null())
		{
			Err(())
		}
		else
		{
			Ok(Self(Left(unsafe { CStr::from_ptr(result) })))
		}
	}
}
