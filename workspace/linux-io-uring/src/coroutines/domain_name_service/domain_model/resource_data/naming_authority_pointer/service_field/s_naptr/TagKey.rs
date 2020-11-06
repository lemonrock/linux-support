// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A tag key.
#[derive(Copy, Clone)]
pub struct TagKey
{
	length: NonZeroU8,
	
	lower_case_bytes: [u8; TagKey::MaximumSize],
}

impl Display for TagKey
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, &self.lower_case_bytes[0 .. self.length()])
	}
}

impl Debug for TagKey
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, &self.lower_case_bytes[0 .. self.length()])
	}
}

impl PartialEq for TagKey
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		if self.length != other.length
		{
			return false
		}
		
		for index in 0 .. self.length()
		{
			let left_byte = self.byte_at_unchecked(index);
			let right_byte = other.byte_at_unchecked(index);
			if left_byte.ne(right_byte)
			{
				return false
			}
		}
		
		true
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		if self.length != other.length
		{
			return true
		}
		
		for index in 0 .. self.length.get() as usize
		{
			let left_byte = self.byte_at_unchecked(index);
			let right_byte = other.byte_at_unchecked(index);
			if left_byte.ne(right_byte)
			{
				return true
			}
		}
		
		false
	}
}

impl Eq for TagKey
{
}

impl PartialOrd for TagKey
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for TagKey
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		use self::Ordering::*;
		
		match self.length.cmp(&other.length)
		{
			Less => return Less,
			Equal => (),
			Greater => return Greater,
		};
		
		
		for index in 0 .. self.length()
		{
			let left_byte = self.byte_at_unchecked(index);
			let right_byte = other.byte_at_unchecked(index);
			match left_byte.cmp(right_byte)
			{
				Less => return Less,
				Equal => continue,
				Greater => return Greater,
			}
		}
		
		Equal
	}
}

impl Hash for TagKey
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		for index in 0 .. self.length()
		{
			let byte = self.byte_at_unchecked(index);
			byte.hash(state)
		}
	}
}

impl TagKey
{
	const MaximumSize: usize = 32;
	
	#[allow(deprecated)]
	#[inline(always)]
	pub(crate) fn parse(value: &'static [u8]) -> Result<Self, TagKeyParseError>
	{
		use self::TagKeyParseError::*;
		
		let value_length = value.len();
		
		let length =
		{
			if unlikely!(value_length == 0)
			{
				return Err(CanNotBeEmpty)
			}
			
			if unlikely!(value_length > Self::MaximumSize)
			{
				return Err(CanNotExceed32Bytes(value_length))
			}
			
			debug_assert!(value_length <= (u8::MAX as usize));
			unsafe { NonZeroU8::new_unchecked(value_length as u8) }
		};
		
		Ok
		(
			Self
			{
				length,
				
				lower_case_bytes:
				{
					let mut lower_case_bytes: [u8; TagKey::MaximumSize] = unsafe { uninitialized() };
					let lower_case_bytes_mut_ptr = lower_case_bytes.as_mut_ptr();
					
					let first_byte = unsafe { * value.get_unchecked(0) };
					let first_byte = match first_byte
					{
						b'A' ..= b'Z' => case_fold_upper_case_byte_to_lower_case_byte(first_byte),
						b'a' ..= b'z' => first_byte,
						b'0' ..= b'9' => return Err(FirstByteCanNotBeNumeric(first_byte)),
						b'+' | b'-' | b'.' => return Err(FirstByteCanNotBeSymbol(first_byte)),
						_ => return Err(FirstByteOutOfRange(first_byte)),
					};
					unsafe { lower_case_bytes_mut_ptr.write(first_byte) }
					
					
					for index in 1 .. value_length
					{
						let subsequent_byte = unsafe { * value.get_unchecked(index) };
						let subsequent_byte = match subsequent_byte
						{
							b'A' ..= b'Z' => case_fold_upper_case_byte_to_lower_case_byte(subsequent_byte),
							b'a' ..= b'z' => subsequent_byte,
							b'0' ..= b'9' => subsequent_byte,
							b'+' | b'-' | b'.' => subsequent_byte,
							_ => return Err(SubsequentByteOutOfRange(subsequent_byte, unsafe { NonZeroU8::new_unchecked(index as u8) })),
						};
						unsafe { lower_case_bytes_mut_ptr.add(index).write(subsequent_byte) }
					}
					
					lower_case_bytes
				},
			}
		)
	}
	
	const ExperimentalPrefix: &'static [u8] = b"x-";
	
	const ExperimentalPrefixLength: usize = Self::ExperimentalPrefix.len();
	
	/// Returns the portion after the experimental prefix.
	///
	/// Will never be `Some(empty)`.
	#[inline(always)]
	pub(crate) fn is_experimental(&self) -> bool
	{
		// Experimental names can not be empty.
		if self.length() >= (Self::ExperimentalPrefixLength + 1)
		{
			&self.lower_case_bytes[0 .. Self::ExperimentalPrefixLength] == Self::ExperimentalPrefix
		}
		else
		{
			false
		}
	}
	
	/// Returns the portion after the experimental prefix.
	///
	/// Will never be `Some(empty)`.
	#[inline(always)]
	pub fn experimental(&self) -> Option<&[u8]>
	{
		if self.is_experimental()
		{
			Some(&self.lower_case_bytes[Self::ExperimentalPrefixLength .. ])
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.length.get() as usize
	}
	
	#[inline(always)]
	fn byte_at_unchecked(&self, index: usize) -> &u8
	{
		unsafe { self.lower_case_bytes.get_unchecked(index) }
	}
}
