// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) struct NaiveTrieNodeIterator<'a, V>
{
	naive_trie_node: &'a NaiveTrieNode<V>,
	index: usize,
}

impl<'a, V> Iterator for NaiveTrieNodeIterator<'a, V>
{
	type Item = (u8, &'a NaiveTrieNode<V>);
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.index == 40
		{
			return None
		}
		
		while self.index != 40
		{
			match self.naive_trie_node.children.get(self.index).unwrap()
			{
				None =>
				{
					self.index += 1;
				}
				
				Some(child) =>
				{
					let result = Some((NaiveTrieNode::<V>::decompress_to_ascii_like_byte(self.index), child.deref()));
					self.index += 1;
					return result
				}
			}
		}
		
		None
	}
}
