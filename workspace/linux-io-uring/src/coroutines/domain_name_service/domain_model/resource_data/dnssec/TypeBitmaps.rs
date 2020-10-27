// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Type bitmaps.
///
/// Deliberately incomplete; we only track type codes 0x0000 to 0x007F and 0x0100 to 0x0107 as these are the only ones in effective use.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeBitmaps
{
	type_codes_0x0000_to_0x007F: [u8; Self::TypeCodes0x0000To0x007FSize],
	type_codes_0x0100_to_0x0107: u8,
}

impl TypeBitmaps
{
	const TypeCodes0x0000To0x007FSize: usize = 16;

	pub(crate) const MinimumTypeBitmapsSize: usize = 0;

	/// Return `None` if the data type is one that we do not track.
	#[inline(always)]
	pub fn is_data_type_present(&self, data_type: DataType) -> Option<bool>
	{
		let (data_type_upper, data_type_lower) = data_type.upper_and_lower();

		#[inline(always)]
		const fn is_bit_set_in_byte(byte: u8, network_bit_number: u8) -> Option<bool>
		{
			let normal_bit_number = 7 - network_bit_number;
			let bitmask = 1 << normal_bit_number;
			Some(byte & bitmask != 0)
		}

		match data_type_upper
		{
			0x00 =>
			{
				if likely!(data_type_lower <= 0x7F)
				{
					let network_byte_number = (data_type_lower >> 3) as usize;
					let network_bit_number = data_type_lower & 0b111;
					is_bit_set_in_byte((&self.type_codes_0x0000_to_0x007F[..]).u8(network_byte_number), network_bit_number)
				}
				else
				{
					None
				}
			}

			0x01 =>
			{
				if likely!(data_type_lower <= 0x07)
				{
					is_bit_set_in_byte(self.type_codes_0x0100_to_0x0107, data_type_lower)
				}
				else
				{
					None
				}
			}

			_ => None,
		}
	}

	/// RFC 4034, Section 4.1.2: "The RR type space is split into 256 window blocks, each representing the low-order 8 bits of the 16-bit RR type space.
	/// Each block that has at least one active RR type is encoded using a single octet window number (from 0 to 255), a single octet bitmap length (from 1 to 32) indicating the number of octets used for the window block's bitmap, and up to 32 octets (256 bits) of bitmap.
	/// Bits representing pseudo-types MUST be clear, as they do not appear in zone data.
	/// If encountered, they MUST be ignored upon being read".
	///
	/// Since type codes (0x00, 127) to (0x00, 255) are for query types and meta types, we can re-use the data occupied by them (16 bytes) for other types.
	/// Currently, only type codes (0x01, 0) and (0x01, 1) are usefuly defined; all other blocks can be ignored.
	#[allow(deprecated)]
	#[inline(always)]
	pub(crate) fn parse_type_bitmaps(data_type: DataType, blocks: &[u8]) -> Result<Self, TypeBitmapsParseError>
	{
		use self::TypeBitmapsParseError::*;
		
		let mut this: Self = unsafe { uninitialized() };

		let blocks_length = blocks.len();
		let mut block_starts_at = 0;
		let mut previous_window_number: i16 = -1;
		let mut have_seen_window_number_0: bool = false;
		while block_starts_at < blocks_length
		{
			const WindowSize: usize = 1;
			const BitmapLengthSize: usize = 1;
			const MinimumBitmapSize: usize = 1;
			if unlikely!(blocks_length < WindowSize + BitmapLengthSize + MinimumBitmapSize)
			{
				return Err(HasAnOverflowingBlockLength(data_type, blocks_length))
			}

			let window_number = blocks.u8(block_starts_at) as i16;
			if unlikely!(window_number <= previous_window_number)
			{
				return Err(HasARepeatedOrDecreasingWindowNumber(data_type))
			}

			let bitmap_length = blocks.u8(block_starts_at + WindowSize) as usize;
			if unlikely!(bitmap_length == 0)
			{
				return Err(HasAZeroBitmapLength(data_type))
			}
			if unlikely!(bitmap_length > 32)
			{
				return Err(HasAnIncorrectBitmapLength(data_type, bitmap_length))
			}
			if unlikely!(blocks_length < WindowSize + BitmapLengthSize + bitmap_length)
			{
				return Err(HasAnOverflowingBitmapLength(data_type, bitmap_length))
			}

			let block_ends_at = block_starts_at + WindowSize + BitmapLengthSize + bitmap_length;

			if likely!(window_number == 0x00)
			{
				have_seen_window_number_0 = true;

				let bitmap = &blocks[(block_starts_at + WindowSize + BitmapLengthSize) .. (block_ends_at)];

				let length_to_copy = if likely!(bitmap_length < Self::TypeCodes0x0000To0x007FSize)
				{
					unsafe { write_bytes(this.type_codes_0x0000_to_0x007F.as_mut_ptr().add(bitmap_length), 0x00, Self::TypeCodes0x0000To0x007FSize - bitmap_length) };
					bitmap_length
				}
				else
				{
					Self::TypeCodes0x0000To0x007FSize
				};
				unsafe { copy_nonoverlapping(bitmap.as_ptr(), this.type_codes_0x0000_to_0x007F.as_mut_ptr(), length_to_copy) };
			}
			else if likely!(window_number == 0x01)
			{
				let bitmap = &blocks[(block_starts_at + WindowSize + BitmapLengthSize) .. (block_ends_at)];

				this.type_codes_0x0100_to_0x0107 = bitmap.u8(0);
			}
			else
			{
				// Deliberately ignore other type bit maps as they are very rarely present or interesting at all; only `TA` and `DLV` are even registered, and they are obsolete.
			}

			block_starts_at = block_ends_at;
			previous_window_number = window_number;
		}

		if unlikely!(!have_seen_window_number_0)
		{
			this.type_codes_0x0000_to_0x007F = unsafe { zeroed() }
		}

		Ok(this)
	}
}
