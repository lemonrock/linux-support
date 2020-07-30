// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! object_name
{
	($name: ident, $MaximumLengthExcludingAsciiNull: expr, $(#[$documentation: meta])*) =>
	{
		$(#[$documentation])*
		///
		/// Assumes that the final byte is always ASCII NULL, ie the maximum `strnlen()` is `MaximumLengthExcludingAsciiNull`.
		///
		/// Deref excludes trailing ASCII NULL.
		#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[repr(transparent)]
		pub struct $name(pub(crate) ArrayVec<[c_char; <$name>::MaximumLengthIncludingAsciiNull]>);
		
		impl<'de> Deserialize<'de> for $name
		{
			#[inline(always)]
			fn deserialize<D:  Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
			{
				struct ObjectNameVisitor;
				
				impl<'de> Visitor<'de> for ObjectNameVisitor
				{
					type Value = $name;
					
					#[inline(always)]
					fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
					{
						formatter.write_str(stringify!("A C-like string with a length between 0 and ", $MaximumLengthIncludingAsciiNull, " inclusive without a trailing NULL"))
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
						$name::from_bytes(v).map_err(|error| match error
						{
							TooLong(size) => E::invalid_length(size, &stringify!("Too long; length is a maximum inclusive value of ", $MaximumLengthIncludingAsciiNull)),
							ContainsAsciiNullWhereItShouldNot => E::invalid_value(Unexpected::Bytes(v), &"ContainsAsciiNullWhereItShouldNot"),
							DoesNotEndWithAsciiNull => E::invalid_value(Unexpected::Bytes(v), &"DoesNotEndWithAsciiNull"),
							Empty => E::invalid_length(0, &stringify!("Empty; length is a maximum inclusive value of ", $MaximumLengthIncludingAsciiNull)),
						})
					}
				}
				
				deserializer.deserialize_bytes(ObjectNameVisitor)
			}
		}
		
		impl Serialize for $name
		{
			#[inline(always)]
			fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
			{
				serializer.serialize_bytes(unsafe { transmute(self.deref()) })
			}
		}
		
		impl Display for $name
		{
			#[inline(always)]
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
			{
				let c_str: &CStr = self.as_ref();
				write!(f, "{}", c_str.to_string_lossy())
			}
		}
		
		impl Debug for $name
		{
			#[inline(always)]
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
			{
				let c_str: &CStr = self.as_ref();
				write!(f, stringify!($name, "({:?})"), c_str)
			}
		}
		
		impl Into<ArrayVec<[c_char; <$name>::MaximumLengthIncludingAsciiNull]>> for $name
		{
			#[inline(always)]
			fn into(self) -> ArrayVec<[c_char; <$name>::MaximumLengthIncludingAsciiNull]>
			{
				self.0
			}
		}
		
		impl<'a> Into<[c_char; <$name>::MaximumLengthIncludingAsciiNull]> for &'a $name
		{
			#[inline(always)]
			fn into(self) -> [c_char; <$name>::MaximumLengthIncludingAsciiNull]
			{
				self.to_object_name()
			}
		}
		
		impl Into<[c_char; <$name>::MaximumLengthIncludingAsciiNull]> for $name
		{
			#[inline(always)]
			fn into(self) -> [c_char; <$name>::MaximumLengthIncludingAsciiNull]
			{
				self.into_object_name()
			}
		}
		
		impl TryInto<String> for $name
		{
			type Error = Utf8Error;
			
			#[inline(always)]
			fn try_into(self) -> Result<String, Self::Error>
			{
				let c_str: &CStr = self.as_ref();
				Ok(c_str.to_str()?.to_string())
			}
		}
		
		impl<'a> TryFrom<&'a Self> for $name
		{
			type Error = Infallible;
			
			#[inline(always)]
			fn try_from(value: &'a Self) -> Result<Self, Self::Error>
			{
				Ok(value.clone())
			}
		}
		
		impl TryFrom<CString> for $name
		{
			type Error = ObjectNameFromBytesError;
			
			#[inline(always)]
			fn try_from(value: CString) -> Result<Self, Self::Error>
			{
				Self::from_bytes(value.as_bytes())
			}
		}
		
		impl<'a> TryFrom<&'a CStr> for $name
		{
			type Error = ObjectNameFromBytesError;
			
			#[inline(always)]
			fn try_from(value: &'a CStr) -> Result<Self, Self::Error>
			{
				Self::from_bytes(value.to_bytes())
			}
		}
		
		impl<'a> TryFrom<String> for $name
		{
			type Error = ObjectNameFromBytesError;
			
			#[inline(always)]
			fn try_from(value: String) -> Result<Self, Self::Error>
			{
				Self::from_bytes(value.as_bytes())
			}
		}
		
		impl<'a> TryFrom<&'a str> for $name
		{
			type Error = ObjectNameFromBytesError;
			
			#[inline(always)]
			fn try_from(value: &'a str) -> Result<Self, Self::Error>
			{
				Self::from_bytes(value.as_bytes())
			}
		}
		
		impl<'a> TryFrom<&'a [u8]> for $name
		{
			type Error = ObjectNameFromBytesError;
			
			#[inline(always)]
			fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
			{
				Self::from_bytes(value)
			}
		}
		
		impl<'a> TryFrom<&'a [c_char]> for $name
		{
			type Error = ObjectNameFromBytesError;
			
			#[inline(always)]
			fn try_from(value: &'a [c_char]) -> Result<Self, Self::Error>
			{
				Self::from_bytes(unsafe { transmute(value) })
			}
		}
		
		impl TryFrom<[c_char; Self::MaximumLengthIncludingAsciiNull]> for $name
		{
			type Error = ObjectNameFromBytesError;
			
			#[inline(always)]
			fn try_from(value: [c_char; Self::MaximumLengthIncludingAsciiNull]) -> Result<Self, Self::Error>
			{
				let length_including_ascii_nul = match Self::index_of_ascii_null_c_char(&value[..])
				{
					Some(index) => index + 1,
					None => return Err(ObjectNameFromBytesError::DoesNotEndWithAsciiNull),
				};
				
				let array_vec = ConstArrayVec
				{
					xs: value,
					len: length_including_ascii_nul as u8,
				};
				
				Ok(unsafe { transmute(array_vec) })
			}
		}
		
		impl<'a> TryFrom<&'a [c_char; Self::MaximumLengthIncludingAsciiNull]> for $name
		{
			type Error = ObjectNameFromBytesError;
			
			#[inline(always)]
			fn try_from(value: &'a [c_char; Self::MaximumLengthIncludingAsciiNull]) -> Result<Self, Self::Error>
			{
				let length_including_ascii_nul = match Self::index_of_ascii_null_c_char(&value[..])
				{
					Some(index) => index + 1,
					None => return Err(ObjectNameFromBytesError::DoesNotEndWithAsciiNull),
				};
				
				let mut array_vec: ArrayVec<[c_char; Self::MaximumLengthIncludingAsciiNull]> = ArrayVec::new();
				
				unsafe
				{
					let pointer = array_vec.as_mut_ptr();
					pointer.copy_from_nonoverlapping(value.as_ptr(), length_including_ascii_nul);
					array_vec.set_len(length_including_ascii_nul)
				}
				Ok(Self(array_vec))
			}
		}
		
		impl TryFrom<ArrayVec<[c_char; Self::MaximumLengthIncludingAsciiNull]>> for $name
		{
			type Error = ObjectNameFromBytesError;
			
			#[inline(always)]
			fn try_from(value: ArrayVec<[c_char; Self::MaximumLengthIncludingAsciiNull]>) -> Result<Self, Self::Error>
			{
				let length_including_ascii_null = value.len();
				
				if unlikely!(length_including_ascii_null == 0)
				{
					return Err(ObjectNameFromBytesError::Empty)
				}
				
				let expected_ascii_null = unsafe { *value.get_unchecked(length_including_ascii_null - 1) };
				if unlikely!(expected_ascii_null != Self::AsciiNull)
				{
					return Err(ObjectNameFromBytesError::DoesNotEndWithAsciiNull)
				}
				
				Ok(Self(value))
			}
		}
		
		impl FromBytes for $name
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
		
		impl Deref for $name
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
		
		impl AsRef<ArrayVec<[c_char; <$name>::MaximumLengthIncludingAsciiNull]>> for $name
		{
			#[inline(always)]
			fn as_ref(&self) -> &ArrayVec<[c_char; <$name>::MaximumLengthIncludingAsciiNull]>
			{
				&self.0
			}
		}
		
		impl AsRef<CStr> for $name
		{
			#[inline(always)]
			fn as_ref(&self) -> &CStr
			{
				unsafe { CStr::from_bytes_with_nul_unchecked(unsafe { transmute(self.0.as_slice()) }) }
			}
		}
		
		impl AsRef<[c_char]> for $name
		{
			#[inline(always)]
			fn as_ref(&self) -> &[c_char]
			{
				self.deref()
			}
		}
		
		impl AsRef<[u8]> for $name
		{
			#[inline(always)]
			fn as_ref(&self) -> &[u8]
			{
				unsafe { transmute(self.deref()) }
			}
		}
		
		impl Borrow<ArrayVec<[c_char; <$name>::MaximumLengthIncludingAsciiNull]>> for $name
		{
			#[inline(always)]
			fn borrow(&self) -> &ArrayVec<[c_char; <$name>::MaximumLengthIncludingAsciiNull]>
			{
				self.as_ref()
			}
		}
		
		impl Borrow<CStr> for $name
		{
			#[inline(always)]
			fn borrow(&self) -> &CStr
			{
				self.as_ref()
			}
		}
		
		impl Borrow<[c_char]> for $name
		{
			#[inline(always)]
			fn borrow(&self) -> &[c_char]
			{
				self.as_ref()
			}
		}
		
		impl Borrow<[u8]> for $name
		{
			#[inline(always)]
			fn borrow(&self) -> &[u8]
			{
				self.as_ref()
			}
		}
		
		impl PartialEq<[c_char; Self::MaximumLengthIncludingAsciiNull]> for $name
		{
			#[inline(always)]
			fn eq(&self, other: &[c_char; Self::MaximumLengthIncludingAsciiNull]) -> bool
			{
				let index = match Self::index_of_ascii_null(other)
				{
					None => return false,
					Some(index) => index,
				};
				
				self.deref() == &other[0 .. index]
			}
		}
		
		impl $name
		{
			const AsciiNull: c_char = b'\0' as i8;
			
			/// Maximum object name length.
			pub const MaximumLengthExcludingAsciiNull: usize = $MaximumLengthExcludingAsciiNull;
			
			/// Maximum object name length (including trailing ASCII NULL).
			pub const MaximumLengthIncludingAsciiNull: usize = Self::MaximumLengthExcludingAsciiNull + 1;
			
			/// Construct from a C-like function call.
			///
			/// Provides the C-like function with a correctly sized buffer.
			#[inline(always)]
			pub fn construct_from_c_function_call<Error>(c_function: impl FnOnce(&mut [c_char; Self::MaximumLengthIncludingAsciiNull]) -> Result<(), Error>, does_not_end_with_ascii_nul_error: impl FnOnce() -> Error) -> Result<Self, Error>
			{
				let mut buffer: ConstArrayVec<[c_char; Self::MaximumLengthIncludingAsciiNull]> = unsafe { transmute(ArrayVec::<[c_char; Self::MaximumLengthIncludingAsciiNull]>::new()) };
				c_function(&mut buffer.xs)?;
				
				let index = match Self::index_of_ascii_null_c_char(&buffer.xs[..])
				{
					Some(index) => index,
					None => return Err(does_not_end_with_ascii_nul_error())
				};
				
				let buffer: ArrayVec<[c_char; Self::MaximumLengthIncludingAsciiNull]> = unsafe { transmute(buffer) };
				unsafe { buffer.set_len(index + 1) };
				Ok(Self(buffer))
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
			fn to_object_name(&self) -> [c_char; Self::MaximumLengthIncludingAsciiNull]
			{
				let buffer = &self.0;
				Self::transmute_array_vec(buffer.clone())
			}
			
			#[inline(always)]
			fn into_object_name(self) -> [c_char; Self::MaximumLengthIncludingAsciiNull]
			{
				let buffer = self.0;
				Self::transmute_array_vec(buffer)
			}
			
			#[inline(always)]
			fn transmute_array_vec(buffer: ArrayVec<[c_char; Self::MaximumLengthIncludingAsciiNull]>) -> [c_char; Self::MaximumLengthIncludingAsciiNull]
			{
				let const_array_vec: ConstArrayVec<[c_char; Self::MaximumLengthIncludingAsciiNull]> = unsafe { transmute(buffer) };
				const_array_vec.xs
			}
			
			#[inline(always)]
			fn index_of_ascii_null_c_char(haystack: &[c_char]) -> Option<usize>
			{
				Self::index_of_ascii_null(unsafe { transmute(haystack) })
			}
			
			#[inline(always)]
			fn index_of_ascii_null(haystack: &[u8]) -> Option<usize>
			{
				const AsciiNull: u8 = <$name>::AsciiNull as u8;
				
				memchr(AsciiNull, haystack)
			}
		}
	}
}
