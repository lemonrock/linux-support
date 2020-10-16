// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct MessageBitField1(u8);

impl MessageBitField1
{
	#[inline(always)]
	fn query_response(self) -> MessageType
	{
		unsafe { transmute(self.0 & 0b1000_000) }
	}
	
	/// Valid codes are of type `MessageOpcode`.
	#[inline(always)]
	fn raw_message_opcode(self) -> u8
	{
		(self.0 & 0b0111_1000) >> 3
	}
	
	#[inline(always)]
	fn authoritative_answer(self) -> bool
	{
		self.0 & 0b0000_0100 != 0
	}

	#[inline(always)]
	fn is_truncated(self) -> bool
	{
		self.0 & 0b0000_0010 != 0
	}
	
	#[inline(always)]
	fn recursion_desired(self) -> bool
	{
		self.0 & 0b0000_0001 != 0
	}

	#[inline(always)]
	const fn new_for_query() -> u8
	{
		const MessageTypeQuery: u8 = 0b0000_0000;
		const QueryRawOpcode: u8 = 0b0000_0000;
		const IsNotAnAuthoritativeAnswer: u8 = 0b0000_0000;
		const IsNotTruncated: u8 = 0b0000_0000;
		const RecursionIsDesired: u8 = 0b0000_0001;

		MessageTypeQuery | QueryRawOpcode | IsNotAnAuthoritativeAnswer | IsNotTruncated | RecursionIsDesired
	}
}
