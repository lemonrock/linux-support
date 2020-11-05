// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C, packed)]
struct RawCharacterString
{
	pub(crate) length: u8,
	bytes: UpTo255Bytes,
}

impl RawCharacterString
{
	#[inline(always)]
	fn as_slice(&self, length: usize) -> &[u8]
	{
		(&self.bytes).unsafe_cast_slice::<u8>(length)
	}
}
