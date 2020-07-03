// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is a bitfield.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DestinationAndSourceRegisters(u8);

impl DestinationAndSourceRegisters
{
	#[cfg(target_endian = "little")]
	#[inline(always)]
	const fn new(destination_register: Register, source_register: Register) -> Self
	{
		let dst_reg = destination_register as u8;
		let src_reg = source_register as u8;
		Self(((src_reg & 0b1111) << 4) | (dst_reg & 0b1111))
	}
	
	#[cfg(target_endian = "big")]
	#[inline(always)]
	const fn new(destination_register: Register, source_register: Register) -> Self
	{
		let dst_reg = destination_register as u8;
		let src_reg = source_register as u8;
		Self((src_reg & 0b1111) | ((dst_reg & 0b1111) << 4))
	}
}
