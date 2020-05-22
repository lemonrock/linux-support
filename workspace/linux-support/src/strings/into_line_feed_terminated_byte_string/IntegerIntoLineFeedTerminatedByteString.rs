// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// How to turn an integer into a line-feed (LF) terminated byte string
pub trait IntegerIntoLineFeedTerminatedByteString
{
	/// Octal.
	fn unpadded_octal(self) -> Cow<'static, [u8]>;
	
	/// Decimal.
	fn unpadded_decimal(self) -> Cow<'static, [u8]>;
	
	/// Hexadecimal using lower case alphabetic digits.
	fn unpadded_lower_case_hexadecimal(self) -> Cow<'static, [u8]>;
	
	/// Hexadecimal using lower case alphabetic digits, with leading zeros padding out to a fixed maximum width for this particular number.
	fn zero_padded_lower_case_hexadecimal(self) -> Cow<'static, [u8]>;
	
	/// Hexadecimal using upper case alphabetic digits.
	fn unpadded_upper_case_hexadecimal(self) -> Cow<'static, [u8]>;
	
	/// Hexadecimal using upper case alphabetic digits, with leading zeros padding out to a fixed maximum width for this particular number.
	fn zero_padded_upper_case_hexadecimal(self) -> Cow<'static, [u8]>;
}

macro_rules! integer_into_line_feed_terminated_byte_string
{
    ($integer: ty, $octal_maximum: expr, $decimal_maximum: expr, $hexadecimal_maximum: expr, $decimal_adjustment_for_sign: expr) =>
    {
		impl IntegerIntoLineFeedTerminatedByteString for $integer
		{
			#[inline(always)]
			fn unpadded_octal(self) -> Cow<'static, [u8]>
			{
				const SizeOfLineFeed: usize = 1;
				const Maximum: usize = $octal_maximum + SizeOfLineFeed;
				const LastIndex: usize = Maximum - 1;

				#[allow(deprecated)]
				let mut buffer: [u8; Maximum] = unsafe { uninitialized() };
				buffer[LastIndex] = b'\n';

				let index = self.octal(LastIndex - SizeOfLineFeed, &mut buffer[..]);

				Cow::Owned((&buffer[index .. ]).to_vec())
			}
			
			#[inline(always)]
			fn unpadded_decimal(self) -> Cow<'static, [u8]>
			{
				const SizeOfLineFeed: usize = 1;
				const Maximum: usize = $decimal_maximum + $decimal_adjustment_for_sign + SizeOfLineFeed;
				const LastIndex: usize = Maximum - 1;

				#[allow(deprecated)]
				let mut buffer: [u8; Maximum] = unsafe { uninitialized() };
				buffer[LastIndex] = b'\n';

				let index = self.decimal(LastIndex - SizeOfLineFeed, &mut buffer[..]);

				Cow::Owned((&buffer[index .. ]).to_vec())
			}
			
			#[inline(always)]
			fn unpadded_lower_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				const SizeOfLineFeed: usize = 1;
				const Maximum: usize = $hexadecimal_maximum + SizeOfLineFeed;
				const LastIndex: usize = Maximum - 1;

				#[allow(deprecated)]
				let mut buffer: [u8; Maximum] = unsafe { uninitialized() };
				buffer[LastIndex] = b'\n';

				let index = self.lower_case_hexadecimal(LastIndex - SizeOfLineFeed, &mut buffer[..]);

				Cow::Owned((&buffer[index .. ]).to_vec())
			}
			
			#[inline(always)]
			fn zero_padded_lower_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				const SizeOfLineFeed: usize = 1;
				const Maximum: usize = $hexadecimal_maximum + SizeOfLineFeed;
				const LastIndex: usize = Maximum - 1;

				#[allow(deprecated)]
				let mut buffer: [u8; Maximum] = unsafe { uninitialized() };
				buffer[LastIndex] = b'\n';

				let index = self.lower_case_hexadecimal(LastIndex - SizeOfLineFeed, &mut buffer[..]);

				unsafe { buffer.as_mut_ptr().write_bytes(b'0', index) }

				Cow::Owned((&buffer[ .. ]).to_vec())
			}
			
			#[inline(always)]
			fn unpadded_upper_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				const SizeOfLineFeed: usize = 1;
				const Maximum: usize = $hexadecimal_maximum + SizeOfLineFeed;
				const LastIndex: usize = Maximum - 1;

				#[allow(deprecated)]
				let mut buffer: [u8; Maximum] = unsafe { uninitialized() };
				buffer[LastIndex] = b'\n';

				let index = self.upper_case_hexadecimal(LastIndex - SizeOfLineFeed, &mut buffer[..]);

				Cow::Owned((&buffer[index .. ]).to_vec())
			}
			
			#[inline(always)]
			fn zero_padded_upper_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				const SizeOfLineFeed: usize = 1;
				const Maximum: usize = $hexadecimal_maximum + SizeOfLineFeed;
				const LastIndex: usize = Maximum - 1;

				#[allow(deprecated)]
				let mut buffer: [u8; Maximum] = unsafe { uninitialized() };
				buffer[LastIndex] = b'\n';

				let index = self.upper_case_hexadecimal(LastIndex - SizeOfLineFeed, &mut buffer[..]);

				unsafe { buffer.as_mut_ptr().write_bytes(b'0', index) }

				Cow::Owned((&buffer[ .. ]).to_vec())
			}
		}
	}
}
integer_into_line_feed_terminated_byte_string!(u128, 43, 39, 32, 0);
integer_into_line_feed_terminated_byte_string!(u64, 22, 20, 16, 0);
integer_into_line_feed_terminated_byte_string!(u32, 11, 10, 8, 0);
integer_into_line_feed_terminated_byte_string!(u16, 6, 5, 4, 0);
integer_into_line_feed_terminated_byte_string!(u8, 3, 3, 2, 0);
#[cfg(target_pointer_width = "64")] integer_into_line_feed_terminated_byte_string!(usize, 22, 20, 16, 0);
#[cfg(target_pointer_width = "32")] integer_into_line_feed_terminated_byte_string!(usize, 11, 10, 8, 0);
#[cfg(target_pointer_width = "16")] integer_into_line_feed_terminated_byte_string!(usize, 6, 5, 4, 0);
integer_into_line_feed_terminated_byte_string!(i128, 43, 39, 32, 1);
integer_into_line_feed_terminated_byte_string!(i64, 22, 20, 16, 1);
integer_into_line_feed_terminated_byte_string!(i32, 11, 10, 8, 1);
integer_into_line_feed_terminated_byte_string!(i16, 6, 5, 4, 1);
integer_into_line_feed_terminated_byte_string!(i8, 3, 3, 2, 1);
#[cfg(target_pointer_width = "64")] integer_into_line_feed_terminated_byte_string!(isize, 22, 20, 16, 1);
#[cfg(target_pointer_width = "32")] integer_into_line_feed_terminated_byte_string!(isize, 11, 10, 8, 1);
#[cfg(target_pointer_width = "16")] integer_into_line_feed_terminated_byte_string!(isize, 6, 5, 4, 1);

macro_rules! non_zero_number_into_line_feed_terminated_byte_string
{
    ($non_zero_type: ty, $zero: expr, $zero_type: ty) =>
    {
		impl IntegerIntoLineFeedTerminatedByteString for $non_zero_type
		{
			#[inline(always)]
			fn unpadded_octal(self) -> Cow<'static, [u8]>
			{
				self.get().unpadded_octal()
			}
			
			#[inline(always)]
			fn unpadded_decimal(self) -> Cow<'static, [u8]>
			{
				self.get().unpadded_decimal()
			}
			
			#[inline(always)]
			fn unpadded_lower_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				self.get().unpadded_lower_case_hexadecimal()
			}
			
			#[inline(always)]
			fn zero_padded_lower_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				self.get().zero_padded_lower_case_hexadecimal()
			}
			
			#[inline(always)]
			fn unpadded_upper_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				self.get().unpadded_upper_case_hexadecimal()
			}
			
			#[inline(always)]
			fn zero_padded_upper_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				self.get().zero_padded_upper_case_hexadecimal()
			}
		}

		impl IntegerIntoLineFeedTerminatedByteString for Option<$non_zero_type>
		{
			#[inline(always)]
			fn unpadded_octal(self) -> Cow<'static, [u8]>
			{
				(unsafe { transmute::<Self, $zero_type>(self) }).unpadded_octal()
			}
			
			#[inline(always)]
			fn unpadded_decimal(self) -> Cow<'static, [u8]>
			{
				(unsafe { transmute::<Self, $zero_type>(self) }).unpadded_decimal()
			}
			
			#[inline(always)]
			fn unpadded_lower_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				(unsafe { transmute::<Self, $zero_type>(self) }).unpadded_lower_case_hexadecimal()
			}
			
			#[inline(always)]
			fn zero_padded_lower_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				(unsafe { transmute::<Self, $zero_type>(self) }).zero_padded_lower_case_hexadecimal()
			}
			
			#[inline(always)]
			fn unpadded_upper_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				(unsafe { transmute::<Self, $zero_type>(self) }).unpadded_upper_case_hexadecimal()
			}
			
			#[inline(always)]
			fn zero_padded_upper_case_hexadecimal(self) -> Cow<'static, [u8]>
			{
				(unsafe { transmute::<Self, $zero_type>(self) }).zero_padded_upper_case_hexadecimal()
			}
		}
    };
}
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroU8, 0u8, u8);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroU16, 0u16, u16);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroU32, 0u32, u32);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroU64, 0u64, u64);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroU128, 0u128, u128);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroUsize, 0usize, usize);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroI8, 0i8, i8);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroI16, 0i16, i16);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroI32, 0i32, i32);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroI64, 0i64, i64);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroI128, 0i128, i128);
non_zero_number_into_line_feed_terminated_byte_string!(std::num::NonZeroIsize, 0isize, isize);
