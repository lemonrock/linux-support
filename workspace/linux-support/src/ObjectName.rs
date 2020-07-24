// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Linux object name, used for Commands, Processes, Threads, NetworkInterfaces, BPF maps and BPF programs.
///
/// Relies on the fact that the following are all the same length:-
///
/// * `TASK_COMM_LEN`.
/// * `BPF_OBJ_NAME_LEN`.
/// * `IFNAMSIZ`.
/// * `IF_NAMESIZE`.
///
/// Assumes that the final byte is always ASCII NULL, ie the maximum `strnlen()` is `MaximumObjectNameLengthExcludingAsciiNull`.
///
/// Deref excludes trailing ASCII NULL.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjectName(pub(crate) ArrayVec<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]>);

impl<'de> Deserialize<'de> for ObjectName
{
	#[inline(always)]
	fn deserialize<D:  Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct ObjectNameVisitor;
		
		impl<'de> Visitor<'de> for ObjectNameVisitor
		{
			type Value = ObjectName;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("A C-like string with a length between 0 and 15 inclusive without a trailing NULL")
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
				Self::from_bytes(v).map_err(|cause| match cause
				{
					TooLong(length) => E::invalid_length(length, &"Too Long: expected bytes with a length between 0 and 15 inclusive without a trailing NULL or interior NULL"),
					ContainsAsciiNullWhereItShouldNot => E::invalid_length(length, &"Contains NULL: bytes with a length between 0 and 15 inclusive without a trailing NULL or interior NULL"),
					_ => unreachable!("Unexpected error in from_bytes()")
				})
			}
		}
		
		deserializer.deserialize_bytes(ObjectNameVisitor)
	}
}

impl Serialize for ObjectName
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_bytes(unsafe { transmute(self.deref()) })
	}
}

impl Display for ObjectName
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let c_str: &CStr = self.as_ref();
		write!(f, "{}", c_str.to_string_lossy())
	}
}

impl Debug for ObjectName
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let c_str: &CStr = self.as_ref();
		write!(f, "ObjectName({:?})", c_str)
	}
}

impl Into<ArrayVec<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]>> for ObjectName
{
	#[inline(always)]
	fn into(self) -> ArrayVec<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]>
	{
		self.0
	}
}

impl<'a> Into<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]> for &'a ObjectName
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]
	{
		self.to_object_name()
	}
}

impl Into<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]> for ObjectName
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]
	{
		self.into_object_name()
	}
}

impl TryInto<String> for ObjectName
{
	type Error = Utf8Error;
	
	#[inline(always)]
	fn try_into(self) -> Result<String, Self::Error>
	{
		let c_str: &CStr = self.as_ref();
		Ok(c_str.to_str()?.to_string())
	}
}

impl TryFrom<[c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]> for ObjectName
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn from(value: [c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]) -> Result<Self, Self::Error>
	{
		let length_including_ascii_nul = match Self::index_of_ascii_nul_c_char(&value[..])
		{
			Some(index) => index + 1,
			None => return Err(ObjectNameFromBytesError::DoesNotEndWithAsciiNull),
		};
		
		let array_vec = ConstArrayVec
		{
			xs: value,
			len: length_including_ascii_nul
		};
		
		Ok(unsafe { transmute(array_vec) })
	}
}

impl<'a> TryFrom<&'a [c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]> for ObjectName
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: &'a [c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]) -> Result<Self, Self::Error>
	{
		let length_including_ascii_nul = match Self::index_of_ascii_nul_c_char(&value[..])
		{
			Some(index) => index + 1,
			None => return Err(ObjectNameFromBytesError::DoesNotEndWithAsciiNull),
		};
		
		let mut array_vec = ArrayVec::new();
		
		unsafe
		{
			let pointer= array_vec.as_mut_ptr();
			pointer.copy_from_nonoverlapping(value.as_ptr(), length_including_ascii_nul);
			array_vec.set_len(length_including_ascii_nul)
		}
		Self(array_vec)
	}
}

impl<'a> TryFrom<&'a [u8]> for ObjectName
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
	{
		Self::from_bytes(value)
	}
}

impl<'a> TryFrom<&'a [c_char]> for ObjectName
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: &'a [c_char]) -> Result<Self, Self::Error>
	{
		Self::from_bytes(unsafe { transmute(value) })
	}
}

impl TryFrom<ArrayVec<[c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]>> for ObjectName
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn try_from(value: ArrayVec<[c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]>) -> Result<Self, Self::Error>
	{
		use self::ObjectNameFromBytesError::*;
		
		let length_including_ascii_nul = value.len();
		
		if unlikely!(length_excluding_ascii_nul == 0)
		{
			return Err(Empty)
		}
		
		let expected_ascii_nul = unsafe { *value.get_unchecked(length_including_ascii_nul - 1) };
		if unlikely!(expected_ascii_nul != Self::AsciiNull)
		{
			return Err(DoesNotEndWithAsciiNull)
		}
		
		Ok(Self(value))
	}
}

impl FromBytes for ObjectName
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		use self::ObjectNameFromBytesError::*;
		
		let length = bytes.len();
		
		if unlikely!(length > Self::MaximumObjectNameLengthExcludingAsciiNull)
		{
			return Err(TooLong(length))
		}
		
		if Self::index_of_ascii_nul(haystack).is_some()
		{
			return Err(ContainsAsciiNullWhereItShouldNot)
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

impl Deref for ObjectName
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

impl AsRef<ArrayVec<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]>> for ObjectName
{
	#[inline(always)]
	fn as_ref(&self) -> &ArrayVec<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]>
	{
		&self.0
	}
}

impl AsRef<CStr> for ObjectName
{
	#[inline(always)]
	fn as_ref(&self) -> &CStr
	{
		unsafe { CStr::from_bytes_with_nul_unchecked(self.0.as_slice()) }
	}
}

impl AsRef<[c_char]> for ObjectName
{
	#[inline(always)]
	fn as_ref(&self) -> &[c_char]
	{
		self.deref()
	}
}

impl Borrow<ArrayVec<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]>> for ObjectName
{
	#[inline(always)]
	fn as_ref(&self) -> &ArrayVec<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]>
	{
		self.as_ref()
	}
}

impl Borrow<CStr> for ObjectName
{
	#[inline(always)]
	fn borrow(&self) -> &CStr
	{
		self.as_ref()
	}
}

impl Borrow<[c_char]> for ObjectName
{
	#[inline(always)]
	fn borrow(&self) -> &[c_char]
	{
		self.as_ref()
	}
}

impl ObjectName
{
	const AsciiNull: c_char = b'\0' as i8;
	
	/// Maximum object name length.
	pub const MaximumObjectNameLengthExcludingAsciiNull: usize = 15;
	
	/// Maximum object name length (including trailing ASCII NULL).
	pub const MaximumObjectNameLengthIncludingAsciiNull: usize = Self::MaximumObjectNameLengthExcludingAsciiNull + 1;
	
	/// Construct from a C-like function call.
	///
	/// Provides the C-like function with a correctly sized buffer.
	#[inline(always)]
	pub fn construct_from_c_function_call<Error>(c_function: impl FnOnce(&mut [c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]) -> Result<(), Error>, does_not_end_with_ascii_nul_error: impl FnOnce() -> Error) -> Result<Self, Error>
	{
		let mut buffer: ConstArrayVec<[c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]> = unsafe { transmute(ArrayVec::<[c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]>::new()) };
		c_function(&mut buffer.xs)?;
		
		let index = match Self::index_of_ascii_nul_c_char(&buffer.xs[..])
		{
			Some(index) => index,
			None => return Err(does_not_end_with_ascii_nul_error)
		};
		
		let buffer: ArrayVec::<[u8; Self::MaximumObjectNameLengthIncludingAsciiNull]> = unsafe { transmute(buffer) };
		unsafe { buffer.set_len(index + 1) };
		Self(buffer)
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
		file_path.write_value(&self[..])
	}
	
	#[inline(always)]
	fn to_object_name(&self) -> [c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]
	{
		self.debug_assertions();
		
		let buffer = &self.0;
		Self::transmute_array_vec(buffer.clone())
	}
	
	#[inline(always)]
	fn into_object_name(self) -> [c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]
	{
		self.debug_assertions();
		
		let buffer = self.0;
		Self::transmute_array_vec(buffer)
	}
	
	#[inline(always)]
	fn debug_assertions()
	{
		debug_assert_eq!(TASK_COMM_LEN, Self::MaximumObjectNameLengthIncludingAsciiNull);
		debug_assert_eq!(BPF_OBJ_NAME_LEN, Self::MaximumObjectNameLengthIncludingAsciiNull);
		debug_assert_eq!(IFNAMSIZ, Self::MaximumObjectNameLengthIncludingAsciiNull);
		debug_assert_eq!(IF_NAMESIZE, Self::MaximumObjectNameLengthIncludingAsciiNull);
	}
	
	#[inline(always)]
	fn transmute_array_vec(buffer: ArrayVec<[c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]>) -> [c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]
	{
		let const_array_vec: ConstArrayVec<[c_char; Self::MaximumObjectNameLengthIncludingAsciiNull]> = unsafe { transmute(array_vec) };
		const_array_vec.xs
	}
	
	#[inline(always)]
	fn index_of_ascii_nul_c_char(haystack: &[c_char]) -> Option<usize>
	{
		Self::index_of_ascii_nul(unsafe { transmute(haystack) })
	}
	
	#[inline(always)]
	fn index_of_ascii_nul(haystack: &[u8]) -> Option<usize>
	{
		const AsciiNull: u8 = Self::AsciiNull as u8;
		
		memchr(AsciiNull, haystack)
	}
}
