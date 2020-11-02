// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Fast, secure HashSet currently built atop hashbrown (SwissTable) using a secure AHash.
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct FastSecureHashSet<T: Eq + Hash>(hashbrown::HashSet<T>);

impl<T: Eq + Hash> Deref for FastSecureHashSet<T>
{
	type Target = hashbrown::HashSet<T>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<T: Eq + Hash> DerefMut for FastSecureHashSet<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<T: Eq + Hash + Clone> Clone for FastSecureHashSet<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(self.0.clone())
	}
	
	#[inline(always)]
	fn clone_from(&mut self, source: &Self)
	{
		self.0.clone_from(&source.0)
	}
}

impl<T: Eq + Hash> Default for FastSecureHashSet<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(hashbrown::HashSet::default())
	}
}

impl<T: Eq + Hash> PartialEq for FastSecureHashSet<T>
{
	#[inline(always)]
    fn eq(&self, other: &Self) -> bool
	{
        self.0.eq(&other.0)
    }
}

impl<T: Eq + Hash> Eq for FastSecureHashSet<T>
{
}

impl<T: Eq + Hash + Debug> Debug for FastSecureHashSet<T>
{
	#[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		self.0.fmt(f)
    }
}

impl<T: Eq + Hash + Clone> BitAnd<&FastSecureHashSet<T>> for &FastSecureHashSet<T>
{
    type Output = FastSecureHashSet<T>;
	
	#[inline(always)]
    fn bitand(self, rhs: &FastSecureHashSet<T>) -> Self::Output
	{
		FastSecureHashSet((&self.0).bitand(&rhs.0))
    }
}

impl<T: Eq + Hash + Clone> BitOr<&FastSecureHashSet<T>> for &FastSecureHashSet<T>
{
    type Output = FastSecureHashSet<T>;
	
	#[inline(always)]
    fn bitor(self, rhs: &FastSecureHashSet<T>) -> Self::Output
	{
		FastSecureHashSet((&self.0).bitor(&rhs.0))
    }
}

impl<T: Eq + Hash + Clone> BitXor<&FastSecureHashSet<T>> for &FastSecureHashSet<T>
{
	type Output = FastSecureHashSet<T>;
	
	#[inline(always)]
	fn bitxor(self, rhs: &FastSecureHashSet<T>) -> Self::Output
	{
		FastSecureHashSet((&self.0).bitxor(&rhs.0))
	}
}

impl<T: Eq + Hash + Clone> Sub<&FastSecureHashSet<T>> for &FastSecureHashSet<T>
{
	type Output = FastSecureHashSet<T>;
	
	#[inline(always)]
	fn sub(self, rhs: &FastSecureHashSet<T>) -> Self::Output
	{
		FastSecureHashSet((&self.0).sub(&rhs.0))
	}
}

impl<'a, T: 'a + Eq + Hash + Copy> Extend<&'a T> for FastSecureHashSet<T>
{
	#[inline(always)]
	fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I)
	{
		self.0.extend(iter)
	}
	
	#[inline(always)]
	fn extend_one(&mut self, k: &'a T)
	{
		self.0.extend_one(k)
	}
	
	#[inline(always)]
	fn extend_reserve(&mut self, additional: usize)
	{
		Extend::<&'a T>::extend_reserve(&mut self.0, additional)
	}
}

impl<T: Eq + Hash> Extend<T> for FastSecureHashSet<T>
{
	#[inline(always)]
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
	{
		self.0.extend(iter)
	}
	
	#[inline(always)]
	fn extend_one(&mut self, k: T)
	{
		self.0.extend_one(k)
	}
	
	#[inline(always)]
	fn extend_reserve(&mut self, additional: usize)
	{
		self.0.extend_reserve(additional)
	}
}

impl<T: Eq + Hash> FromIterator<T> for FastSecureHashSet<T>
{
	#[inline(always)]
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
	{
		Self(hashbrown::HashSet::<T>::from_iter::<I>(iter))
	}
}

impl<T: Eq + Hash> IntoIterator for FastSecureHashSet<T>
{
    type Item = T;
	
    type IntoIter = hashbrown::hash_set::IntoIter<T>;
	
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter
	{
		self.0.into_iter()
    }
}

impl<T: Eq + Hash> FastSecureHashSet<T>
{
	/// New.
	#[inline(always)]
	pub fn new() -> Self
	{
		Self(hashbrown::HashSet::new())
	}
	
	/// With capacity.
	#[inline(always)]
	pub fn with_capacity(capacity: usize) -> Self
	{
		Self(hashbrown::HashSet::with_capacity(capacity))
	}
}
