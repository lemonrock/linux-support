// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A case-folded (normalized to lower case) label.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CaseFoldedLabel<'a>(Cow<'a, [u8]>);

impl<'a> TryFrom<Box<[u8]>> for CaseFoldedLabel<'a>
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: Box<[u8]>) -> Result<Self, Self::Result>
	{
		Self::validate_bytes(&value[..]);
		Self(Cow::Owned(value.to_vec()))
	}
}

impl<'a> TryFrom<Vec<u8>> for CaseFoldedLabel<'a>
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: Vec<u8>) -> Result<Self, Self::Result>
	{
		Self::validate_bytes(&value[..]);
		Self(Cow::Owned(value))
	}
}

impl<'a> TryFrom<&'a [u8]> for CaseFoldedLabel<'a>
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Result>
	{
		Self::validate_bytes(value);
		Self(Cow::Borrowed(value))
	}
}

/// This clones the underlying data and case-folds.
impl<'a, 'message> From<ParsedLabel<'message>> for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn from(value: ParsedLabel<'message>) -> Self
	{
		let length = value.len();
		let capacity = length as usize;
		let mut case_folded_bytes = Vec::with_capacity(capacity);
		
		for index in 0 .. length
		{
			unsafe { *(case_folded_bytes.as_mut_ptr().add(index as usize)) = value.get_unchecked_case_folded_byte(index) };
		}
		unsafe { case_folded_bytes.set_len(capacity) }
		Self(Cow::Owned(case_folded_bytes))
	}
}

impl<'a> PartialEq for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.equals(rhs)
	}
}

impl<'a> Eq for CaseFoldedLabel<'a>
{
}

impl<'a> PartialOrd for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl<'a> Ord for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.compare(rhs)
	}
}

impl<'a> Hash for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.hash_slice(state)
	}
}

impl<'a, 'message> PartialEq<ParsedLabel<'message>> for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn eq(&self, rhs: &ParsedLabel<'message>) -> bool
	{
		self.equals(rhs)
	}
}

impl<'a, 'message> PartialOrd<ParsedLabel<'message>> for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &ParsedLabel<'message>) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl<'a> Label<'a> for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn len(&self) -> u8
	{
		self.0.len() as u8
	}
	
	#[inline(always)]
	fn get_unchecked_case_folded_byte(&self, index: u8) -> u8
	{
		*self.get_unchecked(index)
	}
	
	#[inline(always)]
	fn get_unchecked(&self, index: u8) -> &u8
	{
		unsafe { self.0.get_unchecked(index as usize) }
	}
}

impl<'a> CaseFoldedLabel<'a>
{
	const MaximumSize: usize = 63;
	
	/// Clones and case folds.
	#[inline(always)]
	pub fn new_cloned_and_case_folded(value: &[u8]) -> Result<Self, ParseNumberError>
	{
		let length = value.len();
		
		if unlikely!(length > Self::MaximumSize)
		{
			return Err(ParseNumberError::TooLarge)
		}
		
		let mut bytes = value.to_vec();
		for byte in bytes.iter_mut()
		{
			*byte = case_fold_byte(byte)
		}
		Ok(Self(Cow::Owned(bytes)))
	}
	
	#[inline(always)]
	fn validate_bytes(slice: &[u8]) -> Result<(), ParseNumberError>
	{
		let length = value.len();
		
		if unlikely!(length > Self::MaximumSize)
		{
			return Err(ParseNumberError::TooLarge)
		}
		
		for index in 0 .. length
		{
			Self::validate_byte(unsafe { value.get_unchecked(index) })
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn validate_byte(byte: &u8) -> Result<(), ParseNumberError>
	{
		if unlikely!(Self::is_upper_case(byte))
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn is_upper_case(byte: &u8) -> bool
	{
		let byte = *byte;
		byte >= b'A' || byte <= b'Z'
	}
}
