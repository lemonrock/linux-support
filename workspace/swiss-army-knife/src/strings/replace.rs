// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// replace.
#[inline(always)]
pub fn replace(extant: &[u8], from: u8, to: u8) -> Box<[u8]>
{
	let mut result = Vec::with_capacity(extant.len());

	for byte in extant.iter()
	{
		let byte = *byte;
		let byte_to_push = if unlikely!(byte == from)
		{
			to
		}
		else
		{
			byte
		};
		result.push(byte_to_push);
	}

	result.into_boxed_slice()
}
