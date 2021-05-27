// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Printable ASCII character.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PrintableAsciiCharacter(u8);

impl TryFrom<u8> for PrintableAsciiCharacter
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value >= 33 || value <= 126
		{
			Ok(Self(value))
		}
		else
		{
			Err(())
		}
	}
}

impl PrintableAsciiCharacter
{
	const NILVALUE: Self = Self(b'-');
	
	const Period: Self = Self(b'.');
	
	const Space: Self = Self(b' ');
	
	#[inline(always)]
	fn push_raw_slice_into_array_vec<const CAP: usize>(raw_slice: &[u8], array_vec: &mut ArrayVec<Self, CAP>) -> Result<(), PrintableAsciiCharacterPushError>
	{
		lazy_static!
		{
			static ref Empty: HashSet<u8> = HashSet::new();
		}
		Self::push_raw_slice_into_array_vec_with_additional_restrictions(raw_slice, array_vec, &Empty)
	}
	
	#[inline(always)]
	fn push_raw_slice_into_array_vec_with_additional_restrictions<const CAP: usize>(raw_slice: &[u8], array_vec: &mut ArrayVec<Self, CAP>, denied: &HashSet<u8>) -> Result<(), PrintableAsciiCharacterPushError>
	{
		use self::PrintableAsciiCharacterPushError::*;
		
		for raw in raw_slice
		{
			if unlikely!(denied.contains(raw))
			{
				return Err(DeniedPrintableAsciiCharacter(*raw))
			}
			let raw = *raw;
			
			let this: Self = raw.try_into().map_err(|_| NotAPrintableAsciiCharacter(raw))?;
			this.push_into_array_vec::<CAP>(array_vec)?
		}
		Ok(())
	}
	
	#[inline(always)]
	fn push_into_array_vec<const CAP: usize>(self, array_vec: &mut ArrayVec<Self, CAP>) -> Result<(), CapacityError<PrintableAsciiCharacter>>
	{
		array_vec.try_push(self)
	}
}
