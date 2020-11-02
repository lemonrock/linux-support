// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Fast, secure HashMap currently built atop hashbrown (SwissTable) using a secure AHash.
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct FastSecureHashMap<K: Eq + Hash, V>(hashbrown::HashMap<K, V>);

impl<K: Eq + Hash, V> Deref for FastSecureHashMap<K, V>
{
	type Target = hashbrown::HashMap<K, V>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<K: Eq + Hash, V> DerefMut for FastSecureHashMap<K, V>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<K: Eq + Hash + Clone, V: Clone> Clone for FastSecureHashMap<K, V>
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

impl<K: Eq + Hash, V: PartialEq> PartialEq for FastSecureHashMap<K, V>
{
	#[inline(always)]
    fn eq(&self, other: &Self) -> bool
	{
		self.0.eq(&other.0)
    }
}

impl<K: Eq + Hash, V: Eq> Eq for FastSecureHashMap<K, V>
{
}

impl<K: Eq + Hash + Debug, V: Debug> Debug for FastSecureHashMap<K, V>
{
	#[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		self.0.fmt(f)
    }
}

impl<K: Eq + Hash, V> Default for FastSecureHashMap<K, V>
{
	#[inline(always)]
    fn default() -> Self
	{
		Self(hashbrown::HashMap::default())
    }
}

impl<K: Eq + Hash + Borrow<Q>, Q: ?Sized + Eq + Hash, V> Index<&Q> for FastSecureHashMap<K, V>
{
    type Output = V;
	
	#[inline(always)]
    fn index(&self, key: &Q) -> &V
	{
        self.0.index(key)
    }
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a FastSecureHashMap<K, V>
{
	type Item = (&'a K, &'a V);
	
	type IntoIter = hashbrown::hash_map::Iter<'a, K, V>;
	
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter
	{
		(&self.0).into_iter()
	}
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a mut FastSecureHashMap<K, V>
{
	type Item = (&'a K, &'a mut V);
	
	type IntoIter = hashbrown::hash_map::IterMut<'a, K, V>;
	
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter
	{
		(&mut self.0).into_iter()
	}
}

impl<K: Eq + Hash, V> IntoIterator for FastSecureHashMap<K, V>
{
	type Item = (K, V);
	
	type IntoIter = hashbrown::hash_map::IntoIter<K, V>;
	
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter
	{
		self.0.into_iter()
	}
}

impl<'a, K: 'a + Eq + Hash + Copy, V: 'a + Copy> Extend<(&'a K, &'a V)> for FastSecureHashMap<K, V>
{
	#[inline(always)]
	fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T)
	{
		(&mut self.0).extend(iter);
	}
	
	#[inline(always)]
	fn extend_one(&mut self, (k, v): (&'a K, &'a V))
	{
		(&mut self.0).extend_one((k, v));
	}
	
	#[inline(always)]
	fn extend_reserve(&mut self, additional: usize)
	{
		Extend::<(&'a K, &'a V)>::extend_reserve(&mut self.0, additional)
	}
}

impl<K: Eq + Hash, V,> Extend<(K, V)> for FastSecureHashMap<K, V>
{
	#[inline(always)]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T)
	{
		(&mut self.0).extend(iter)
    }
	
	#[inline(always)]
    fn extend_one(&mut self, (k, v): (K, V))
	{
		(&mut self.0).extend_one((k, v));
    }
	
	#[inline(always)]
    fn extend_reserve(&mut self, additional: usize)
	{
		(&mut self.0).extend_reserve(additional);
    }
}

impl<K: Eq + Hash, V> FromIterator<(K, V)> for FastSecureHashMap<K, V>
{
	#[inline(always)]
	fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self
	{
		 Self(hashbrown::HashMap::<K, V>::from_iter(iter))
	}
}

impl<K: Eq + Hash, V> FastSecureHashMap<K, V>
{
	/// New.
	#[inline(always)]
	pub fn new() -> Self
	{
		Self(hashbrown::HashMap::new())
	}
	
	/// With capacity.
	#[inline(always)]
	pub fn with_capacity(capacity: usize) -> Self
	{
		Self(hashbrown::HashMap::with_capacity(capacity))
	}
}
