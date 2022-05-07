// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Splits bytes using a needle (delimiter).
///
/// More efficient than using `(&[u8]).split(needle)`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SplitBytesNIterator<'a>
{
	n: usize,
	needle: u8,
	bytes: &'a [u8],
	splits: usize,
}

impl<'a> Iterator for SplitBytesNIterator<'a>
{
	type Item = &'a [u8];
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		// NOTE: `self.n` can legimitately be zero.
		if unlikely!(self.splits == self.n)
		{
			return None
		}
		
		if unlikely!(self.splits == (self.n - 1))
		{
			self.splits = self.n;
			return Some(self.bytes)
		}
		
		match self.bytes.memchr(self.needle)
		{
			None =>
			{
				self.splits = self.n;
				Some(self.bytes)
			}
			
			Some(index) =>
			{
				self.splits += 1;
				
				let item = &self.bytes[.. index];
				self.bytes = &self.bytes[index + 1 .. ];
				
				Some(item)
			}
		}
	}
}

impl<'a> SplitBytesNIterator<'a>
{
	#[inline(always)]
	const fn new(n: usize, needle: u8, bytes: &'a [u8]) -> Self
	{
		Self
		{
			n,
			needle,
			bytes,
			splits: 0,
		}
	}
}
