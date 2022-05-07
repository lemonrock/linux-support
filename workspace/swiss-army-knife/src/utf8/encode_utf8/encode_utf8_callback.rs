// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(super) fn encode_utf8_callback<A, R>(character: char, argument: A, utf8_sequence_1: impl FnOnce(A, Utf8Sequence1) -> R, utf8_sequence_2: impl FnOnce(A, Utf8Sequence2) -> R, utf8_sequence_3: impl FnOnce(A, Utf8Sequence3) -> R, utf8_sequence_4: impl FnOnce(A, Utf8Sequence4) -> R) -> R
{
	let code = character as u32;
	
	if code < MAX_ONE_B
	{
		utf8_sequence_1(argument, Utf8Sequence1::encode_u32(code))
	}
	else if code < MAX_TWO_B
	{
		utf8_sequence_2(argument, Utf8Sequence2::encode_u32(code))
	}
	else if code < MAX_THREE_B
	{
		utf8_sequence_3(argument, Utf8Sequence3::encode_u32(code))
	}
	else
	{
		utf8_sequence_4(argument, Utf8Sequence4::encode_u32(code))
	}
}
