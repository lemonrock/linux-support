// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a `Locator` for ILNPv6 along with its preference.
///
/// Used in a `L32` record; similar in purpose to an `A` record.
///
/// Has the same syntax and semantics as a 64-bit Internet Protocol version 6 routing prefix.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Locator64(pub [u8; 8]);

impl Display for NodeIdentifier
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{:04X}:", (self.0 >> 48) as u16)?;
		write!(f, "{:04X}:", ((self.0 & 0x0000_FFFF_0000_0000) >> 32) as u16)?;
		write!(f, "{:04X}:", ((self.0 & 0x0000_0000_FFFF_0000) >> 16) as u16)?;
		write!(f, "{:04X}", (self.0 & 0x0000_0000_0000_FFFF) as u16)?;
		Ok(())
	}
}
