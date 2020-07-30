// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A physical identifier.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PhysicalIdentifier(ArrayVec<[u8; Self::MaximumLength]>);

impl From<ArrayVec<[u8; Self::MaximumLength]>> for PhysicalIdentifier
{
	#[inline(always)]
	fn from(value: ArrayVec<[u8; Self::MaximumLength]>) -> Result<Self, Self::Error>
	{
		Self(value)
	}
}

impl Into<ArrayVec<[u8; Self::MaximumLength]>> for PhysicalIdentifier
{
	#[inline(always)]
	fn into(self) -> ArrayVec<[u8; Self::MaximumLength]>
	{
		self.0
	}
}

impl<'a> TryFrom<&'a [u8]> for PhysicalIdentifier
{
	type Error = PhysicalIdentifierFromBytesError;
	
	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
	{
		Self::from_bytes(value)
	}
}

impl FromBytes for PhysicalIdentifier
{
	type Error = PhysicalIdentifierFromBytesError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		let length = bytes.len();
		
		if unlikely!(length > Self::MaximumLength)
		{
			return Err(PhysicalIdentifierFromBytesError::TooLong(length))
		}
		
		let mut array_vec = ArrayVec::new();
		
		unsafe
		{
			let pointer = array_vec.as_mut_ptr() as *mut c_char;
			pointer.copy_from_nonoverlapping(bytes.as_ptr() as *const c_char, length);
			array_vec.set_len(length)
		}
		Ok(Self(array_vec))
	}
}

impl Deref for PhysicalIdentifier
{
	type Target = ArrayVec<[u8; Self::MaximumLength]>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl PhysicalIdentifier
{
	/// Maximum length.
	pub const MaximumLength: usize = MAX_PHYS_ITEM_ID_LEN;
}
