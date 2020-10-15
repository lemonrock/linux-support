// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A wrapper type to use with mutable static fields where one does not want the overhead and inflexibility of `lazy_static!`.
pub struct StaticInitializedOnce<Value: Sized>
{
	#[cfg(debug_assertions)] initialization_pattern: u8,
	value: MaybeUninit<Value>,
}

impl<Value: Sized> StaticInitializedOnce<Value>
{
	#[cfg(debug_assertions)] const Uninitialized: u8 = 0x00;
	
	#[cfg(debug_assertions)] const Initialized: u8 = 0xFF;
	
	/// Uninitialized value suitable for use in a static.
	#[inline(always)]
	pub const fn uninitialized() -> Self
	{
		Self
		{
			#[cfg(debug_assertions)] initialization_pattern: Self::Uninitialized,
			value: MaybeUninit::uninit(),
		}
	}
	
	/// Initialize once.
	///
	/// Not thread safe; indeed, may panic in debug mode.
	#[inline(always)]
	pub fn initialize_once(&mut self, value: Value) -> &mut Value
	{
		#[cfg(debug_assertions)]
		{
			debug_assert_eq!(self.initialization_pattern, Self::Uninitialized);
			self.initialization_pattern = Self::Initialized;
		}
		self.value.write(value)
	}
	
	/// Value.
	#[inline(always)]
	pub fn value(&self) -> &'static Value
	{
		#[cfg(debug_assertions)]
		{
			debug_assert_eq!(self.initialization_pattern, Self::Initialized);
		}
		unsafe { & * self.value.as_ptr() }
	}
	
	/// Value.
	#[inline(always)]
	pub fn value_mut(&mut self) -> &'static mut Value
	{
		#[cfg(debug_assertions)]
		{
			debug_assert_eq!(self.initialization_pattern, Self::Initialized);
		}
		unsafe { &mut * self.value.as_mut_ptr() }
	}
}
