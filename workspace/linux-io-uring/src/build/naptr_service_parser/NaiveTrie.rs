// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct NaiveTrie<V>
{
	root: NaiveTrieNode<V>
}

impl<V> Deref for NaiveTrie<V>
{
	type Target = NaiveTrieNode<V>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.root
	}
}

impl<V> DerefMut for NaiveTrie<V>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.root
	}
}

impl<V> NaiveTrie<V>
{
	#[inline(always)]
	fn new() -> Self
	{
		Self
		{
			root: NaiveTrieNode::new(),
		}
	}
}

impl NaiveTrie<String>
{
	fn add(&mut self, hash_map: HashMap<impl AsRef<str>, impl AsRef<str>>)
	{
		for (key, value) in hash_map
		{
			let key  = key.as_ref();
			let value  = value.as_ref();
			
			let length = key.len();
			debug_assert_ne!(length, 0);
			debug_assert!(length <= MaximumServiceFieldSize);
			let key = key.as_bytes().to_ascii_lowercase();
			
			self.store(&key[..], value.to_string());
		}
	}
}
