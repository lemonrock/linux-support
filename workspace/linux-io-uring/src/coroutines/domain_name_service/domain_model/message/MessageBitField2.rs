// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
	fn raw_response_code(self) -> u8
	{
		self.0 & 0b0000_1111
	}

	#[inline(always)]
	const fn new_for_query() -> u8
	{
		const RecursionIsAvailableShouldNotBeSetInQuery: u8 = 0b0000_0000;
		const UnassignedBitsMustBeZero: u8 = 0b0000_0000;
		const AuthenticDataBitShouldNotBeSetInQuery: u8 = 0b0000_0000;
		const CheckingIsEnabled: u8 = 0b0000_0000;
		const OkResponseCode: u8 = 0b0000_0000;

		RecursionIsAvailableShouldNotBeSetInQuery | UnassignedBitsMustBeZero | AuthenticDataBitShouldNotBeSetInQuery | CheckingIsEnabled | OkResponseCode
	}
}
