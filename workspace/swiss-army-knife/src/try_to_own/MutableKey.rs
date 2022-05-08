// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A mutable key for use in a HashMap that needs to support `TryToOwn`.
///
/// Implemented as a new type.
#[repr(transparent)]
pub struct MutableKey<K>(UnsafeCell<K>);

impl<K: Debug> Debug for MutableKey<K>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		self.borrow_x().fmt(formatter)
	}
}

impl<K: Display> Display for MutableKey<K>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		self.borrow_x().fmt(formatter)
	}
}

impl<K: Default> Default for MutableKey<K>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(K::default())
	}
}

impl<K: Clone> Clone for MutableKey<K>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self::new(self.borrow_x().clone())
	}
}

impl<K: PartialEq> PartialEq for MutableKey<K>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.borrow_x().eq(other.borrow_x())
	}
}

impl<K: Eq + PartialEq> Eq for MutableKey<K>
{
}

impl<K: PartialOrd> PartialOrd for MutableKey<K>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		self.borrow_x().partial_cmp(other.borrow())
	}
}

impl<K: PartialOrd + Ord + PartialEq + Eq> Ord for MutableKey<K>
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.borrow_x().cmp(other.borrow())
	}
}

impl<K: Hash> Hash for MutableKey<K>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.borrow_x().hash(state)
	}
}

impl<K> Borrow<K> for MutableKey<K>
{
	#[inline(always)]
	fn borrow(&self) -> &K
	{
		self.borrow_x()
	}
}

impl<K: TryToOwnInPlace> MutableKey<K>
{
	#[inline(always)]
	fn try_to_own_in_place(&self) -> Result<(), TryReserveError>
	{
		let pointer = self.0.get();
		let key = unsafe { &mut *pointer };
		key.try_to_own_in_place()
	}
}

impl<K> MutableKey<K>
{
	/// Create a new instance as a new type.
	#[inline(always)]
	pub const fn new(key: K) -> Self
	{
		Self(UnsafeCell::new(key))
	}
	
	#[inline(always)]
	fn borrow_x(&self) -> &K
	{
		let pointer = self.get();
		unsafe { &*pointer }
	}
	
	#[inline(always)]
	const fn get(&self) -> *mut K
	{
		self.0.get()
	}
}
