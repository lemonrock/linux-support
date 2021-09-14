// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A trait to make it more efficient to write UTF-8 characters.
pub trait ByteWritable
{
	/// Failure to reserve.
	type ReservationError: error::Error;
	
	/// Requests a reservation before one or more calls to `push_byte()`.
	///
	/// Default implementation does nothing.
	#[inline(always)]
	fn reserve_exact_before_push_byte(&mut self, length: usize) -> Result<(), Self::ReservationError>
	{
		let _ = length;
		Ok(())
	}
	
	/// Pushes a byte; does not have to do a check.
	fn push_byte(&mut self, byte: u8);
	
	/// More efficient than Rust's stdlib implementation with a bias to shorter UTF-8 sequences.
	#[inline(always)]
	fn encode_utf8_raw(&mut self, character: char) -> Result<usize, Self::ReservationError>
	{
		const TAG_CONT: u8 = 0b1000_0000;
		const TAG_TWO_B: u8 = 0b1100_0000;
		const TAG_THREE_B: u8 = 0b1110_0000;
		const TAG_FOUR_B: u8 = 0b1111_0000;
		
		let code = character as u32;
		if likely!(code < 0x80)
		{
			const Size: usize = 1;
			self.reserve_exact_before_push_byte(Size)?;
			self.push_byte(code as u8);
			Ok(Size)
		}
		else if likely!(code < 0x800)
		{
			const Size: usize = 2;
			self.reserve_exact_before_push_byte(Size)?;
			self.push_byte((code >> 6 & 0x1F) as u8 | TAG_TWO_B);
			self.push_byte((code & 0x3F) as u8 | TAG_CONT);
			Ok(Size)
		}
		else if likely!(code < 0x10000)
		{
			const Size: usize = 3;
			self.reserve_exact_before_push_byte(Size)?;
			self.push_byte((code >> 12 & 0x0F) as u8 | TAG_THREE_B);
			self.push_byte((code >> 6 & 0x3F) as u8 | TAG_CONT);
			self.push_byte((code & 0x3F) as u8 | TAG_CONT);
			Ok(Size)
		}
		else
		{
			const Size: usize = 4;
			self.reserve_exact_before_push_byte(Size)?;
			self.push_byte((code >> 18 & 0x07) as u8 | TAG_FOUR_B);
			self.push_byte((code >> 12 & 0x3F) as u8 | TAG_CONT);
			self.push_byte((code >> 6 & 0x3F) as u8 | TAG_CONT);
			self.push_byte((code & 0x3F) as u8 | TAG_CONT);
			Ok(Size)
		}
	}
	
}

impl ByteWritable for String
{
	type ReservationError = TryReserveError;
	
	#[inline(always)]
	fn reserve_exact_before_push_byte(&mut self, length: usize) -> Result<(), Self::ReservationError>
	{
		self.try_reserve_exact(length)
	}
	
	#[inline(always)]
	fn push_byte(&mut self, byte: u8)
	{
		(unsafe { self.as_mut_vec() }).push_byte(byte)
	}
}

impl ByteWritable for Vec<u8>
{
	type ReservationError = TryReserveError;
	
	#[inline(always)]
	fn reserve_exact_before_push_byte(&mut self, length: usize) -> Result<(), Self::ReservationError>
	{
		self.try_reserve_exact(length)
	}
	
	#[inline(always)]
	fn push_byte(&mut self, byte: u8)
	{
		self.push_unchecked(byte)
	}
}

impl<const CAP: usize> ByteWritable for ArrayVec<u8, CAP>
{
	type ReservationError = CapacityError;
	
	#[inline(always)]
	fn reserve_exact_before_push_byte(&mut self, length: usize) -> Result<(), Self::ReservationError>
	{
		if unlikely!(self.remaining_capacity() < length)
		{
			Err(CapacityError::new(()))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn push_byte(&mut self, byte: u8)
	{
		unsafe { self.push_unchecked(byte) }
	}
}

impl<'a> ByteWritable for &'a mut [u8]
{
	type ReservationError = CapacityError;
	
	#[inline(always)]
	fn reserve_exact_before_push_byte(&mut self, length: usize) -> Result<(), Self::ReservationError>
	{
		let remaining_capacity = self.len();
		if unlikely!(remaining_capacity < length)
		{
			Err(CapacityError::new(()))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn push_byte(&mut self, byte: u8)
	{
		*self.get_unchecked_mut_safe(0) = byte;
		
		let pointer = self.as_mut_ptr();
		let new_pointer = unsafe { pointer.add(1) };
		let new_length = self.len() - 1;
		*self = unsafe { from_raw_parts_mut(new_pointer, new_length) };
	}
}
