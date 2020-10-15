// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a `Locator` for ILNPv4 along with its preference.
///
/// Used in a `L32` record; logically equivalent to an `A` record.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Locator32
{
	/// Indicates the owner name's relative preference for record among other records associated with this owner name.
	///
	/// Lower preference values are preferred over higher preference values.
	pub preference: u16,

	/// `Locator32`.
	///
	/// Identical on-the-wire to the `ADDRESS` field of an existing DNS `A` record.
	pub locator: Ipv4Addr,
}
