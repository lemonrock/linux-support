// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A hardware address.
///
/// Nearly always an Ethernet Media Access Control (MAC) hardware address with a length of `6`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HardwareAddress(ArrayVec<[u8; Self::MaximumLength.get()]>);

impl Deref for HardwareAddress
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl From<ArrayVec<[u8; Self::MaximumLength.get()]>> for HardwareAddress
{
	#[inline(always)]
	fn from(value: ArrayVec<[u8; Self::MaximumLength.get()]>) -> Self
	{
		Self(value)
	}
}

impl<'a> TryFrom<&'a [u8]> for HardwareAddress
{
	type Error = String;
	
	#[allow(deprecated)]
	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
	{
		let length = value.len();
		if unlikely!(length < Self::MinimumLength.get())
		{
			Err(format!("field has a hardware address that is too short ({}), can not be less than HardwareAddress::MinimumLength ({})", length, Self::MinimumLength))
		}
		else if unlikely!(length > Self::MaximumLength.get())
		{
			Err(format!("field has a hardware address that is too long ({}), can not greater than HardwareAddress::MaximumLength ({})", length, Self::MaximumLength))
		}
		else
		{
			let mut bytes: [u8; Self::MaximumLength.get()] = unsafe { uninitialized() };
			unsafe { bytes.as_mut_ptr().copy_from_nonoverlapping(value.as_ptr(), length) };
			let mut buffer = ConstArrayVec
			{
				xs: bytes,
				len: length as u8,
			};
			Ok(Self(unsafe { transmute(buffer) }))
		}
	}
}

impl HardwareAddress
{
	/// Minimum length.
	pub const MinimumLength: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };
	
	/// Maximum length.
	pub const MaximumLength: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(MAX_ADDR_LEN) };
	
	/// Ethernet Media Access Control (MAC) hardware address length (`6`).
	pub const EthernetMediaAccessControlLength: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(6) };
}
