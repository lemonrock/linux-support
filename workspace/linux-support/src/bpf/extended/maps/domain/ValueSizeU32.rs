// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Many map maximum value sizes depend on the value `KMALLOC_MAX_SIZE` less some amount of necessary overhead.
/// `KMALLOC_MAX_SIZE` varies by architecture but it is possible to assume a lower limit.
///
/// Rather than have a variable value per map depending on how much overhead they require (which itself can change as the Linux kernel evolves), it seems wiser to simply use an arbitrary cap below this for a maximum limit.
///
/// We have arbitrary chosen to use a maximum value of 2Mb, which gives the Linux kernel and its various maps 2Mb of overhead.
/// Given current overheads do not seem to exceed about 2Kb, this would seem sufficient for several years, at least.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ValueSizeU32(NonZeroU32);

impl TryFrom<usize> for ValueSizeU32
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		if unlikely!(value > u32::MAX as usize)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Self::try_from(value as u32)
		}
	}
}

impl TryFrom<u32> for ValueSizeU32
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if unlikely!(value == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			let non_zero = unsafe { NonZeroU32::new_unchecked(value) };
			Self::try_from(non_zero)
		}
	}
}

impl TryFrom<NonZeroU32> for ValueSizeU32
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU32) -> Result<Self, Self::Error>
	{
		if value >= Self::ExclusiveMaximum.0
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl ValueSizeU32
{
	/// See `include/linux/mmzone.h` in Linux kernel sources.
	const MAX_ORDER: u32 = 11;
	
	/// Varies by architecture, but usually `12` for systems with a 4Kb page size except:-
	///
	/// * ARM64 can optionally be configured to use 16 (64kb page size) using `CONFIG_ARM64_64K_PAGES`;
	/// * PowerPC can optionally be configured to other page sizes using `CONFIG_PPC_PAGE_SHIFT`;
	/// * SPARC64, NDS32, Alpha and OpenRisc use `13` (8Kb page size);
	/// * Hexagon can optionally be configured to use 14, 16, 18 or 20;
	/// * ARC can optionally be configured to use 12, 13, or 14.
	///
	/// None of these choices or processors are mainstream enough for the intended users of this code.
	///
	/// Linux does not support a page size smaller than 4Kb, so this is safe.
	pub(crate) const PAGE_SHIFT: u32 = 12;
	
	/// See `include/linux/slab.h` in Linux kernel sources.
	const KMALLOC_SHIFT_MAX: u32 = Self::MAX_ORDER + Self::PAGE_SHIFT - 1;
	
	/// See `include/linux/slab.h` in Linux kernel sources.
	const KMALLOC_MAX_SIZE: u32 = 1 << Self::KMALLOC_SHIFT_MAX;
	
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self::new_unsafe(1);
	
	/// Inclusive maximum.
	pub const ExclusiveMaximum: Self = Self::new_unsafe(Self::KMALLOC_MAX_SIZE / 2);
	
	/// Exclusive maximum.
	pub const InclusiveMaximum: Self = Self::new_unsafe(Self::ExclusiveMaximum.0.get() - 1);
	
	#[inline(always)]
	const fn new_unsafe(value: u32) -> Self
	{
		Self(unsafe { NonZeroU32::new_unchecked(value) })
	}
	
	#[inline(always)]
	pub(crate) const fn to_non_zero_u32(self) -> NonZeroU32
	{
		self.0
	}
	
	#[inline(always)]
	pub(crate) fn try_from_value_size<V: Sized>() -> Result<Self, ParseNumberError>
	{
		Self::try_from(size_of::<V>())
	}
}
