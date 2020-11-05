// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Next secure version 3 (`NSEC3`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NextSecureVersion3<SHA1: OwnedOrParsed<Sha1>>
{
	/// Opt-Out.
	pub opt_out: bool,

	/// Iteration count.
	pub iterations: u16,

	/// Salt.
	pub salt: &'a [u8],

	/// Next owner name, hashed.
	pub next_hashed_owner_name: NextSecureVersion3Hash<SHA1>,

	/// Type bitmaps.
	pub type_bitmaps: TypeBitmaps,
}
