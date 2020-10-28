// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Big-endian (network byte order) u32.
pub type BigEndianU32 = [u8; 4];

impl FromNetworkEndianToNativeEndian for BigEndianU32
{
	type NumericType = u32;
	
	#[inline(always)]
	fn from_network_endian_to_native_endian(self) -> Self::NumericType
	{
		Self::NumericType::from_be_bytes(self)
	}
}
