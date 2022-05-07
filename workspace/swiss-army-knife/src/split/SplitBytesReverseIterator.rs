// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Splits bytes using a needle (delimiter).
///
/// More efficient than using `(&[u8]).split(needle)`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SplitBytesReverseIterator<'a>
{
	needle: u8,
	bytes: &'a [u8],
	finished: bool,
}

impl<'a> Iterator for SplitBytesReverseIterator<'a>
{
	type Item = &'a [u8];
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if unlikely!(self.finished)
		{
			return None
		}
		
		match self.bytes.memrchr(self.needle)
		{
			None =>
			{
				self.finished = true;
				Some(self.bytes)
			}
			
			Some(index) =>
			{
				let item = &self.bytes[index + 1 .. ];
				self.bytes = &self.bytes[ .. index ];
				
				Some(item)
			}
		}
	}
}

impl<'a> SplitBytesReverseIterator<'a>
{
	#[inline(always)]
	const fn new(needle: u8, bytes: &'a [u8]) -> Self
	{
		Self
		{
			needle,
			bytes,
			finished: false,
		}
	}
}
