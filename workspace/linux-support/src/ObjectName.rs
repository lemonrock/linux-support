// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Assumes that the final byte is always ASCII NULL, ie the maximum `strnlen()` is `MaximumLengthExcludingAsciiNull`.
///
/// Deref excludes trailing ASCII NULL.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjectName<const MaximumLengthIncludingAsciiNull: usize>(pub(crate) ArrayVec<c_char, MaximumLengthIncludingAsciiNull>);

impl<'de, const MaximumLengthIncludingAsciiNull: usize> Deserialize<'de> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn deserialize<D:  Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct ObjectNameVisitor<const MaximumLengthIncludingAsciiNull: usize>;
		
		impl<'de, const MaximumLengthIncludingAsciiNull: usize> Visitor<'de> for ObjectNameVisitor<MaximumLengthIncludingAsciiNull>
		{
			type Value = ObjectName<MaximumLengthIncludingAsciiNull>;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str(stringify!("A C-like string with a length between 0 and ", MaximumLengthIncludingAsciiNull, " inclusive without a trailing NULL"))
			}
			
			#[inline(always)]
			fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E>
			{
				self.visit_bytes(v.as_ref())
			}
			
			#[inline(always)]
			fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E>
			{
				use self::ObjectNameFromBytesError::*;
				Self::Value::from_bytes(v).map_err(|error| match error
				{
					TooLong(size) => E::invalid_length(size, &stringify!("Too long; length is a maximum inclusive value of ", MaximumLengthIncludingAsciiNull)),
					ContainsAsciiNullWhereItShouldNot => E::invalid_value(Unexpected::Bytes(v), &"ContainsAsciiNullWhereItShouldNot"),
					DoesNotEndWithAsciiNull => E::invalid_value(Unexpected::Bytes(v), &"DoesNotEndWithAsciiNull"),
					Empty => E::invalid_length(0, &stringify!("Empty; length is a maximum inclusive value of ", MaximumLengthIncludingAsciiNull)),
				})
			}
		}
		
		deserializer.deserialize_bytes(ObjectNameVisitor::<MaximumLengthIncludingAsciiNull>)
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> Serialize for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_bytes(unsafe { transmute(self.deref()) })
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> Display for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let c_str: &CStr = self.as_ref();
		write!(f, "{}", c_str.to_string_lossy())
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> Debug for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let c_str: &CStr = self.as_ref();
		write!(f, stringify!(ObjectName, "({}, {:?})"), MaximumLengthIncludingAsciiNull, c_str)
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> Into<ArrayVec<c_char, MaximumLengthIncludingAsciiNull>> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn into(self) -> ArrayVec<c_char, MaximumLengthIncludingAsciiNull>
	{
		self.0
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> Into<&'a str> for &'a ObjectName<MaximumLengthIncludingAsciiNull>
{
	fn into(self) -> &'a str
	{
		let array: &[c_char; MaximumLengthIncludingAsciiNull] = self.into();
		let array: &[u8; MaximumLengthIncludingAsciiNull] = unsafe { transmute(array) };
		unsafe { from_utf8_unchecked(&array[..]) }
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> Into<[c_char; MaximumLengthIncludingAsciiNull]> for &'a ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn into(self) -> [c_char; MaximumLengthIncludingAsciiNull]
	{
		self.to_object_name()
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> Into<&'a [u8; MaximumLengthIncludingAsciiNull]> for &'a ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn into(self) -> &'a [u8; MaximumLengthIncludingAsciiNull]
	{
		let x: &[c_char; MaximumLengthIncludingAsciiNull] = self.into();
		unsafe { transmute(x) }
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> Into<&'a [c_char; MaximumLengthIncludingAsciiNull]> for &'a ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn into(self) -> &'a [c_char; MaximumLengthIncludingAsciiNull]
	{
		unsafe { & * (self.0.as_ptr() as *const [c_char; MaximumLengthIncludingAsciiNull]) }
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> Into<[c_char; MaximumLengthIncludingAsciiNull]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn into(self) -> [c_char; MaximumLengthIncludingAsciiNull]
	{
		self.into_object_name()
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> TryInto<String> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = Utf8Error;
	
	#[inline(always)]
	fn try_into(self) -> Result<String, Self::Error>
	{
		let c_str: &CStr = self.as_ref();
		Ok(c_str.to_str()?.to_string())
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> TryFrom<&'a Self> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = Infallible;
	
	#[inline(always)]
	fn try_from(value: &'a Self) -> Result<Self, Self::Error>
	{
		Ok(value.clone())
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> TryFrom<CString> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: CString) -> Result<Self, Self::Error>
	{
		Self::from_bytes(value.as_bytes())
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> TryFrom<&'a CStr> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: &'a CStr) -> Result<Self, Self::Error>
	{
		Self::from_bytes(value.to_bytes())
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> TryFrom<String> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: String) -> Result<Self, Self::Error>
	{
		Self::from_bytes(value.as_bytes())
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> TryFrom<&'a str> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: &'a str) -> Result<Self, Self::Error>
	{
		Self::from_bytes(value.as_bytes())
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> TryFrom<&'a [u8]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
	{
		Self::from_bytes(value)
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> TryFrom<&'a [c_char]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: &'a [c_char]) -> Result<Self, Self::Error>
	{
		Self::from_bytes(unsafe { transmute(value) })
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> TryFrom<[u8; MaximumLengthIncludingAsciiNull]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: [u8; MaximumLengthIncludingAsciiNull]) -> Result<Self, Self::Error>
	{
		Self::try_from(value)
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> TryFrom<[c_char; MaximumLengthIncludingAsciiNull]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: [c_char; MaximumLengthIncludingAsciiNull]) -> Result<Self, Self::Error>
	{
		let length_including_ascii_nul = match Self::index_of_ascii_null_c_char(&value[..])
		{
			Some(index) => index + 1,
			None => return Err(ObjectNameFromBytesError::DoesNotEndWithAsciiNull),
		};
		
		let mut array_vec = ArrayVec::from(value);
		unsafe { array_vec.set_len(length_including_ascii_nul) };
		
		Ok(Self(array_vec))
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> TryFrom<&'a [u8; MaximumLengthIncludingAsciiNull]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: &'a [u8; MaximumLengthIncludingAsciiNull]) -> Result<Self, Self::Error>
	{
		let value: &'a [c_char; MaximumLengthIncludingAsciiNull] = unsafe { transmute(value) };
		Self::try_from(value)
	}
}

impl<'a, const MaximumLengthIncludingAsciiNull: usize> TryFrom<&'a [c_char; MaximumLengthIncludingAsciiNull]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: &'a [c_char; MaximumLengthIncludingAsciiNull]) -> Result<Self, Self::Error>
	{
		let length_including_ascii_nul = match Self::index_of_ascii_null_c_char(&value[..])
		{
			Some(index) => index + 1,
			None => return Err(ObjectNameFromBytesError::DoesNotEndWithAsciiNull),
		};
		
		let mut array_vec: ArrayVec<c_char, MaximumLengthIncludingAsciiNull> = ArrayVec::new();
		
		unsafe
		{
			let pointer = array_vec.as_mut_ptr();
			pointer.copy_from_nonoverlapping(value.as_ptr(), length_including_ascii_nul);
			array_vec.set_len(length_including_ascii_nul)
		}
		Ok(Self(array_vec))
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> TryFrom<ArrayVec<u8, MaximumLengthIncludingAsciiNull>> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: ArrayVec<u8, MaximumLengthIncludingAsciiNull>) -> Result<Self, Self::Error>
	{
		Self::try_from(value)
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> TryFrom<ArrayVec<c_char, MaximumLengthIncludingAsciiNull>> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: ArrayVec<c_char, MaximumLengthIncludingAsciiNull>) -> Result<Self, Self::Error>
	{
		let length_including_ascii_null = value.len();
		
		if unlikely!(length_including_ascii_null == 0)
		{
			return Err(ObjectNameFromBytesError::Empty)
		}
		
		let expected_ascii_null = value.get_unchecked_value_safe(length_including_ascii_null - 1);
		if unlikely!(expected_ascii_null != Self::AsciiNull)
		{
			return Err(ObjectNameFromBytesError::DoesNotEndWithAsciiNull)
		}
		
		Ok(Self(value))
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> FromBytes for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		let length = bytes.len();
		
		if unlikely!(length > Self::MaximumLengthExcludingAsciiNull)
		{
			return Err(ObjectNameFromBytesError::TooLong(length))
		}
		
		if Self::index_of_ascii_null(bytes).is_some()
		{
			return Err(ObjectNameFromBytesError::ContainsAsciiNullWhereItShouldNot)
		}
		
		let mut array_vec = ArrayVec::new();
		
		unsafe
		{
			let pointer = array_vec.as_mut_ptr() as *mut c_char;
			pointer.copy_from_nonoverlapping(bytes.as_ptr() as *const c_char, length);
			array_vec.set_len(length)
		}
		array_vec.push(Self::AsciiNull);
		Ok(Self(array_vec))
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> Deref for ObjectName<MaximumLengthIncludingAsciiNull>
{
	type Target = [c_char];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		let bytes = self.0.as_slice();
		let length = bytes.len();
		debug_assert_ne!(length, 0, "There should always be a trailing ASCII NULL");
		&bytes[0 .. length - 1]
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> AsRef<ArrayVec<c_char, MaximumLengthIncludingAsciiNull>> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn as_ref(&self) -> &ArrayVec<c_char, MaximumLengthIncludingAsciiNull>
	{
		&self.0
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> AsRef<CStr> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn as_ref(&self) -> &CStr
	{
		unsafe { CStr::from_bytes_with_nul_unchecked(transmute(self.0.as_slice())) }
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> AsRef<OsStr> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn as_ref(&self) -> &OsStr
	{
		OsStr::from_bytes(self.as_ref())
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> AsRef<Path> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn as_ref(&self) -> &Path
	{
		let this: &OsStr = self.as_ref();
		Path::new(this)
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> AsRef<[c_char]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn as_ref(&self) -> &[c_char]
	{
		self.deref()
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> AsRef<[u8]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn as_ref(&self) -> &[u8]
	{
		unsafe { transmute(self.deref()) }
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> Borrow<ArrayVec<c_char, MaximumLengthIncludingAsciiNull>> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn borrow(&self) -> &ArrayVec<c_char, MaximumLengthIncludingAsciiNull>
	{
		self.as_ref()
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> Borrow<CStr> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn borrow(&self) -> &CStr
	{
		self.as_ref()
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> Borrow<[c_char]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn borrow(&self) -> &[c_char]
	{
		self.as_ref()
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> Borrow<[u8]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn borrow(&self) -> &[u8]
	{
		self.as_ref()
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> PartialEq<[c_char; MaximumLengthIncludingAsciiNull]> for ObjectName<MaximumLengthIncludingAsciiNull>
{
	#[inline(always)]
	fn eq(&self, other: &[c_char; MaximumLengthIncludingAsciiNull]) -> bool
	{
		let index = match Self::index_of_ascii_null(unsafe { transmute(other as &[c_char]) })
		{
			None => return false,
			Some(index) => index,
		};
		
		self.deref() == &other[0 .. index]
	}
}

impl<const MaximumLengthIncludingAsciiNull: usize> ObjectName<MaximumLengthIncludingAsciiNull>
{
	const AsciiNull: c_char = b'\0' as i8;
	
	/// Maximum object name length.
	pub const MaximumLengthExcludingAsciiNull: usize = MaximumLengthIncludingAsciiNull - 1;
	
	/// Construct from a C-like function call.
	///
	/// Provides the C-like function with a correctly sized buffer.
	#[inline(always)]
	pub fn construct_from_c_function_call<Error>(c_function: impl FnOnce(&mut [c_char; MaximumLengthIncludingAsciiNull]) -> Result<(), Error>, does_not_end_with_ascii_nul_error: impl FnOnce() -> Error) -> Result<Self, Error>
	{
		let mut buffer: [c_char; MaximumLengthIncludingAsciiNull] = unsafe_uninitialized();
		c_function(&mut buffer)?;
		
		let index = match Self::index_of_ascii_null_c_char(&buffer)
		{
			Some(index) => index,
			None => return Err(does_not_end_with_ascii_nul_error())
		};
		
		let mut array_vec = ArrayVec::from(buffer);
		unsafe { array_vec.set_len(index + 1) };
		
		Ok(Self(array_vec))
	}
	
	/// Reads from a Linux ProcPath or SysPath resource.
	#[inline(always)]
	pub fn read_from_file_line_feed_terminated(file_path: &Path) -> io::Result<Self>
	{
		Ok(file_path.read_value()?)
	}
	
	/// Writes to a Linux ProcPath or SysPath resource.
	#[inline(always)]
	pub fn write_to_file_line_feed_terminated(&self, file_path: &Path) -> io::Result<()>
	{
		file_path.write_value_then_line_feed(self.as_ref())
	}
	
	#[inline(always)]
	fn to_object_name(&self) -> [c_char; MaximumLengthIncludingAsciiNull]
	{
		let buffer = &self.0;
		let pointer = buffer.as_ptr();
		unsafe { * (pointer as *const [c_char; MaximumLengthIncludingAsciiNull]) }
	}
	
	#[inline(always)]
	fn into_object_name(self) -> [c_char; MaximumLengthIncludingAsciiNull]
	{
		let mut array_vec = self.0;
		unsafe { array_vec.set_len(MaximumLengthIncludingAsciiNull) };
		array_vec.into_inner().unwrap()
	}
	
	#[inline(always)]
	fn index_of_ascii_null_c_char(haystack: &[c_char]) -> Option<usize>
	{
		Self::index_of_ascii_null(unsafe { transmute(haystack) })
	}
	
	#[inline(always)]
	fn index_of_ascii_null(haystack: &[u8]) -> Option<usize>
	{
		memchr(Self::AsciiNull as u8, haystack)
	}
}
