// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// UTF-8 sequence of 1, 2, 3 or 4 bytes.
pub trait Utf8Sequence: Copy + Eq + Ord + Hash + Debug + Default
{
	/// Length.
	const Length: Utf8CharacterLength;
	
	/// Length.
	const UsizeLength: usize = Self::Length.into();
	
	#[doc(hidden)]
	type Remainder: Copy + Eq + Ord + Debug;
	
	#[doc(hidden)]
	fn construct(first: u8, remainder: Self::Remainder) -> Self;
	
	/// Given the first byte, is this sequence matched?
	fn is(first: u8) -> bool;
	
	#[doc(hidden)]
	fn into_raw_unicode_code_point(self) -> u32;
	
	/// Rust limitation: This method is the same in all implementations, but Rust does not permit `const impl` traits to have default function implementations.
	fn try_into_char(self) -> Result<char, CharTryFromError>;
	
	/// Rust limitation: This method is the same in all implementations, but Rust does not permit `const impl` traits to have default function implementations.
	unsafe fn unchecked_into_char(self) -> char;
	
	/// Rust limitation: This method is the same in all implementations, but Rust does not permit `const impl` traits to have default function implementations.
	fn encode_character(character: char) -> Self;
	
	/// Encode.
	fn encode_u32(code: u32) -> Self;
	
	/// Rust limitation: This method is the same in all implementations, but Rust does not permit `const impl` traits to have default function implementations.
	fn write_unchecked(self, to: NonNull<u8>);
	
	/// Rust limitation: This method is the same in all implementations, but Rust does not permit `const impl` traits to have default function implementations.
	fn into_unchecked_utf8_sequence_and_character(self) -> Utf8SequenceAndCharacter;
	
	/// Rust limitation: This method is the same in all implementations, but Rust does not permit `const impl` traits to have default function implementations.
	fn try_into_utf8_sequence_and_character(self) -> Result<Utf8SequenceAndCharacter, CharTryFromError>;
}
