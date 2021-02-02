// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A virtual address.
///
/// This is the same as the value returned from `malloc()`, a `*mut T` pointer, a `&T` reference, etc.
///
/// No checks are made for its validity.
///
/// It supports conversion of `Into<NonNull>` and `Into<NonZero*>` without checking (except in debug builds) that the value is not zero.
/// This is a breach of the contract of the `Into` trait, but is considered acceptable as testing every time for a condition that is highly unlikely to have arisen is expensive.
/// If this matters to you, use the equivalent `TryInto` implementations or `Into<Option<NonZero*>>` implementations.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct VirtualAddress(usize);

impl<T: ?Sized> From<NonNull<[T]>> for VirtualAddress
{
	#[inline(always)]
	fn from(value: NonNull<[T]>) -> Self
	{
		Self(value.as_mut_ptr() as usize)
	}
}

impl<'a, T: 'a + ?Sized> From<&'a [T]> for VirtualAddress
{
	#[inline(always)]
	fn from(value: &'a [T]) -> Self
	{
		Self(value.as_ptr() as usize)
	}
}

impl<'a, T: 'a + ?Sized> From<&'a mut [T]> for VirtualAddress
{
	#[inline(always)]
	fn from(value: &'a mut [T]) -> Self
	{
		Self(value.as_ptr() as usize)
	}
}

impl<'a, T> From<*const [T]> for VirtualAddress
{
	#[inline(always)]
	fn from(value: *const [T]) -> Self
	{
		Self(value.as_ptr() as usize)
	}
}

impl<'a, T> From<*mut [T]> for VirtualAddress
{
	#[inline(always)]
	fn from(value: *mut [T]) -> Self
	{
		Self(value.as_ptr() as usize)
	}
}

impl<T: Sized> From<NonNull<T>> for VirtualAddress
{
	#[inline(always)]
	fn from(value: NonNull<T>) -> Self
	{
		Self(value.as_ptr() as usize)
	}
}

impl<T: Sized> From<Option<NonNull<T>>> for VirtualAddress
{
	#[inline(always)]
	fn from(value: Option<NonNull<T>>) -> Self
	{
		let address = match value
		{
			None => 0,
			Some(value) => value.as_ptr() as usize,
		};
		Self(address)
	}
}

impl<'a, T: 'a + Sized> From<&'a T> for VirtualAddress
{
	#[inline(always)]
	fn from(value: &'a T) -> Self
	{
		Self(value as *const T as usize)
	}
}

impl<'a, T: 'a + Sized> From<&'a mut T> for VirtualAddress
{
	#[inline(always)]
	fn from(value: &'a mut T) -> Self
	{
		Self(value as *mut T as usize)
	}
}

impl<T: Sized> From<*const T> for VirtualAddress
{
	#[inline(always)]
	fn from(value: *const T) -> Self
	{
		Self(value as usize)
	}
}

impl<T: Sized> From<*mut T> for VirtualAddress
{
	#[inline(always)]
	fn from(value: *mut T) -> Self
	{
		Self(value as usize)
	}
}

impl From<usize> for VirtualAddress
{
	#[inline(always)]
	fn from(value: usize) -> Self
	{
		Self(value)
	}
}

impl From<u64> for VirtualAddress
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		Self(value as usize)
	}
}

impl<T: ?Sized> Into<Option<NonNull<T>>> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> Option<NonNull<T>>
	{
		NonNull::new(self.0 as *mut T)
	}
}

impl<T: ?Sized> Into<NonNull<T>> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> NonNull<T>
	{
		new_non_null(self.0 as *mut T)
	}
}

impl<T: ?Sized> TryInto<NonNull<T>> for VirtualAddress
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_into(self) -> Result<NonNull<T>, Self::Error>
	{
		if self.0 == 0
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(self.into())
		}
	}
}

impl<T: ?Sized> Into<*const T> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> *const T
	{
		self.0 as *const T
	}
}

impl<T: ?Sized> Into<*mut T> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> *mut T
	{
		self.0 as *mut T
	}
}

impl Into<u128> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> u128
	{
		self.0 as u128
	}
}

impl Into<NonZeroU128> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> NonZeroU128
	{
		new_non_zero_u128(self.0 as u128)
	}
}

impl Into<Option<NonZeroU128>> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> Option<NonZeroU128>
	{
		NonZeroU128::new(self.0 as u128)
	}
}

impl TryInto<NonZeroU128> for VirtualAddress
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_into(self) -> Result<NonZeroU128, Self::Error>
	{
		if self.0 == 0
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(self.into())
		}
	}
}

impl Into<usize> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0
	}
}

impl Into<NonZeroUsize> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> NonZeroUsize
	{
		new_non_zero_usize(self.0)
	}
}

impl Into<Option<NonZeroUsize>> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> Option<NonZeroUsize>
	{
		NonZeroUsize::new(self.0)
	}
}

impl TryInto<NonZeroUsize> for VirtualAddress
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_into(self) -> Result<NonZeroUsize, Self::Error>
	{
		if self.0 == 0
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(self.into())
		}
	}
}

impl Into<u64> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Into<NonZeroU64> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> NonZeroU64
	{
		new_non_zero_u64(self.0 as u64)
	}
}

impl Into<Option<NonZeroU64>> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> Option<NonZeroU64>
	{
		NonZeroU64::new(self.0 as u64)
	}
}

impl TryInto<NonZeroU64> for VirtualAddress
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_into(self) -> Result<NonZeroU64, Self::Error>
	{
		if self.0 == 0
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(self.into())
		}
	}
}

impl Add<usize> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: usize) -> Self::Output
	{
		Self(self.0 + rhs)
	}
}

impl AddAssign<usize> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: usize)
	{
		self.0 += rhs
	}
}

impl Sub<usize> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: usize) -> Self::Output
	{
		Self(self.0 - rhs)
	}
}

impl SubAssign<usize> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: usize)
	{
		self.0 -= rhs
	}
}

impl Add<NonZeroUsize> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: NonZeroUsize) -> Self::Output
	{
		Self(self.0 + rhs.get())
	}
}

impl AddAssign<NonZeroUsize> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: NonZeroUsize)
	{
		self.0 += rhs.get()
	}
}

impl Sub<NonZeroUsize> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: NonZeroUsize) -> Self::Output
	{
		Self(self.0 - rhs.get())
	}
}

impl SubAssign<NonZeroUsize> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: NonZeroUsize)
	{
		self.0 -= rhs.get()
	}
}

impl Add<NonZeroU64> for VirtualAddress
{
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: NonZeroU64) -> Self::Output
	{
		Self(self.0 + rhs.get() as usize)
	}
}

impl AddAssign<NonZeroU64> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: NonZeroU64)
	{
		self.0 += rhs.get() as usize
	}
}

impl Sub<NonZeroU64> for VirtualAddress
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: NonZeroU64) -> Self::Output
	{
		Self(self.0 - rhs.get() as usize)
	}
}

impl SubAssign<NonZeroU64> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: NonZeroU64)
	{
		self.0 -= rhs.get() as usize
	}
}

impl Add<u64> for VirtualAddress
{
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: u64) -> Self::Output
	{
		Self(self.0 + rhs as usize)
	}
}

impl AddAssign<u64> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: u64)
	{
		self.0 += rhs as usize
	}
}

impl Sub<u64> for VirtualAddress
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: u64) -> Self::Output
	{
		Self(self.0 - rhs as usize)
	}
}

impl SubAssign<u64> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: u64)
	{
		self.0 -= rhs as usize
	}
}

impl Add<NonZeroU32> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: NonZeroU32) -> Self::Output
	{
		Self(self.0 + rhs.get() as usize)
	}
}

impl AddAssign<NonZeroU32> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: NonZeroU32)
	{
		self.0 += rhs.get() as usize
	}
}

impl Sub<NonZeroU32> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: NonZeroU32) -> Self::Output
	{
		Self(self.0 - rhs.get() as usize)
	}
}

impl SubAssign<NonZeroU32> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: NonZeroU32)
	{
		self.0 -= rhs.get() as usize
	}
}

impl Add<u32> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u32) -> Self::Output
	{
		Self(self.0 + rhs as usize)
	}
}

impl AddAssign<u32> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: u32)
	{
		self.0 += rhs as usize
	}
}

impl Sub<u32> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: u32) -> Self::Output
	{
		Self(self.0 - rhs as usize)
	}
}

impl SubAssign<u32> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: u32)
	{
		self.0 -= rhs as usize
	}
}

impl Add<NonZeroU16> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: NonZeroU16) -> Self::Output
	{
		Self(self.0 + rhs.get() as usize)
	}
}

impl AddAssign<NonZeroU16> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: NonZeroU16)
	{
		self.0 += rhs.get() as usize
	}
}

impl Sub<NonZeroU16> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: NonZeroU16) -> Self::Output
	{
		Self(self.0 - rhs.get() as usize)
	}
}

impl SubAssign<NonZeroU16> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: NonZeroU16)
	{
		self.0 -= rhs.get() as usize
	}
}

impl Add<u16> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u16) -> Self::Output
	{
		Self(self.0 + rhs as usize)
	}
}

impl AddAssign<u16> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: u16)
	{
		self.0 += rhs as usize
	}
}

impl Sub<u16> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: u16) -> Self::Output
	{
		Self(self.0 - rhs as usize)
	}
}

impl SubAssign<u16> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: u16)
	{
		self.0 -= rhs as usize
	}
}

impl Add<NonZeroU8> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: NonZeroU8) -> Self::Output
	{
		Self(self.0 + rhs.get() as usize)
	}
}

impl AddAssign<NonZeroU8> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: NonZeroU8)
	{
		self.0 += rhs.get() as usize
	}
}

impl Sub<NonZeroU8> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: NonZeroU8) -> Self::Output
	{
		Self(self.0 - rhs.get() as usize)
	}
}

impl SubAssign<NonZeroU8> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: NonZeroU8)
	{
		self.0 -= rhs.get() as usize
	}
}

impl Add<u8> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u8) -> Self::Output
	{
		Self(self.0 + rhs as usize)
	}
}

impl AddAssign<u8> for VirtualAddress
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: u8)
	{
		self.0 += rhs as usize
	}
}

impl Sub<u8> for VirtualAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: u8) -> Self::Output
	{
		Self(self.0 - rhs as usize)
	}
}

impl SubAssign<u8> for VirtualAddress
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: u8)
	{
		self.0 -= rhs as usize
	}
}

impl Sub<Self> for VirtualAddress
{
	type Output = usize;
	
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		self.0 - rhs.0
	}
}

impl ParseNumber for VirtualAddress
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		Ok(Self(usize::parse_number(bytes, radix, parse_byte)?))
	}
}

impl ParseNumberOption for VirtualAddress
{
	#[inline(always)]
	fn parse_number_option(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Option<Self>, ParseNumberError>
	{
		let value = usize::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(value == 0)
		{
			Ok(None)
		}
		else
		{
			Ok(Some(Self(value)))
		}
	}
}

impl VirtualAddress
{
	/// Saturating subtraction.
	#[inline(always)]
	pub fn saturating_sub(self, rhs: Self) -> usize
	{
		Self(self.0.saturating_sub(rhs.0))
	}
	
	/// Saturating addition of 1.
	#[inline(always)]
	fn saturating_increment(self) -> Self
	{
		Self(self.0.saturating_add(1))
	}
	
	/// Relative offset from the start of the system page containing this virtual address.
	///
	/// May be zero, but will always be less than the system page size.
	#[inline(always)]
	pub fn offset_from_start_of_page(self) -> usize
	{
		self.0 % PageSize::default() as usize
	}
	
	/// The address of the page which contains this physical address.
	/// May be the same value.
	#[inline(always)]
	pub fn first_address_in_page(self) -> Self
	{
		Self(self.0 & !(PageSize::default() as usize - 1))
	}
	
	/// This function will translate virtual addresses to physical addresses.
	///
	/// Before using this function, the memory reference by a virtual address should have been `mlock()`'d.
	#[inline(always)]
	pub fn virtual_addresses_to_physical_addresses<HVA: HasVirtualAddress>(proc_path: &ProcPath, have_virtual_addresses: impl Iterator<Item=HVA>, mut physical_address_user: impl FnMut(HVA, Self, PhysicalAddress))
	{
		PageMap::read_our_pagemap(proc_path, have_virtual_addresses, |has_virtual_address, virtual_address, page_map_entry|
		{
			let physical_address_of_physical_page: PhysicalAddress = page_map_entry.physical_page_frame_number().into();
			let physical_address = physical_address_of_physical_page.add(virtual_address.offset_from_start_of_page());
			physical_address_user(has_virtual_address, virtual_address, physical_address)
		}).expect("could not read pagemap");
	}

	/// Pointer to value.
	#[inline(always)]
	pub fn pointer_to<T>(self, offset: usize) -> NonNull<T>
	{
		debug_assert_eq!(offset % align_of::<T>(), 0, "misaligned access");
		debug_assert_ne!(self.0 + offset, 0, "A zero address and zero offset is not permitted");

		unsafe { new_non_null((self.0 as *mut T).add(offset)) }
	}

	/// Pointer.
	#[inline(always)]
	pub fn offset_in_bytes(self, offset_in_bytes: usize) -> Self
	{
		self + offset_in_bytes
	}
}
