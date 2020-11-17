// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Next secure (`NSEC`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NextSecure<N: Name>
{
	/// Next domain name.
	pub next_domain_name: N,

	/// Type bitmaps.
	pub type_bitmaps: TypeBitmaps,
}
