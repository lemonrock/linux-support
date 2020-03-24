// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! unsigned_into_line_feed_terminated_byte_string
{
    ($unsigned_integer: ty, $Maximum: expr) =>
    {
		impl<'a> IntoLineFeedTerminatedByteString<'a> for $unsigned_integer
		{
			#[inline(always)]
			fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
			{
				const Maximum: usize = $Maximum + 1;

				#[allow(deprecated)]
				let mut buffer: [u8; Maximum] = unsafe { uninitialized() };
				buffer[Maximum - 1] = b'\n';

				let mut index = Maximum - 2;
				let mut x = self;
				loop
				{
					const Radix: $unsigned_integer = 10;
					let remainder = x % Radix;
					x = x / Radix;

					debug_assert!(Radix <= 10, "This logic does not work for Radices greater than 10");
					buffer[index] = b'0' + (remainder as u8);

					index -= 1;
					if x == 0
					{
						break
					}
				}

				Cow::from((&buffer[index .. ]).to_vec())
			}
		}
    };
}

unsigned_into_line_feed_terminated_byte_string!(u8, 3);
unsigned_into_line_feed_terminated_byte_string!(u16, 5);
unsigned_into_line_feed_terminated_byte_string!(u32, 10);
unsigned_into_line_feed_terminated_byte_string!(u64, 20);
unsigned_into_line_feed_terminated_byte_string!(u128, 39);
#[cfg(target_pointer_width = "16")] unsigned_into_line_feed_terminated_byte_string!(usize, 5);
#[cfg(target_pointer_width = "32")] unsigned_into_line_feed_terminated_byte_string!(usize, 10);
#[cfg(target_pointer_width = "64")] unsigned_into_line_feed_terminated_byte_string!(usize, 20);
