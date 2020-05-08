// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Loads an `Atomic*` (eg `AtomicU8`) non-atomically without any synchronization.
pub trait LoadNonAtomically
{
	type NonAtomic: Sized + Copy;

	/// Loads from, say, an AtomicU32 non-atomically
	fn load_non_atomically(self) -> Self::NonAtomic;
}

macro_rules! load_non_atomically
{
	($atomic_type: ident, $non_atomic_type: ty) =>
	{
		impl<'a> LoadNonAtomically for &'a std::sync::atomic::$atomic_type
		{
			type NonAtomic = $non_atomic_type;

			#[inline(always)]
			fn load_non_atomically(self) -> Self::NonAtomic
			{
				let pointer: *const Self = unsafe { transmute(self) };
				unsafe { pointer.cast::<Self::NonAtomic>().read() }
			}
		}

		impl LoadNonAtomically for std::ptr::NonNull<std::sync::atomic::$atomic_type>
		{
			type NonAtomic = $non_atomic_type;

			#[inline(always)]
			fn load_non_atomically(self) -> Self::NonAtomic
			{
				(unsafe { self.as_ref() }).load_non_atomically()
			}
		}

		impl LoadNonAtomically for *const std::sync::atomic::$atomic_type
		{
			type NonAtomic = $non_atomic_type;

			#[inline(always)]
			fn load_non_atomically(self) -> Self::NonAtomic
			{
				(unsafe { & * self }).load_non_atomically()
			}
		}

		impl LoadNonAtomically for *mut std::sync::atomic::$atomic_type
		{
			type NonAtomic = $non_atomic_type;

			#[inline(always)]
			fn load_non_atomically(self) -> Self::NonAtomic
			{
				(unsafe { & * self }).load_non_atomically()
			}
		}
	}
}

load_non_atomically!(AtomicBool, bool);
load_non_atomically!(AtomicU8, u8);
load_non_atomically!(AtomicU16, u16);
load_non_atomically!(AtomicU32, u32);
load_non_atomically!(AtomicU64, u64);
load_non_atomically!(AtomicUsize, usize);
load_non_atomically!(AtomicI8, i8);
load_non_atomically!(AtomicI16, i16);
load_non_atomically!(AtomicI32, i32);
load_non_atomically!(AtomicI64, i64);
load_non_atomically!(AtomicIsize, isize);

impl<'a, T: 'a> LoadNonAtomically for &'a std::sync::atomic::AtomicPtr<T>
{
	type NonAtomic = *mut T;

	#[inline(always)]
	fn load_non_atomically(self) -> Self::NonAtomic
	{
		let pointer: *const Self = unsafe { transmute(self) };
		unsafe { pointer.cast::<Self::NonAtomic>().read() }
	}
}

impl<'a, T: 'a> LoadNonAtomically for std::ptr::NonNull<std::sync::atomic::AtomicPtr<T>>
{
	type NonAtomic = *mut T;

	#[inline(always)]
	fn load_non_atomically(self) -> Self::NonAtomic
	{
		(unsafe { self.as_ref() }).load_non_atomically()
	}
}

impl<'a, T: 'a> LoadNonAtomically for *const std::sync::atomic::AtomicPtr<T>
{
	type NonAtomic = *mut T;

	#[inline(always)]
	fn load_non_atomically(self) -> Self::NonAtomic
	{
		(unsafe { & * self }).load_non_atomically()
	}
}

impl<'a, T: 'a> LoadNonAtomically for *mut std::sync::atomic::AtomicPtr<T>
{
	type NonAtomic = *mut T;

	#[inline(always)]
	fn load_non_atomically(self) -> Self::NonAtomic
	{
		(unsafe { & * self }).load_non_atomically()
	}
}
