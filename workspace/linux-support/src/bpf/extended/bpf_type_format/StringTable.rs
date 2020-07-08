// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct StringTable
{
	deduplication: HashMap<String, u32>,
	string_table: Vec<u8>,
}

impl StringTable
{
	/// Push any valid as long as it does not contain an embedded ASCII NULL.
	#[inline(always)]
	pub(crate) fn push_any(&mut self, any: &str) -> Result<Option<NonZeroU32>, BtfTypeError>
	{
		self.push(any, |any| Self::to_ascii_null_terminated_bytes(any))
	}
	
	/// `ident` is validated as a c identifier, with substitution of `_` (underscore) for invalid characters.
	///
	/// If it is an `ident` has a `kind` of `BtfKind::Section` then a period (dot) is also permitted.
	#[inline(always)]
	pub(crate) fn push_c_identifier(&mut self, ident: &str, kind: BtfKind) -> Result<Option<NonZeroU32>, BtfTypeError>
	{
		self.push(ident, |ident| Self::convert_to_valid_c_identifier(ident, kind))
	}
	
	#[inline(always)]
	fn push(&mut self, value: &str, to_bytes: impl FnOnce(&str) -> Result<Vec<u8>, BtfTypeError>) -> Result<Option<NonZeroU32>, BtfTypeError>
	{
		use self::BtfTypeError::*;
		
		if let Some(offset) = self.deduplication.get(value)
		{
			return Ok(NonZeroU32::new(*offset))
		}
		
		let bytes = to_bytes(value);
		
		debug_assert!(BTF_MAX_NAME_OFFSET <= u32::MAX);
		let raw_offset = self.deduplication.len();
		if unlikely!(raw_offset > (BTF_MAX_NAME_OFFSET as usize))
		{
			return Err(StringTableOffsetIsTooLarge)
		}
		let offset = raw_offset as u32;
		
		let end_offset = (offset as u64).checked_add(bytes.len() as u64).ok_or(IdentifierIsTooLarge)?;
		if unlikely!(end_offset > (BTF_MAX_NAME_OFFSET as u64))
		{
			return Err(IdentifierIsTooLarge)
		}
		
		self.string_table.extend_from_slice(&bytes[..]);
		
		self.deduplication.insert(value.to_string(), offset);
		
		Ok(NonZeroU32::new(offset))
	}
	
	// Logic based on the Linux kernel function `__btf_name_valid()`.
	#[inline(always)]
	fn convert_to_valid_c_identifier(ident: &str, kind: BtfKind) -> Result<Vec<u8>, BtfTypeError>
	{
		use self::BtfTypeError::*;
		
		if unlikely!(ident.is_empty())
		{
			return Err(IdentifierIsEmpty);
		}
		
		let period_is_permitted = kind == BtfKind::Section;
		
		let mut bytes = Self::to_ascii_null_terminated_bytes(ident)?;
		
		let bytes_length = bytes.len();
		const KSYM_NAME_LEN: usize = 128;
		if unlikely!(bytes_length > KSYM_NAME_LEN)
		{
			return Err(IdentifierIsMoreThan127BytesLong);
		}
		
		for index in 0 .. (bytes_length - 1)
		{
			let byte = unsafe { bytes.get_unchecked_mut(index) };
			let is_valid = match *byte
			{
				b'A' ..= b'Z' => true,
				b'a' ..= b'z' => true,
				b'0' ..= b'9' => index != 0,
				b'_' => true,
				b'.' => period_is_permitted,
				_ => false,
			};
			if unlikely!(!is_valid)
			{
				*byte = b'_';
			}
		}
		
		Ok(bytes)
	}
	
	#[inline(always)]
	fn to_ascii_null_terminated_bytes(value: &str) -> Result<Vec<u8>, BtfTypeError>
	{
		match CString::new(value.as_bytes())
		{
			Ok(c_string) => Ok(c_string.into_bytes_with_nul.to_vec()),
			Err(cause) => Err(BtfTypeError::IdentifierContainsAsciiNul(cause)),
		}
	}
}
