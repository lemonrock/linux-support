// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Link Speed, also known as Transfer Rate.
///
/// Linux as of 24 March 2019 does not support for PCI Express 6.0's 64 GT/s speed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LinkSpeed
{
	/// 32 GT/s (PCI Express 5.0).
	_32,

	/// 16 GT/s (PCI Express 4.0).
	_16,

	/// 8 GT/s (PCI Express 3.0).
	_8,

	/// 5 GT/s (PCI Express 2.0).
	_5,

	/// 2.5 GT/s (PCI Express 1.0).
	_2dot5,

	/// Unknown speed.
	Unknown,
}

impl FromStr for LinkSpeed
{
	type Err = LinkSpeedFromStrError;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		use self::LinkSpeed::*;

		match s
		{
			"32 GT/s" => Ok(_32),
			"16 GT/s" => Ok(_16),
			"8 GT/s" => Ok(_8),
			"5 GT/s" => Ok(_5),
			"2.5 GT/s" => Ok(_2dot5),
			"Unknown speed" => Ok(Unknown),
			_ => Err(LinkSpeedFromStrError::Unrecognised(s.to_string()))
		}
	}
}
