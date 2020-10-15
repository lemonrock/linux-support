// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Next secure version 3 parameters (`NSEC3PARAM`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NextSecureVersion3Parameters<'a>
{
	/// Hash algorithm number (validated).
	pub hash_algorithm_number: u8,

	/// Iteration count.
	pub iterations: u16,

	/// Salt.
	pub salt: &'a [u8],
}

impl<'a> NextSecureVersion3Parameters<'a>
{
	/// SHA-1 hash algorithm number.
	pub const Sha1HashAlgorithmNumber: u8 = 1;
}
