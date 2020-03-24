// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



/// Processor.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum Processor
{
	/// 80386.
	_386,
	
	/// 80486.
	_486,

	/// Pentium.
	Pentium,

	/// Unofficial.
	PentiumAlt,

	/// Unofficial.
	P6,

	/// Alpha.
	Alpha,

	/// PowerPC.
	PowerPC,
	
	/// MIPS.
	Mips,
	
	/// Coprocessor.
	Coprocessor,
}

impl Processor
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		0x00
	}
	
	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::Processor::*;

		match value
		{
			0x00 => zero_programming_interface!(_386, programming_interface),
			0x01 => zero_programming_interface!(_486, programming_interface),
			0x02 => zero_programming_interface!(Pentium, programming_interface),
			0x03 => zero_programming_interface!(PentiumAlt, programming_interface),
			0x04 => zero_programming_interface!(P6, programming_interface),
			0x10 => zero_programming_interface!(Alpha, programming_interface),
			0x20 => zero_programming_interface!(PowerPC, programming_interface),
			0x30 => zero_programming_interface!(Mips, programming_interface),
			0x40 => zero_programming_interface!(Coprocessor, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::Processor::*;

		match self
		{
			_386 => 0x00,
			_486 => 0x01,
			Pentium => 0x02,
			PentiumAlt => 0x03,
			P6 => 0x04,
			Alpha => 0x10,
			PowerPC => 0x20,
			Mips => 0x30,
			Coprocessor => 0x40,
		}
	}
}
