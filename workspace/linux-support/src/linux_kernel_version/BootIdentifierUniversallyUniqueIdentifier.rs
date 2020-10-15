// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Boot identifier UUID in Big-Endian format.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BootIdentifierUniversallyUniqueIdentifier(BigEndianU128);

impl Deref for BootIdentifierUniversallyUniqueIdentifier
{
	type Target = BigEndianU128;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl Into<u128> for BootIdentifierUniversallyUniqueIdentifier
{
	#[inline(always)]
	fn into(self) -> u128
	{
		self.into_u128()
	}
}

impl FromBytes for BootIdentifierUniversallyUniqueIdentifier
{
	type Error = ParseNumberError;
	
	/// eg `033acd7c-b7ab-43a2-b973-1ccc461ffdd2`.
	#[allow(deprecated)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		use self::ParseNumberError::*;
		
		let mut iterator = bytes.split_bytes_n(5, b'-');
		
		/// A piece of tehe UUID, eg `033acd7c`.
		#[inline(always)]
		fn parse_piece<'a>(iterator: &mut impl Iterator<Item=&'a [u8]>, uuid_index: &mut usize, uuid_big_endian_bytes: &mut BigEndianU128, expected_length: usize) -> Result<(), ParseNumberError>
		{
			let piece = iterator.next().ok_or(TooShort)?;
			if unlikely!(piece.len() != expected_length)
			{
				return Err(TooSmall)
			}
			
			let mut numeric_value_byte = 0;
			for nibble_index in 0 .. expected_length
			{
				let numeric_value_nibble =
				{
					let byte = unsafe { *piece.get_unchecked(nibble_index) };
					
					let subtract = match byte
					{
						b'0'..=b'9' => b'0',
						b'a'..=b'f' => (b'a' + 10),
						_ => return Err(InvalidByte { byte })
					};
					
					byte - subtract
				};
				
				let is_upper_nibble = (nibble_index % 2) == 0;
				if is_upper_nibble
				{
					numeric_value_byte = numeric_value_nibble << 4;
				}
				else
				{
					numeric_value_byte |= numeric_value_nibble;
					unsafe { *uuid_big_endian_bytes.get_unchecked_mut(*uuid_index) = numeric_value_byte };
					uuid_index.add_assign(1);
					numeric_value_byte = 0;
				}
			}
			
			Ok(())
		}
		
		let mut uuid_index = 0;
		let mut uuid_big_endian_bytes = unsafe { uninitialized() };
		parse_piece(&mut iterator, &mut uuid_index, &mut uuid_big_endian_bytes, 8)?;
		parse_piece(&mut iterator, &mut uuid_index, &mut uuid_big_endian_bytes, 4)?;
		parse_piece(&mut iterator, &mut uuid_index, &mut uuid_big_endian_bytes, 4)?;
		parse_piece(&mut iterator, &mut uuid_index, &mut uuid_big_endian_bytes, 4)?;
		parse_piece(&mut iterator, &mut uuid_index, &mut uuid_big_endian_bytes, 12)?;
		
		Ok(Self(uuid_big_endian_bytes))
	}
}

impl BootIdentifierUniversallyUniqueIdentifier
{
	/// New instance.
	#[inline(always)]
	pub fn new(proc_path: &ProcPath) -> io::Result<Self>
	{
		proc_path.sys_kernel_random_file_path("boot_id").read_value()
	}
	
	/// As `u128`.
	#[inline(always)]
	pub const fn into_u128(&self) -> u128
	{
		u128::from_be_bytes(self.0)
	}
}
