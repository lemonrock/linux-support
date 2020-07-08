// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An unnamed (tuple index) field.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnnamedField
{
	/// Type.
	pub type_: &'static Type,
	
	/// Offset of field.
	///
	/// Maximum value is (2^24 - 1) / 8 = 2,097,151.
	pub offset_in_bytes: u32,
}

impl Field for UnnamedField
{
	#[inline(always)]
	fn to_btf_member(&self, type_identifiers: &mut BtfTypeIdentifiers, index: u16) -> Result<btf_member, BtfTypeError>
	{
		Ok
		(
			btf_member
			{
				name_off: unsafe { transmute(type_identifiers.push_c_identifier(&format!("_{}", index), BtfKind::Function)?) },
				type_identifier: type_identifiers.get_or_create_type_identifier(self.type_)?,
				offset: self.offset_in_bits()?,
			}
		)
	}
}

impl UnnamedField
{
	const MaximumOffsetInBits: u32 = 2^24 - 1;
	
	const BitsInAByte: u32 = 8;
	
	const MaximumOffsetInBytes: u32 = Self::MaximumOffsetInBits / Self::BitsInAByte;
	
	/// New instance.
	#[inline(always)]
	pub const fn new<T: 'static + HasReflectionInformation>(offset_in_bytes: u32) -> Self
	{
		Self
		{
			type_: &T::Type,
			offset_in_bytes,
		}
	}
	
	#[inline(always)]
	fn offset_in_bits(&self) -> Result<u32, BtfTypeError>
	{
		let offset_in_bytes = self.offset_in_bytes;
		if unlikely!(self.offset_in_bytes > Self::MaximumOffsetInBytes)
		{
			Err(BtfTypeError::FieldOffsetTooLarge)
		}
		else
		{
			let offset_in_bits = offset_in_bytes * Self::BitsInAByte;
			
			const NotABitFieldSize: u32 = 0 << 24;
			Ok(NotABitFieldSize | offset_in_bits)
		}
	}
}
