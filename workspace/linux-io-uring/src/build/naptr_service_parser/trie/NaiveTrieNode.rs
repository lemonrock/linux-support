// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Needs to only store 'a-z', '0-9', ':', '+', '-' and '.'.
///
/// Hence the space required is 26 + 10 + 4 => 40.
pub(super) struct NaiveTrieNode<V>
{
	value: Option<V>,
	children: [Option<Box<Self>>; 40],
	has_children: bool,
}

impl<V> NaiveTrieNode<V>
{
	#[inline(always)]
	fn new() -> Self
	{
		Self
		{
			value: None,
			children:
			[
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
			],
			has_children: false,
		}
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	fn is_leaf(&self) -> bool
	{
		!self.has_children
	}
	
	#[inline(always)]
	pub(super) fn iter<'a>(&'a self) -> NaiveTrieNodeIterator<'a, V>
	{
		NaiveTrieNodeIterator
		{
			naive_trie_node: self,
			index: 0
		}
	}
	
	#[inline(always)]
	pub(super) fn value<'a>(&'a self) -> Option<&'a V>
	{
		self.value.as_ref()
	}
	
	fn store(&mut self, key: &[u8], value: V)
	{
		if key.is_empty()
		{
			debug_assert!(self.value.is_none(), "Duplicate key");
			
			self.value = Some(value)
		}
		else
		{
			let index = Self::compress_from_ascii_like_byte(key[0]);
			let child = self.children.get_mut(index).unwrap();
			if child.is_none()
			{
				*child = Some(Box::new(NaiveTrieNode::new()));
				self.has_children = true;
			}
			
			child.as_mut().unwrap().store(&key[1 .. ], value)
		}
	}
	
	#[inline(always)]
	fn decompress_to_ascii_like_byte(index: usize) -> u8
	{
		let index = index as u8;
		match index
		{
			0 => b'a',
			
			1 => b'b',
			
			2 => b'c',
			
			3 => b'd',
			
			4 => b'e',
			
			5 => b'f',
			
			6 => b'g',
			
			7 => b'h',
			
			8 => b'i',
			
			9 => b'j',
			
			10 => b'k',
			
			11 => b'l',
			
			12 => b'm',
			
			13 => b'n',
			
			14 => b'o',
			
			15 => b'p',
			
			16 => b'q',
			
			17 => b'r',
			
			18 => b's',
			
			19 => b't',
			
			20 => b'u',
			
			21 => b'v',
			
			22 => b'w',
			
			23 => b'x',
			
			24 => b'y',
			
			25 => b'z',
			
			26 => b'0',
			
			27 => b'1',
			
			28 => b'2',
			
			29 => b'3',
			
			30 => b'4',
			
			31 => b'5',
			
			32 => b'6',
			
			33 => b'7',
			
			34 => b'8',
			
			35 => b'9',
			
			36 => b':',
			
			37 => b'+',
			
			38 => b'-',
			
			39 => b'.',
			
			_ => unreachable!(),
		}
	}
	
	#[inline(always)]
	fn compress_from_ascii_like_byte(byte: u8) -> usize
	{
		match byte
		{
			b'a' => 0,
			
			b'b' => 1,
			
			b'c' => 2,
			
			b'd' => 3,
			
			b'e' => 4,
			
			b'f' => 5,
			
			b'g' => 6,
			
			b'h' => 7,
			
			b'i' => 8,
			
			b'j' => 9,
			
			b'k' => 10,
			
			b'l' => 11,
			
			b'm' => 12,
			
			b'n' => 13,
			
			b'o' => 14,
			
			b'p' => 15,
			
			b'q' => 16,
			
			b'r' => 17,
			
			b's' => 18,
			
			b't' => 19,
			
			b'u' => 20,
			
			b'v' => 21,
			
			b'w' => 22,
			
			b'x' => 23,
			
			b'y' => 24,
			
			b'z' => 25,
			
			b'0' => 26,
			
			b'1' => 27,
			
			b'2' => 28,
			
			b'3' => 29,
			
			b'4' => 30,
			
			b'5' => 31,
			
			b'6' => 32,
			
			b'7' => 33,
			
			b'8' => 34,
			
			b'9' => 35,
			
			b':' => 36,
			
			b'+' => 37,
			
			b'-' => 38,
			
			b'.' => 39,
			
			_ => panic!("Invalid byte `0x{:02?}`", byte)
		}
	}
}
