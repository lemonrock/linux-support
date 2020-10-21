// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `HINFO` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HINFOHandleRecordTypeError
{
	/// Resource data for resource record type `HINFO` has too short a length (value in tuple).
	HasTooShortALength(usize),
	
	/// Character string was invalid.
	CharacterStrings(NoCharacterStringsError),
	
	/// Resource data for resource record type `HINFO` has too short a length after checking length of CPU field (value in tuple).
	CpuDataMissing(usize),
	
	/// CPU character string was invalid.
	CpuDataCharacterString(CharacterStringLengthIncorrectError),
	
	/// Resource data for resource record type `HINFO` has too short a length after checking length of OS field (value in tuple).
	OsDataMissing(usize),
	
	/// OS character string was invalid.
	OsDataCharacterString(CharacterStringLengthIncorrectError),
	
	/// After parsing resource data in a record of type `HINFO`, there is unattributed data remaining.
	WouldHaveUnusuedDataRemaining,
}

impl Display for HINFOHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for HINFOHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::HINFOHandleRecordTypeError::*;
		
		match self
		{
			&CharacterStrings(ref error) => Some(error),
			
			&CpuDataCharacterString(ref error) => Some(error),
			
			&OsDataCharacterString(ref error) => Some(error),
			
			_ => None,
		}
	}
}
