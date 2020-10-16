// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! guard_hash_digest_if_final_field
{
	($resource_data: ident, $digest_offset: ident, $digest_size_in_bits: expr, $name: ident, $dns_protocol_error: ident) =>
	{
		{
			let digest_data = &$resource_data[$digest_offset .. ];

			let length = digest_data.len();

			const BitsInAByte: usize = 8;
			const DigestSizeInBytes: usize = $digest_size_in_bits / BitsInAByte;

			if unlikely!(length != DigestSizeInBytes)
			{
				return Err($dns_protocol_error(length))
			}

			$name(digest_data.start_pointer().unsafe_cast::<[u8; DigestSizeInBytes]>())
		}
	}
}
