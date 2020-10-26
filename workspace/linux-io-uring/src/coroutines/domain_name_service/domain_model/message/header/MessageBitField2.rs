// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct MessageBitField2(u8);

impl MessageBitField2
{
	#[inline(always)]
	fn recursion_available(self) -> bool
	{
		self.0 & 0b1000_000 != 0
	}

	#[inline(always)]
	fn z(self) -> bool
	{
		self.0 & 0b0100_0000 != 0
	}

	#[inline(always)]
	fn authentic_data(self) -> bool
	{
		self.0 & 0b0010_0000 != 0
	}

	#[inline(always)]
	fn checking_disabled(self) -> bool
	{
		self.0 & 0b0001_0000 != 0
	}
	
	#[inline(always)]
	fn raw_message_response_code(self) -> RCodeLower4Bits
	{
		RCodeLower4Bits(self.0 & 0b0000_1111)
	}

	#[inline(always)]
	const fn new_for_query() -> u8
	{
		// Also known as `RD` or `rd`.
		const RecursionIsAvailableShouldNotBeSetInQuery: u8 = 0b0000_0000;
		
		// Also known as `Z` or `z`.
		const UnassignedBitsMustBeZero: u8 = 0b0000_0000;
		
		// Also known as `AD` or `ad`.
		// Set in accordance with RFC 6840, Section 5.7, Setting the AD Bit on Queries: "This document defines setting the AD bit in a query as a signal indicating that the requester understands and is interested in the value of the AD bit in the response".
		// However, some historic servers just blindly copy the value of the `AD` bit without actually validating data.
		// Ho-hum; this is the classic problem of using a 'the data is secure bit'.
		const AuthenticDataBit: u8 = 0b0010_0000;
		
		// Also known as `CD` or `cd`, the checking disabled bit.
		//
		// RFC 6840, Section 5.9, Always Set the CD Bit on Queries conflicts with this setting.
		const CheckingIsEnabled: u8 = 0b0000_0000;
		
		const OkResponseCode: u8 = 0b0000_0000;

		RecursionIsAvailableShouldNotBeSetInQuery | UnassignedBitsMustBeZero | AuthenticDataBit | CheckingIsEnabled | OkResponseCode
	}
}
