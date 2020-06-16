// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Splits bytes using a needle (delimiter).
///
/// More efficient than using `(&[u8]).split(needle)`.
pub trait SplitBytes
{
	/// Splits bytes using a needle (delimiter).
	///
	/// More efficient than using `(&[u8]).split(|byte| *byte == needle)`.
	fn split_bytes(&self, needle: u8) -> SplitBytesIterator;
	
	/// Splits bytes using a needle (delimiter).
	///
	/// More efficient than using `(&[u8]).splitn(|byte| *byte == needle)`.
	///
	/// `n` is the maximum number of splits.
	fn split_bytes_n(&self, n: usize, needle: u8) -> SplitBytesNIterator;
	
	/// Splits bytes using a needle (delimiter).
	///
	/// More efficient than using `(&[u8]).split(|byte| *byte == needle).rev()`.
	fn split_bytes_reverse(&self, needle: u8) -> SplitBytesReverseIterator;
}

impl SplitBytes for [u8]
{
	#[inline(always)]
	fn split_bytes(&self, needle: u8) -> SplitBytesIterator
	{
		SplitBytesIterator::new(needle, self)
	}
	
	#[inline(always)]
	fn split_bytes_n(&self, n: usize, needle: u8) -> SplitBytesNIterator
	{
		SplitBytesNIterator::new(n, needle, self)
	}
	
	#[inline(always)]
	fn split_bytes_reverse(&self, needle: u8) -> SplitBytesReverseIterator
	{
		SplitBytesReverseIterator::new(needle, self)
	}
}

impl<A: AsRef<[u8]>> SplitBytes for A
{
	#[inline(always)]
	fn split_bytes(&self, needle: u8) -> SplitBytesIterator
	{
		SplitBytesIterator::new(needle, self.as_ref())
	}
	
	#[inline(always)]
	fn split_bytes_n(&self, n: usize, needle: u8) -> SplitBytesNIterator
	{
		SplitBytesNIterator::new(n, needle, self.as_ref())
	}
	
	#[inline(always)]
	fn split_bytes_reverse(&self, needle: u8) -> SplitBytesReverseIterator
	{
		SplitBytesReverseIterator::new(needle, self.as_ref())
	}
}
