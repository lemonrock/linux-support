// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Try to own an object in-place.
pub trait TryToOwnInPlace
{
	/// After calling this method, all lifetimes will have been erased and replaced with static; all data will be owned.
	///
	/// Rust does not permit a way to expressed the restriction of a `'static` lifetime.
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>;
}

impl<T: TryToOwnInPlace> TryToOwnInPlace for [T]
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		for index in 0 .. self.len()
		{
			self.get_unchecked_mut_safe(index).try_to_own_in_place()?
		}
		Ok(())
	}
}

impl<T: TryToOwnInPlace> TryToOwnInPlace for Option<T>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		if let Some(some) = self
		{
			some.try_to_own_in_place()
		}
		else
		{
			Ok(())
		}
	}
}

impl<'a, B: 'static + TryToOwned + ?Sized> TryToOwnInPlace for Cow<'a, B>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		use Cow::*;
		if let Borrowed(borrowed) = self
		{
			let string = (*borrowed).try_to_owned()?;
			*self = Owned(string)
		}
		Ok(())
	}
}

impl<TTOIP: TryToOwnInPlace> TryToOwnInPlace for Vec<TTOIP>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		let length = self.len();
		for index in 0 .. length
		{
			let ttoip = self.get_unchecked_mut_safe(index);
			ttoip.try_to_own_in_place()?;
		}
		Ok(())
	}
}
