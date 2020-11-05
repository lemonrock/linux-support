// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait Digest: PartialEq + Eq + PartialOrd + Ord + Hash + Deref<Target=Self::Array> + AsRef<[u8]> + Borrow<[u8]>
{
	type Array;
	
	const DigestSizeInBits: usize;
	
	const DigestSizeInBytes: usize = Self::DigestSizeInBits / BitsInAByte;
	
	unsafe fn new_unchecked<'message>(digest_data: &'message [u8]) -> &'message Self;
}
