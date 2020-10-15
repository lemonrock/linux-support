// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A naming authority pointer record header.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamingAuthorityPointerHeader<'a>
{
	/// Record ordering.
	pub order: u16,

	/// Record weighting, as for `MX` or `KX` records.
	pub preference: u16,

	/// Flags; should consist of bytes A-Z and 0-9 but this is **not validated**.
	pub flags: &'a [u8],

	/// Arbitrary string data, not zero-terminated.
	pub services: &'a [u8],
}
