// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Trait to make use of memchr more ergonomic, particularly with strings.
pub trait MemoryCharacter
{
	/// Searches from start for `needle`, returning its index.
	///
	/// Will never return `Some(usize::MAX)`.
	fn memchr(&self, needle: u8) -> Option<usize>;
	
	/// Searches from start for `needle1` and `needle2`, returning the first index matching either of them.
	///
	/// Will never return `Some(usize::MAX)`.
	fn memchr2(&self, needle1: u8, needle2: u8) -> Option<usize>;
	
	/// Searches from start for `needle1`, `needle2` and `needle3`, returning the first index matching either of them.
	///
	/// Will never return `Some(usize::MAX)`.
	fn memchr3(&self, needle1: u8, needle2: u8, needle3: u8) -> Option<usize>;
	
	/// Searches from end for `needle`, returning its index.
	///
	/// Will never return `Some(usize::MAX)`.
	fn memrchr(&self, needle: u8) -> Option<usize>;
	
	/// Searches from end for `needle1` and `needle2`, returning the first index matching either of them.
	///
	/// Will never return `Some(usize::MAX)`.
	fn memrchr2(&self, needle1: u8, needle2: u8) -> Option<usize>;
	
	/// Searches from end for `needle1`, `needle2` and `needle3`, returning the first index matching either of them.
	///
	/// Will never return `Some(usize::MAX)`.
	fn memrchr3(&self, needle1: u8, needle2: u8, needle3: u8) -> Option<usize>;
}

impl MemoryCharacter for [u8]
{
	#[inline(always)]
	fn memchr(&self, needle: u8) -> Option<usize>
	{
		memchr(needle, self)
	}
	
	#[inline(always)]
	fn memchr2(&self, needle1: u8, needle2: u8) -> Option<usize>
	{
		memchr2(needle1, needle2, self)
	}
	
	#[inline(always)]
	fn memchr3(&self, needle1: u8, needle2: u8, needle3: u8) -> Option<usize>
	{
		memchr3(needle1, needle2, needle3, self)
	}
	
	#[inline(always)]
	fn memrchr(&self, needle: u8) -> Option<usize>
	{
		memrchr(needle, self)
	}
	
	#[inline(always)]
	fn memrchr2(&self, needle1: u8, needle2: u8) -> Option<usize>
	{
		memrchr2(needle1, needle2, self)
	}
	
	#[inline(always)]
	fn memrchr3(&self, needle1: u8, needle2: u8, needle3: u8) -> Option<usize>
	{
		memrchr3(needle1, needle2, needle3, self)
	}
}

impl MemoryCharacter for str
{
	#[inline(always)]
	fn memchr(&self, needle: u8) -> Option<usize>
	{
		self.as_bytes().memchr(needle)
	}
	
	#[inline(always)]
	fn memchr2(&self, needle1: u8, needle2: u8) -> Option<usize>
	{
		self.as_bytes().memchr2(needle1, needle2)
	}
	
	#[inline(always)]
	fn memchr3(&self, needle1: u8, needle2: u8, needle3: u8) -> Option<usize>
	{
		self.as_bytes().memchr3(needle1, needle2, needle3)
	}
	
	#[inline(always)]
	fn memrchr(&self, needle: u8) -> Option<usize>
	{
		self.as_bytes().memrchr(needle)
	}
	
	#[inline(always)]
	fn memrchr2(&self, needle1: u8, needle2: u8) -> Option<usize>
	{
		self.as_bytes().memrchr2(needle1, needle2)
	}
	
	#[inline(always)]
	fn memrchr3(&self, needle1: u8, needle2: u8, needle3: u8) -> Option<usize>
	{
		self.as_bytes().memrchr3(needle1, needle2, needle3)
	}
}
