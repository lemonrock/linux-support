// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A structure that skips the `reserve_exact_before_push_byte()` checks because the calling code knows how many characters it is going to encode.
///
/// As a result, `self.encode_utf8_raw()` will always return `Ok`.
pub struct UnsafePerformantByteWritable<'a, BW: ByteWritable>(&'a mut BW);

impl<'a, BW: ByteWritable> ByteWritable for UnsafePerformantByteWritable<'a, BW>
{
	type ReservationError = BW::ReservationError;
	
	#[inline(always)]
	fn push_byte(&mut self, byte: u8)
	{
		self.0.push_byte(byte)
	}
}

impl<'a, BW: ByteWritable> UnsafePerformantByteWritable<'a, BW>
{
	/// Create a new instance.
	#[inline(always)]
	pub const fn new(byte_writable: &'a mut BW) -> Self
	{
		Self(byte_writable)
	}
}
