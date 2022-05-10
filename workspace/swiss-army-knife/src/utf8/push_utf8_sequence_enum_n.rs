// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! push_utf8_sequence_enum_n
{
	($self: ident, $utf8_sequence: ident, $member: ident) =>
	{
		{
			debug_assert_eq!($utf8_sequence.utf8_character_length(), Utf8CharacterLength::$member);
			
			match $utf8_sequence
			{
				Utf8SequenceEnum::$member(utf8_sequence) => $self.push_utf8_sequence(utf8_sequence),
				
				_ => unsafe { unreachable_unchecked() }
			}
		}
	}
}
