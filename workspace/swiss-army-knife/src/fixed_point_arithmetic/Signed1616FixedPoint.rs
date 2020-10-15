// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Q-format `Q16.16` signed fixed point number, viz:-
///
/// * Integer component is signed and 16 bits (`i16`).
/// * Fraction component is 16 bits.
///
/// See upstream library `https://github.com/PetteriAimonen/libfixmath` for example code to implement trigonometric functions.
#[derive(Default, Debug, Copy, Clone)]
#[derive(Deserialize, Serialize)]
#[repr(C)]
pub struct Signed1616FixedPoint(i32);

impl From<i32> for Signed1616FixedPoint
{
	#[inline(always)]
	fn from(value: i32) -> Self
	{
		Self::from_i32(value)
	}
}

impl Into<i32> for Signed1616FixedPoint
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self.into_i32()
	}
}

impl From<u32> for Signed1616FixedPoint
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		Self::from_u32(value)
	}
}

impl Into<u32> for Signed1616FixedPoint
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.into_u32()
	}
}

impl From<(BigEndianI16, BigEndianU16)> for Signed1616FixedPoint
{
	#[inline(always)]
	fn from(value: (BigEndianI16, BigEndianU16)) -> Self
	{
		Self::new(i16::from_be_bytes(value.0), u16::from_be_bytes(value.1))
	}
}

impl Into<(BigEndianI16, BigEndianU16)> for Signed1616FixedPoint
{
	#[inline(always)]
	fn into(self) -> (BigEndianI16, BigEndianU16)
	{
		(
			self.integer().to_be_bytes(),
			self.fraction().to_be_bytes(),
		)
	}
}

impl Into<(i16, u16)> for Signed1616FixedPoint
{
	#[inline(always)]
	fn into(self) -> (i16, u16)
	{
		(
			self.integer(),
			self.fraction(),
		)
	}
}

impl TryFrom<Unsigned1616FixedPoint> for Signed1616FixedPoint
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: Unsigned1616FixedPoint) -> Result<Self, Self::Error>
	{
		if value.integer() > (Self::LargestInteger as u16)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}

impl PartialEq for Signed1616FixedPoint
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		let left = (*self).into_i32();
		let right = (*rhs).into_i32();
		left == right
	}
}

impl Eq for Signed1616FixedPoint
{
}

impl PartialOrd for Signed1616FixedPoint
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for Signed1616FixedPoint
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		use self::Ordering::*;
		
		match self.integer().cmp(&rhs.integer())
		{
			Less => Less,
			
			Greater => Greater,
			
			Equal =>
			{
				let compare = self.fraction().cmp(&rhs.fraction());
				
				let both_are_negative = self.is_negative();
				if both_are_negative
				{
					compare.reverse()
				}
				else
				{
					compare
				}
			}
		}
	}
}

impl Hash for Signed1616FixedPoint
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		let this = (*self).into_i32();
		this.hash(state)
	}
}

impl Add for Signed1616FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output
	{
		self.checked_add(rhs).unwrap()
	}
}

impl Sub for Signed1616FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		self.checked_sub(rhs).unwrap()
	}
}

impl Mul for Signed1616FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output
	{
		self.checked_mul(rhs).unwrap()
	}
}

impl Div for Signed1616FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn div(self, rhs: Self) -> Self::Output
	{
		self.checked_div(rhs).unwrap()
	}
}

impl Rem for Signed1616FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn rem(self, rhs: Self) -> Self::Output
	{
		let this = self.into_i32();
		
		Self::from_i32(this % (rhs.into_i32()))
	}
}

impl Neg for Signed1616FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn neg(self) -> Self
	{
		Self::new(-self.integer(), self.fraction())
	}
}

impl AddAssign for Signed1616FixedPoint
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self)
	{
		*self = (*self).add(rhs)
	}
}

impl SubAssign for Signed1616FixedPoint
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self)
	{
		*self = (*self).sub(rhs)
	}
}

impl MulAssign for Signed1616FixedPoint
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Self)
	{
		*self = (*self).mul(rhs)
	}
}

impl DivAssign for Signed1616FixedPoint
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: Self)
	{
		*self = (*self).div(rhs)
	}
}

impl RemAssign for Signed1616FixedPoint
{
	#[inline(always)]
	fn rem_assign(&mut self, rhs: Self)
	{
		*self = (*self).rem(rhs)
	}
}

impl Signed1616FixedPoint
{
	const LargestInteger: i16 = 0x7FFF;
	
	const ZeroFraction: u16 = 0x0000;
	
	const SignBit: u32 = 0x8000_0000;
	
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self::new(0x8000u16 as i16, Self::ZeroFraction);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self::new(Self::LargestInteger, 0xFFFF);
	
	/// Zero, `0`.
	pub const Zero: Self = Self::new(0x0000, Self::ZeroFraction);
	
	/// One, `1`.
	pub const One: Self = Self::new(0x0001, Self::ZeroFraction);
	
	/// The constant `e`.
	pub const E: Self = Self::new(0x0002, 0xB7E1);
	
	/// The constant `π`.
	pub const PI: Self = Self::new(0x0003, 0x243F);
	
	/// New instance.
	#[inline(always)]
	pub const fn new(integer: i16, fraction: u16) -> Self
	{
		Self((((integer as u32) << 16) | (fraction as u32)) as i32)
	}
	
	/// Integer.
	#[inline(always)]
	pub const fn integer(self) -> i16
	{
		((self.0 as u32) >> 16) as u16 as i16
	}
	
	/// Fraction.
	#[inline(always)]
	pub const fn fraction(self) -> u16
	{
		(self.0 as u32) as u16
	}
	
	/// Constant from i32.
	#[inline(always)]
	pub const fn from_i32(value: i32) -> Self
	{
		unsafe { transmute(value) }
	}
	
	/// Constant into i32.
	#[inline(always)]
	pub const fn into_i32(self) -> i32
	{
		unsafe { transmute(self) }
	}
	
	/// Constant from u32.
	#[inline(always)]
	pub const fn from_u32(value: u32) -> Self
	{
		unsafe { transmute(value) }
	}
	
	/// Constant into u32.
	#[inline(always)]
	pub const fn into_u32(self) -> u32
	{
		unsafe { transmute(self) }
	}
	
	/// Computes the absolute value of self.
	///
	/// The absolute value of `Self::InclusiveMinimum` cannot be represented without a sign, and attempting to calculate it will cause an overflow.
	/// This means that code in debug mode will trigger a panic on this case and optimized code will return `Self::InclusiveMinimum` without a panic.
	#[inline(always)]
	pub const fn abs(self) -> Self
	{
		Self::new(self.integer().abs(), self.fraction())
		
	}
	
	/// Returns true if self is positive and false if the number is zero or negative.
	#[inline(always)]
	pub const fn is_positive(self) -> bool
	{
		self.integer().is_positive()
	}
	
	/// Returns true if self is negative and false if the number is zero or positive.
	#[inline(always)]
	pub const fn is_negative(self) -> bool
	{
		self.integer().is_negative()
	}
	
	/// Returns a number representing sign of self.
	///
	/// * `0` if the number is zero.
	/// * `1` if the number is positive.
	/// * `-1` if the number is negative.
	#[inline(always)]
	pub const fn signum(self) -> i16
	{
		self.integer().signum()
	}
	
	/// Overflow can only happen if `sign_of_left == sign_of_left`, and then it causes `sign_of_sum_or_difference != sign_of_left`.
	#[inline(always)]
	const fn detect_overflow(left: u32, right: u32, sum_or_difference: u32) -> bool
	{
		((left ^ right) & Self::SignBit == 0) && ((left ^ sum_or_difference) & Self::SignBit != 0)
	}
	
	/// Saturating addition.
	#[inline(always)]
	pub fn saturating_add(self, rhs: Self) -> Self
	{
		match self.checked_add(rhs)
		{
			None => if self.into_i32() >= 0
			{
				Self::InclusiveMaximum
			}
			else
			{
				Self::InclusiveMinimum
			},
			
			Some(sum) => sum,
		}
	}
	
	/// Saturating subtraction.
	#[inline(always)]
	pub fn saturating_sub(self, rhs: Self) -> Self
	{
		match self.checked_sub(rhs)
		{
			None => if self.into_i32() >= 0
			{
				Self::InclusiveMaximum
			}
			else
			{
				Self::InclusiveMinimum
			},
			
			Some(difference) => difference,
		}
	}
	
	/// Saturating multiplication.
	#[inline(always)]
	pub fn saturating_mul(self, rhs: Self) -> Self
	{
		match self.checked_mul(rhs)
		{
			None =>
			{
				let left = self.into_i32();
				let right = rhs.into_i32();
				
				if (left >= 0) == (right >= 0)
				{
					return Self::InclusiveMaximum
				}
				else
				{
					return Self::InclusiveMinimum
				}
			},
			
			Some(product) => product
		}
	}
	
	/// Saturating division.
	#[inline(always)]
	pub fn saturating_div(self, rhs: Self) -> Self
	{
		match self.checked_div(rhs)
		{
			None =>
			{
				let left = self.into_i32();
				let right = rhs.into_i32();
				
				if (left >= 0) == (right >= 0)
				{
					return Self::InclusiveMaximum
				}
				else
				{
					return Self::InclusiveMinimum
				}
			}
			
			Some(result) => result,
		}
	}
	
	/// Checked addition.
	#[inline(always)]
	pub fn checked_add(self, rhs: Self) -> Option<Self>
	{
		let left = self.into_u32();
		let right = rhs.into_u32();
		let sum = left.wrapping_add(right);
		
		if Self::detect_overflow(left, right, sum)
		{
			None
		}
		else
		{
			Some(Self::from_u32(sum))
		}
	}
	
	/// Checked subtraction.
	#[inline(always)]
	pub fn checked_sub(self, rhs: Self) -> Option<Self>
	{
		let left = self.into_u32();
		let right = rhs.into_u32();
		let difference = left.wrapping_sub(right);
		
		if Self::detect_overflow(left, right, difference)
		{
			None
		}
		else
		{
			Some(Self::from_u32(difference))
		}
	}
	
	/// Checked multiplication.
	///
	/// Takes advantage of 64-bit CPUs.
	///
	/// Performs a 32 × 32 -> 64 bit multiplication.
	/// The middle 32 bits are the result, the bottom 16 bits are used for rounding, and the upper 16 bits are used for overflow detection.
	#[inline(always)]
	pub fn checked_mul(self, rhs: Self) -> Option<Self>
	{
		let left = self.into_i32() as i64;
		let right = rhs.into_i32() as i64;
		
		let product = left * right;
		
		// The upper 17 bits should all be the same (the sign).
		let upper = ((product >> 47) as i32) as u32;
		
		let product = if product < 0
		{
			if !upper != 0
			{
				return None
			}
			
			// This adjustment is required in order to round -1/2 correctly
			product - 1
		}
		else if upper != 0
		{
			return None
		}
		else
		{
			product
		};
		
		let mut result = (product >> 16) as i32;
		result += ((product & 0x8000) >> 15) as i32;
		
		Some(Self::from_i32(result))
	}
	
	/// Checked division.
	///
	/// Takes advantage of 32-bit CPUs.
	/// Does not take advantage of 64-bit CPUs.
	#[inline(always)]
	pub fn checked_div(self, rhs: Self) -> Option<Self>
	{
		let a = self.into_i32();
		let b = rhs.into_i32();
		
		// This uses a hardware 32/32 bit division multiple times, until we have computed all the bits in (a<<17)/b.
		// Usually this takes 1-3 iterations.
		if b == 0
		{
			return Some(Self::InclusiveMinimum)
		}
		
		let mut remainder: u32 =
		(
			if a >= 0
			{
				a
			}
			else
			{
				-a
			}
		) as u32;
		
		let mut divider: u32 =
		(
			if b >= 0
			{
				b
			}
			else
			{
				-b
			}
		) as u32;
		
		let mut quotient: u32 = 0;
		let mut bit_pos: i32 = 17;
		
		// Kick-start the division a bit.
		// This improves speed in the worst-case scenarios where N and D are large
		// It gets a lower estimate for the result by N/(D >> 17 + 1).
		if divider & 0xFFF00000 as u32 != 0
		{
			let shifted_div = (divider >> 17).wrapping_add(1);
			quotient = remainder.wrapping_div(shifted_div);
			remainder = (remainder as u64).wrapping_sub((quotient as u64).wrapping_mul(divider as u64) >> 17) as u32
		}
		
		// If the divider is divisible by 2^n, take advantage of it.
		while (divider & 0xF) == 0 && bit_pos >= 4
		{
			divider >>= 4;
			bit_pos -= 4
		}
		
		while remainder != 0 && bit_pos >= 0
		{
			// Shift remainder as much as we can without overflowing.
			let mut shift = remainder.leading_zeros() as i32;
			if shift > bit_pos
			{
				shift = bit_pos
			}
			remainder <<= shift;
			bit_pos -= shift;
			let div = remainder.wrapping_div(divider);
			remainder = remainder.wrapping_rem(divider);
			quotient = (quotient as u32).wrapping_add(div << bit_pos);
			if div & !(0xFFFF_FFFF >> bit_pos) != 0
			{
				return None
			}
			remainder <<= 1;
			bit_pos -= 1
		}
		
		// Quotient is always positive so rounding is easy.
		quotient = quotient.wrapping_add(1);
		let mut result: i32 = (quotient >> 1) as i32;
		
		// Figure out the sign of the result.
		if ((a ^ b) as u32) & Self::SignBit != 0
		{
			if result == Self::InclusiveMinimum.into_i32()
			{
				return None
			}
			result = -result
		}
		
		return Some(Self::from_i32(result));
	}
	
	/// Saturating multiply by a scalar.
	#[inline(always)]
	pub fn saturating_mul_by_scalar(self, scalar: i16) -> Self
	{
		match self.checked_mul_by_scalar(scalar)
		{
			Some(result) => result,
			
			None => if self.is_positive()
			{
				Self::InclusiveMaximum
			}
			else
			{
				Self::InclusiveMinimum
			},
		}
	}
	
	/// Checked multiply by a scalar.
	#[inline(always)]
	pub fn checked_mul_by_scalar(self, scalar: i16) -> Option<Self>
	{
		self.checked_mul(Self::new(scalar, Self::ZeroFraction))
	}
	
	/// Checked divide by a scalar.
	#[inline(always)]
	pub fn checked_div_by_scalar(self, scalar: i16) -> Option<Self>
	{
		self.checked_div(Self::new(scalar, Self::ZeroFraction))
	}
}
