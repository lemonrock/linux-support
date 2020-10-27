// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `TXT` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TXTHandleRecordTypeError
{
	/// Character string was invalid.
	CharacterStrings(NoCharacterStringsError),
	
	/// Value `usize` is the character string index that failed.
	CharacterStringLengthIncorrect(usize, CharacterStringLengthIncorrectError),
}

impl Display for TXTHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for TXTHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::TXTHandleRecordTypeError::*;
		
		match self
		{
			&CharacterStrings(ref error) => Some(error),
			
			&CharacterStringLengthIncorrect(_character_string_index, ref error) => Some(error),
			
			_ => None,
		}
	}
}
