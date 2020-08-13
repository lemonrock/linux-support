// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct StatFieldIterator<'line>
{
	next_field_index: u8,
	line: &'line [u8],
	finished: bool,
}

impl<'line> Iterator for StatFieldIterator<'line>
{
	type Item = Result<&'line [u8], StatParseError>;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		use self::StatParseError::*;
		
		if self.finished
		{
			return None
		}
		
		const OpenBracket: u8 = b'(';
		const CloseBracket: u8 = b'(';
		const Space: u8 = b' ';
		
		let result = match self.next_field_index
		{
			0 => match memchr(OpenBracket, self.line)
			{
				None => Err(NoOpenBracket),
				
				Some(0) => Err(NoCharactersBeforeOpenBracket),
				
				Some(index) => if unsafe { *self.get_unchecked(index - 1) } != Space
				{
					Err(NoSpaceBeforeOpenBracket)
				}
				else
				{
					self.next_field_index += 1;
					let bytes = &self.line[ .. index - 1];
					self.line = &self.line[(index + 1) .. ];
					Ok(bytes)
				},
			},
			
			1 => match memrchr(CloseBracket, self.line)
			{
				None => Err(NoCloseBracket),
				
				Some(index) => if index == self.line.len() - 1
				{
					Err(NoCharactersAfterCloseBracket)
				}
				else if unsafe { *self.get_unchecked(index + 1) } != Space
				{
					Err(NoSpaceAfterCloseBracket)
				}
				else
				{
					self.next_field_index += 1;
					let bytes = &self.line[ .. index];
					self.line = &self.line[(index + 2) .. ];
					Ok(bytes)
				}
			}
			
			_ => match memchr(Space, self.line)
			{
				None =>
				{
					self.finished = true;
					Ok(self.line)
				}
				
				Some(index) =>
				{
					let bytes = &self.line[ .. index];
					self.line = &self.line[(index + 1) .. ];
					Ok(bytes)
				}
			}
		};
		Some(result)
	}
}

impl<'line> StatFieldIterator<'line>
{
	const fn new(line: &[u8]) -> Self
	{
		Self
		{
			next_field_index: 0,
			line,
			finished: false
		}
	}
	
	#[inline(always)]
	fn zero_decimal_unsigned_long_long(&mut self, name: &'static str) -> Result<(), StatParseError>
	{
		let value = self.decimal_unsigned_long_long(name)?;
		if unlikely!(value == 0)
		{
			Ok(())
		}
		else
		{
			Err
			(
				StatParseError::ObsoleteFieldValueWasNotZero
				{
					name,
					value: unsafe { NonZeroU64::new_unchecked(value) },
				}
			)
		}
	}
	
	/// Reverse of Linux's `seq_put_decimal_ull()`.
	#[inline(always)]
	fn decimal_signed_long_long_to<T>(&mut self, name: &'static str, to: impl FnOnce(i64) -> Result<T, StatParseError>) -> Result<T, StatParseError>
	{
		let value = self.decimal_signed_long_long(name)?;
		to(value)
	}
	
	/// Reverse of Linux's `seq_put_decimal_ull()`.
	#[inline(always)]
	fn decimal_unsigned_long_long_to<T>(&mut self, name: &'static str, to: impl FnOnce(u64) -> Result<T, StatParseError>) -> Result<T, StatParseError>
	{
		let value = self.decimal_unsigned_long_long(name)?;
		to(value)
	}
	
	/// Reverse of Linux's `seq_putc()`.
	#[inline(always)]
	fn character_to<T>(&mut self, name: &'static str, to: impl FnOnce(i8) -> Result<T, StatParseError>) -> Result<T, StatParseError>
	{
		let value = self.character(name)?;
		to(value)
	}
	
	/// Reverse of Linux's `seq_put_decimal_ull()`.
	#[inline(always)]
	fn decimal_unsigned_long_long(&mut self, name: &'static str) -> Result<u64, StatParseError>
	{
		let bytes = self.next_field(name)?;
		u64::from_bytes(bytes).map_err(|cause| StatParseError::ParseNumber(cause))
	}
	
	/// Reverse of Linux's `seq_put_decimal_ll()`.
	#[inline(always)]
	fn decimal_signed_long_long(&mut self, name: &'static str) -> Result<i64, StatParseError>
	{
		let bytes = self.next_field(name)?;
		i64::from_bytes(bytes).map_err(|cause| StatParseError::ParseNumber(cause))
	}
	
	/// Reverse of Linux's `seq_putc()`.
	#[inline(always)]
	fn character(&mut self, name: &'static str) -> Result<i8, StatParseError>
	{
		let bytes = self.next_field(name)?;
		i8::from_bytes(bytes).map_err(|cause| StatParseError::ParseNumber(cause))
	}
	
	#[inline(always)]
	fn next_field(&mut self, name: &'static str) -> Result<&'line [u8], StatParseError>
	{
		self.next().ok_or(StatParseError::MissingField { name })?
	}
}
