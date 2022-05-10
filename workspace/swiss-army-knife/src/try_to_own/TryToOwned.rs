// This file is part of swiss-army-knife. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/swiss-army-knife/master/COPYRIGHT. No part of swiss-army-knife, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of swiss-army-knife. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/swiss-army-knife/master/COPYRIGHT.


/// Exists to support `Cow`.
pub trait TryToOwned: ToOwned
{
	/// Try to create an owned instance with a `'static` lifetime.
	///
	/// Rust does not permit a way to expressed the restriction of a `'static` lifetime.
	fn try_to_owned(&self) -> Result<<Self as ToOwned>::Owned, TryReserveError>;
}

impl TryToOwned for str
{
	#[inline(always)]
	fn try_to_owned(&self) -> Result<<Self as ToOwned>::Owned, TryReserveError>
	{
		let bytes = self.as_bytes().try_to_owned()?;
		Ok(unsafe { String::from_utf8_unchecked(bytes) })
	}
}

impl<T: Copy> TryToOwned for [T]
{
	#[inline(always)]
	fn try_to_owned(&self) -> Result<<Self as ToOwned>::Owned, TryReserveError>
	{
		let count = self.len();
		let from_pointer = self.as_ptr();
		
		let mut vec = Vec::new();
		vec.try_reserve(count)?;
		let to_pointer: *mut T = vec.as_mut_ptr();
		
		unsafe
		{
			to_pointer.copy_from_nonoverlapping(from_pointer, count);
			vec.set_len(count);
		}
		Ok(vec)
	}
}

impl<T: Copy> TryToOwned for T
{
	#[inline(always)]
	fn try_to_owned(&self) -> Result<<Self as ToOwned>::Owned, TryReserveError>
	{
		Ok(*self)
	}
}
