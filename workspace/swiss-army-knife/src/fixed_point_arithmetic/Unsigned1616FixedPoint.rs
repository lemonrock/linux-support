// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Q-format `Q16.16` unsigned fixed point number, viz:-
///
/// * Integer component is unsigned and 16 bits (`u16`).
/// * Fraction component is 16 bits.
///
/// See upstream library `https://github.com/PetteriAimonen/libfixmath` for example code to implement trigonometric functions.
#[derive(Default, Debug, Copy, Clone)]
#[derive(Deserialize, Serialize)]
#[repr(C)]
pub struct Unsigned1616FixedPoint(u32);

impl From<u32> for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		Self::from_u32(value)
	}
}

impl Into<u32> for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.into_u32()
	}
}

impl From<(BigEndianU16, BigEndianU16)> for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn from(value: (BigEndianU16, BigEndianU16)) -> Self
	{
		Self::new(u16::from_be_bytes(value.0), u16::from_be_bytes(value.1))
	}
}

impl Into<(BigEndianU16, BigEndianU16)> for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn into(self) -> (BigEndianU16, BigEndianU16)
	{
		(
			self.integer().to_be_bytes(),
			self.fraction().to_be_bytes(),
		)
	}
}

impl Into<(u16, u16)> for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn into(self) -> (u16, u16)
	{
		(
			self.integer(),
			self.fraction(),
		)
	}
}

impl TryFrom<Signed1616FixedPoint> for Unsigned1616FixedPoint
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: Signed1616FixedPoint) -> Result<Self, Self::Error>
	{
		if value.is_negative()
		{
			Err(ParseNumberError::TooSmall)
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}

impl PartialEq for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		let left = (*self).into_u32();
		let right = (*rhs).into_u32();
		left == right
	}
}

impl Eq for Unsigned1616FixedPoint
{
}

impl PartialOrd for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		let left = (*self).into_u32();
		let right = (*rhs).into_u32();
		left.cmp(&right)
	}
}

impl Hash for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		let this = (*self).into_u32();
		this.hash(state)
	}
}

impl Add for Unsigned1616FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output
	{
		self.checked_add(rhs).unwrap()
	}
}

impl Sub for Unsigned1616FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		self.checked_sub(rhs).unwrap()
	}
}

impl Rem for Unsigned1616FixedPoint
{
	type Output = Self;
	
	#[inline(always)]
	fn rem(self, rhs: Self) -> Self::Output
	{
		let this = self.into_u32();
		
		Self::from_u32(this % (rhs.into_u32()))
	}
}

impl AddAssign for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self)
	{
		*self = (*self).add(rhs)
	}
}

impl SubAssign for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self)
	{
		*self = (*self).sub(rhs)
	}
}

impl RemAssign for Unsigned1616FixedPoint
{
	#[inline(always)]
	fn rem_assign(&mut self, rhs: Self)
	{
		*self = (*self).rem(rhs)
	}
}

impl Unsigned1616FixedPoint
{
	const SmallestInteger: u16 = 0x0000;
	
	const LargestInteger: u16 = 0xFFFF;
	
	const FractionSizeInBits: u64 = 16;
	
	const FractionsPerInteger: u64 = 1 << Self::FractionSizeInBits;
	
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self::new(Self::SmallestInteger, 0x0000);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self::new(Self::LargestInteger, 0xFFFF);
	
	/// Zero, `0`.
	pub const Zero: Self = Self::new(Self::SmallestInteger, 0x0000);
	
	/// One, `1`.
	pub const One: Self = Self::new(0x0001, 0x0000);
	
	/// The constant `e`.
	pub const E: Self = Self::new(0x0002, 0xB7E1);
	
	/// The constant `π`.
	pub const PI: Self = Self::new(0x0003, 0x243F);
	
	/// New instance.
	#[inline(always)]
	pub const fn new(integer: u16, fraction: u16) -> Self
	{
		Self(((integer as u32) << 16) | (fraction as u32))
	}
	
	/// Integer.
	#[inline(always)]
	pub const fn integer(self) -> u16
	{
		(self.0 >> 16) as u16
	}
	
	/// Fraction.
	#[inline(always)]
	pub const fn fraction(self) -> u16
	{
		self.0 as u16
	}
	
	/// Constant from u32.
	#[inline(always)]
	pub const fn from_u32(value: u32) -> Self
	{
		Self(value)
	}
	
	/// Constant into u32.
	#[inline(always)]
	pub const fn into_u32(self) -> u32
	{
		self.0
	}
	
	/// Saturating addition.
	#[inline(always)]
	pub fn saturating_add(self, rhs: Self) -> Self
	{
		match self.checked_add(rhs)
		{
			None => Self::InclusiveMaximum,
			
			Some(sum) => sum,
		}
	}
	
	/// Saturating subtraction.
	#[inline(always)]
	pub fn saturating_sub(self, rhs: Self) -> Self
	{
		match self.checked_sub(rhs)
		{
			None => Self::InclusiveMaximum,
			
			Some(difference) => difference,
		}
	}
	
	/// Checked addition.
	#[inline(always)]
	pub fn checked_add(self, rhs: Self) -> Option<Self>
	{
		let left_integer = self.integer();
		let right_integer = rhs.integer();
		
		if let Some(integer) = left_integer.checked_add(right_integer)
		{
			let left_fraction = self.fraction() as u64;
			let right_fraction = rhs.fraction() as u64;
			
			let fraction = left_fraction + right_fraction;
			if fraction < Self::FractionsPerInteger
			{
				Some(Self::new(integer, fraction as u16))
			}
			else
			{
				if integer == Self::LargestInteger
				{
					None
				}
				else
				{
					let fraction = fraction - Self::FractionsPerInteger;
					debug_assert!(fraction < Self::FractionsPerInteger);
					Some(Self::new(integer + 1, fraction as u16))
				}
			}
		}
		else
		{
			None
		}
	}
	
	/// Checked subtraction.
	#[inline(always)]
	pub fn checked_sub(self, rhs: Self) -> Option<Self>
	{
		let left_integer = self.integer();
		let right_integer = rhs.integer();
		
		match left_integer.checked_sub(right_integer)
		{
			None => None,
			
			Some(integer) =>
			{
				let left_fraction = self.fraction();
				let right_fraction = rhs.fraction();
				
				if left_fraction >= right_fraction
				{
					let fraction = left_fraction - right_fraction;
					Some(Self::new(integer, fraction))
				}
				else
				{
					if integer == Self::SmallestInteger
					{
						None
					}
					else
					{
						let integer = integer - 1;
						let fraction = (left_fraction as u64) + Self::FractionsPerInteger - (right_fraction as u64);
						Some(Self::new(integer, fraction as u16))
					}
				}
			}
		}
	}
	
	/// Saturating multiply by a scalar.
	#[inline(always)]
	pub fn saturating_mul_by_scalar(self, scalar: u16) -> Self
	{
		match self.checked_mul_by_scalar(scalar)
		{
			Some(result) => result,
			
			None => Self::InclusiveMaximum,
		}
	}
	
	/// Checked multiply by a scalar.
	#[inline(always)]
	pub fn checked_mul_by_scalar(self, scalar: u16) -> Option<Self>
	{
		let original_integer = self.integer() as u64;
		let original_fraction = self.fraction() as u64;
		let scalar = scalar as u64;
		
		let total_fraction = original_fraction * scalar;
		let extra_integer = total_fraction / Self::FractionsPerInteger;
		let fraction = (total_fraction % Self::FractionsPerInteger) as u32;
		
		match original_integer.checked_mul(scalar)
		{
			Some(multiplied) => match multiplied.checked_add(extra_integer)
			{
				Some(integer) => return Some(Self::new(integer as u16, fraction as u16)),
				
				None => None,
			},
			
			None => None,
		}
	}
	
	/// Checked divide by a scalar.
	#[inline(always)]
	pub fn checked_div_by_scalar(self, scalar: u16) -> Option<Self>
	{
		if scalar == 0
		{
			None
		}
		else
		{
			let original_integer = self.integer() as u64;
			let original_fraction = self.fraction() as u64;
			let scalar = scalar as u64;
			
			let integer = original_integer / scalar;
			let carry = original_integer - integer * scalar;
			let extra_fraction = carry * Self::FractionsPerInteger / scalar;
			let fraction = original_fraction / scalar + extra_fraction;
			Some(Self::new(integer as u16, fraction as u16))
		}
	}
}
