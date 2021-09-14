// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Converts to a decimal UTF-8 string of bytes.
pub trait NumberAsDecimalString: Copy
{
	#[doc(hidden)]
	type Unsigned: Copy + Rem<Output=Self::Unsigned> + DivAssign + Eq;
	
	#[doc(hidden)]
	const MaximumDigits: usize;
	
	#[doc(hidden)]
	const AdjustmentForSign: bool;
	
	#[doc(hidden)]
	const Maximum: usize = Self::MaximumDigits + (Self::AdjustmentForSign as usize);
	
	#[doc(hidden)]
	const UnsignedZero: Self::Unsigned;
	
	#[doc(hidden)]
	const UnsignedRadixTen: Self::Unsigned;
	
	#[doc(hidden)]
	fn is_negative(self) -> bool;
	
	#[doc(hidden)]
	fn into_unsigned(self) -> Self::Unsigned;
	
	#[doc(hidden)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8;
	
	/// Converts to a decimal UTF-8 string of bytes according to a format, `NADSF`.
	///
	/// Returns the number of bytes written.
	///
	/// Performant as only makes one reservation check, but can allocate more memory than is eventually needed.
	#[inline(always)]
	fn performant_to_decimal_utf8_string<NADSF: NumberAsDecimalStringFormat, BW: ByteWritable>(self, byte_writable: &mut BW) -> Result<usize, BW::ReservationError>
	{
		let mut unsafe_performant_byte_writable = UnsafePerformantByteWritable(byte_writable);
		unsafe_performant_byte_writable.reserve_exact_before_push_byte(NADSF::MaximumUtf8BytesPerDigit * Self::Maximum)?;
		self.to_decimal_utf8_string::<NADSF, _>(&mut unsafe_performant_byte_writable)
	}
	
	/// Converts to a decimal UTF-8 string of bytes according to a format, `NADSF`.
	///
	/// Returns the number of bytes written.
	fn to_decimal_utf8_string<NADSF: NumberAsDecimalStringFormat, BW: ByteWritable>(self, byte_writable: &mut BW) -> Result<usize, BW::ReservationError>
	{
		let mut total_written = 0;
		
		let mut x = self.into_unsigned();
		
		if Self::AdjustmentForSign
		{
			if self.is_negative()
			{
				total_written += byte_writable.encode_utf8_raw(NADSF::Negative)?;
			}
		}
		
		loop
		{
			let remainder = x % Self::UnsignedRadixTen;
			x /= Self::UnsignedRadixTen;
			
			let character = match Self::unsigned_into_u8(remainder)
			{
				0 => NADSF::Zero,
				
				1 => NADSF::One,
				
				2 => NADSF::Two,
				
				3 => NADSF::Three,
				
				4 => NADSF::Four,
				
				5 => NADSF::Five,
				
				6 => NADSF::Six,
				
				7 => NADSF::Seven,
				
				8 => NADSF::Eight,
				
				9 => NADSF::Nine,
				
				_ => unreachable_code_const("Radix is 10")
			};
			total_written += byte_writable.encode_utf8_raw(character)?;
			
			if x == Self::UnsignedZero
			{
				return Ok(total_written)
			}
		}
	}
}

const MaximumDigits8: usize = 3;

const MaximumDigits16: usize = 5;

const MaximumDigits32: usize = 10;

const MaximumDigits64: usize = 20;

const MaximumDigits128: usize = 39;

impl NumberAsDecimalString for u8
{
	type Unsigned = Self;
	
	const MaximumDigits: usize = MaximumDigits8;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned
	}
}

impl NumberAsDecimalString for u16
{
	type Unsigned = Self;
	
	const MaximumDigits: usize = MaximumDigits16;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for u32
{
	type Unsigned = Self;
	
	const MaximumDigits: usize = MaximumDigits32;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for u64
{
	type Unsigned = Self;
	
	const MaximumDigits: usize = MaximumDigits64;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for u128
{
	type Unsigned = Self;
	
	const MaximumDigits: usize = MaximumDigits128;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for usize
{
	type Unsigned = Self;
	
	#[cfg(target_pointer_width = "16")] const MaximumDigits: usize = MaximumDigits16;
	#[cfg(target_pointer_width = "32")] const MaximumDigits: usize = MaximumDigits32;
	#[cfg(target_pointer_width = "64")] const MaximumDigits: usize = MaximumDigits64;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for i8
{
	type Unsigned = u8;
	
	const MaximumDigits: usize = MaximumDigits8;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned
	}
}

impl NumberAsDecimalString for i16
{
	type Unsigned = u16;
	
	const MaximumDigits: usize = MaximumDigits16;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for i32
{
	type Unsigned = u32;
	
	const MaximumDigits: usize = MaximumDigits32;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for i64
{
	type Unsigned = u64;
	
	const MaximumDigits: usize = MaximumDigits64;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for i128
{
	type Unsigned = u128;
	
	const MaximumDigits: usize = MaximumDigits128;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for isize
{
	type Unsigned = usize;
	
	#[cfg(target_pointer_width = "16")] const MaximumDigits: usize = MaximumDigits16;
	#[cfg(target_pointer_width = "32")] const MaximumDigits: usize = MaximumDigits32;
	#[cfg(target_pointer_width = "64")] const MaximumDigits: usize = MaximumDigits64;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroU8
{
	type Unsigned = u8;
	
	const MaximumDigits: usize = MaximumDigits8;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroU16
{
	type Unsigned = u16;
	
	const MaximumDigits: usize = MaximumDigits16;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroU32
{
	type Unsigned = u32;
	
	const MaximumDigits: usize = MaximumDigits32;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroU64
{
	type Unsigned = u64;
	
	const MaximumDigits: usize = MaximumDigits64;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroU128
{
	type Unsigned = u128;
	
	const MaximumDigits: usize = MaximumDigits128;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroUsize
{
	type Unsigned = usize;
	
	#[cfg(target_pointer_width = "16")] const MaximumDigits: usize = MaximumDigits16;
	#[cfg(target_pointer_width = "32")] const MaximumDigits: usize = MaximumDigits32;
	#[cfg(target_pointer_width = "64")] const MaximumDigits: usize = MaximumDigits64;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroI8
{
	type Unsigned = u8;
	
	const MaximumDigits: usize = MaximumDigits8;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self.get() < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get().unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned
	}
}

impl NumberAsDecimalString for NonZeroI16
{
	type Unsigned = u16;
	
	const MaximumDigits: usize = MaximumDigits16;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self.get() < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get().unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroI32
{
	type Unsigned = u32;
	
	const MaximumDigits: usize = MaximumDigits32;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self.get() < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get().unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroI64
{
	type Unsigned = u64;
	
	const MaximumDigits: usize = MaximumDigits64;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self.get() < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get().unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroI128
{
	type Unsigned = u128;
	
	const MaximumDigits: usize = MaximumDigits128;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self.get() < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get().unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for NonZeroIsize
{
	type Unsigned = usize;
	
	#[cfg(target_pointer_width = "16")] const MaximumDigits: usize = MaximumDigits16;
	#[cfg(target_pointer_width = "32")] const MaximumDigits: usize = MaximumDigits32;
	#[cfg(target_pointer_width = "64")] const MaximumDigits: usize = MaximumDigits64;
	
	const AdjustmentForSign: bool = true;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		self.get() < 0
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self.get().unsigned_abs()
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned as u8
	}
}

impl NumberAsDecimalString for bool
{
	type Unsigned = u8;
	
	const MaximumDigits: usize = 1;
	
	const AdjustmentForSign: bool = false;
	
	const UnsignedZero: Self::Unsigned = 0;
	
	const UnsignedRadixTen: Self::Unsigned = 10;
	
	#[inline(always)]
	fn is_negative(self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn into_unsigned(self) -> Self::Unsigned
	{
		self as u8
	}
	
	#[inline(always)]
	fn unsigned_into_u8(unsigned: Self::Unsigned) -> u8
	{
		unsigned
	}
}
