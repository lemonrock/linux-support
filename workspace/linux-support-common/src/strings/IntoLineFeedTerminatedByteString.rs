// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Converts data to a byte string terminated with a new line (`\n`).
pub trait IntoLineFeedTerminatedByteString<'a>
{
	/// Converts data to a byte string terminated with a new line (`\n`).
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>;
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for &'a [u8]
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		Cow::from(self)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for Vec<u8>
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		Cow::from(self)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for &'a Vec<u8>
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		Cow::from(&self[..])
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for Cow<'a, [u8]>
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		self
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for bool
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		let value = if self
		{
			b"1\n"
		}
		else
		{
			b"0\n"
		};
		Cow::from(value as &[u8])
	}
}

macro_rules! number_into_line_feed_terminated_byte_string
{
    ($unsigned_integer: ty, $Maximum: expr, $MaximumAdjustmentForSign: expr) =>
    {
		impl<'a> IntoLineFeedTerminatedByteString<'a> for $unsigned_integer
		{
			#[inline(always)]
			fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
			{
				const SizeOfLineFeed: usize = 1;
				const Maximum: usize = $Maximum + $MaximumAdjustmentForSign + SizeOfLineFeed;
				const LastIndex: usize = Maximum - 1;

				#[allow(deprecated)]
				let mut buffer: [u8; Maximum] = unsafe { uninitialized() };
				buffer[LastIndex] = b'\n';

				let index = self.lower_case_hexadecimal(LastIndex - SizeOfLineFeed, &mut buffer[..]);

				Cow::from((&buffer[index .. ]).to_vec())
			}
		}
    };
}
number_into_line_feed_terminated_byte_string!(u8, 3, 0);
number_into_line_feed_terminated_byte_string!(u16, 5, 0);
number_into_line_feed_terminated_byte_string!(u32, 10, 0);
number_into_line_feed_terminated_byte_string!(u64, 20, 0);
number_into_line_feed_terminated_byte_string!(u128, 39, 0);
#[cfg(target_pointer_width = "16")] number_into_line_feed_terminated_byte_string!(usize, 5, 0);
#[cfg(target_pointer_width = "32")] number_into_line_feed_terminated_byte_string!(usize, 10, 0);
#[cfg(target_pointer_width = "64")] number_into_line_feed_terminated_byte_string!(usize, 20, 0);
number_into_line_feed_terminated_byte_string!(i8, 3, 1);
number_into_line_feed_terminated_byte_string!(i16, 5, 1);
number_into_line_feed_terminated_byte_string!(i32, 10, 1);
number_into_line_feed_terminated_byte_string!(i64, 19, 1);
number_into_line_feed_terminated_byte_string!(i128, 39, 1);
#[cfg(target_pointer_width = "16")] number_into_line_feed_terminated_byte_string!(isize, 5, 1);
#[cfg(target_pointer_width = "32")] number_into_line_feed_terminated_byte_string!(isize, 10, 1);
#[cfg(target_pointer_width = "64")] number_into_line_feed_terminated_byte_string!(isize, 19, 1);

macro_rules! non_zero_number_into_line_feed_terminated_byte_string
{
    ($non_zero_type: ty) =>
    {
		impl<'a> IntoLineFeedTerminatedByteString<'a> for $non_zero_type
		{
			#[inline(always)]
			fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
			{
				self.get().into_line_feed_terminated_byte_string()
			}
		}
    };
}
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroU8);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroU16);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroU32);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroU64);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroU128);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroUsize);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroI8);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroI16);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroI32);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroI64);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroI128);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroIsize);
