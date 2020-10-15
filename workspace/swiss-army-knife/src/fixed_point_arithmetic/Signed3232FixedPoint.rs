// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Q-format `Q32.32` signed fixed point number, viz:-
///
/// * Integer component is signed and 32 bits (`i32`).
/// * Fraction component is 32 bits.
///
/// See upstream library `https://github.com/PetteriAimonen/libfixmath` for example code to implement trigonometric functions.
#[derive(Default, Debug, Copy, Clone)]
#[derive(Deserialize, Serialize)]
#[repr(C)]
pub struct Signed3232FixedPoint(i64);

impl From<i64> for Signed3232FixedPoint
{
	#[inline(always)]
	fn from(value: i64) -> Self
	{
		Self::from_i64(value)
	}
}

impl Into<i64> for Signed3232FixedPoint
{
	#[inline(always)]
	fn into(self) -> i64
	{
		self.into_i64()
	}
}

impl From<u64> for Signed3232FixedPoint
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		Self::from_u64(value)
	}
}

impl Into<u64> for Signed3232FixedPoint
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.into_u64()
	}
}

impl From<(BigEndianI32, BigEndianU32)> for Signed3232FixedPoint
{
	#[inline(always)]
	fn from(value: (BigEndianI32, BigEndianU32)) -> Self
	{
		Self::new(i32::from_be_bytes(value.0), u32::from_be_bytes(value.1))
	}
}

impl Into<(BigEndianI32, BigEndianU32)> for Signed3232FixedPoint
{
	#[inline(always)]
	fn into(self) -> (BigEndianI32, BigEndianU32)
	{
		(
			self.integer().to_be_bytes(),
			self.fraction().to_be_bytes(),
		)
	}
}

impl Into<(i32, u32)> for Signed3232FixedPoint
{
	#[inline(always)]
	fn into(self) -> (i32, u32)
	{
		(
			self.integer(),
			self.fraction(),
		)
	}
}

impl TryFrom<Unsigned3232FixedPoint> for Signed3232FixedPoint
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: Unsigned3232FixedPoint) -> Result<Self, Self::Error>
	{
		if value.integer() > (Self::LargestInteger as u32)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}

impl PartialEq for Signed3232FixedPoint
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		let left = (*self).into_i64();
		let right = (*rhs).into_i64();
		left == right
	}
}

impl Eq for Signed3232FixedPoint
{
}

impl PartialOrd for Signed3232FixedPoint
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for Signed3232FixedPoint
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

impl Hash for Signed3232FixedPoint
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		let this = (*self).into_i64();
		this.hash(state)
	}
}

impl Add for Signed3232FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output
	{
		self.checked_add(rhs).unwrap()
	}
}

impl Sub for Signed3232FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		self.checked_sub(rhs).unwrap()
	}
}

impl Rem for Signed3232FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn rem(self, rhs: Self) -> Self::Output
	{
		let this = self.into_i64();
		
		Self::from_i64(this % (rhs.into_i64()))
	}
}

impl Neg for Signed3232FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn neg(self) -> Self
	{
		Self::new(-self.integer(), self.fraction())
	}
}

impl AddAssign for Signed3232FixedPoint
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self)
	{
		*self = (*self).add(rhs)
	}
}

impl SubAssign for Signed3232FixedPoint
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self)
	{
		*self = (*self).sub(rhs)
	}
}

impl RemAssign for Signed3232FixedPoint
{
	#[inline(always)]
	fn rem_assign(&mut self, rhs: Self)
	{
		*self = (*self).rem(rhs)
	}
}

impl Signed3232FixedPoint
{
	const LargestInteger: i32 = 0x7FFF_FFFF;
	
	const ZeroFraction: u32 = 0x0000_0000;
	
	const SignBit: u64 = 0x8000_0000_0000_0000;
	
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self::new(0x8000_0000u32 as i32, Self::ZeroFraction);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self::new(Self::LargestInteger, 0xFFFF_FFFF);
	
	/// Zero, `0`.
	pub const Zero: Self = Self::new(0x0000_0000, Self::ZeroFraction);
	
	/// One, `1`.
	pub const One: Self = Self::new(0x0000_0001, Self::ZeroFraction);
	
	/// New instance.
	#[inline(always)]
	pub const fn new(integer: i32, fraction: u32) -> Self
	{
		Self((((integer as u64) << 32) | (fraction as u64)) as i64)
	}
	
	/// Integer.
	#[inline(always)]
	pub const fn integer(self) -> i32
	{
		((self.0 as u64) >> 32) as u32 as i32
	}
	
	/// Fraction.
	#[inline(always)]
	pub const fn fraction(self) -> u32
	{
		(self.0 as u32) as u32
	}
	
	/// Constant from i64.
	#[inline(always)]
	pub const fn from_i64(value: i64) -> Self
	{
		unsafe { transmute(value) }
	}
	
	/// Constant into i64.
	#[inline(always)]
	pub const fn into_i64(self) -> i64
	{
		unsafe { transmute(self) }
	}
	
	/// Constant from u64.
	#[inline(always)]
	pub const fn from_u64(value: u64) -> Self
	{
		unsafe { transmute(value) }
	}
	
	/// Constant into u64.
	#[inline(always)]
	pub const fn into_u64(self) -> u64
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
	pub const fn signum(self) -> i32
	{
		self.integer().signum()
	}
	
	/// Overflow can only happen if `sign_of_left == sign_of_left`, and then it causes `sign_of_sum_or_difference != sign_of_left`.
	#[inline(always)]
	const fn detect_overflow(left: u64, right: u64, sum_or_difference: u64) -> bool
	{
		((left ^ right) & Self::SignBit == 0) && ((left ^ sum_or_difference) & Self::SignBit != 0)
	}
	
	/// Saturating addition.
	#[inline(always)]
	pub fn saturating_add(self, rhs: Self) -> Self
	{
		match self.checked_add(rhs)
		{
			None => if self.into_i64() >= 0
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
			None => if self.into_i64() >= 0
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
	
	/// Checked addition.
	#[inline(always)]
	pub fn checked_add(self, rhs: Self) -> Option<Self>
	{
		let left = self.into_u64();
		let right = rhs.into_u64();
		let sum = left.wrapping_add(right);
		
		if Self::detect_overflow(left, right, sum)
		{
			None
		}
		else
		{
			Some(Self::from_u64(sum))
		}
	}
	
	/// Checked subtraction.
	#[inline(always)]
	pub fn checked_sub(self, rhs: Self) -> Option<Self>
	{
		let left = self.into_u64();
		let right = rhs.into_u64();
		let difference = left.wrapping_sub(right);
		
		if Self::detect_overflow(left, right, difference)
		{
			None
		}
		else
		{
			Some(Self::from_u64(difference))
		}
	}
}
