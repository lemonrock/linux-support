// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A SHA-2 512 digest.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Sha2_512<'message>(&'message [u8; 512 / BitsInAByte]);

impl<'message> Digest<'message> for Sha2_512<'message>
{
	const DigestSizeInBits: usize = 512;
	
	#[inline(always)]
	unsafe fn new_unchecked(digest_data: *const u8) -> Self
	{
		Self(& * (digest_data as *const [u8; 512 / BitsInAByte]))
	}
}
