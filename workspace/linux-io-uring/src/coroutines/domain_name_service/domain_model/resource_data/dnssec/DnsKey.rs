// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DNS key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DnsKey<OOPB: OwnedOrParsedBytes>
{
	/// Computed key tag.
	pub computed_key_tag: KeyTag,

	/// DNS key purpose.
	pub purpose: DnsKeyPurpose,

	/// Certificate algorithm.
	pub security_algorithm: SecurityAlgorithm,

	/// Certificate type and data.
	pub public_key: OOPB,
}
