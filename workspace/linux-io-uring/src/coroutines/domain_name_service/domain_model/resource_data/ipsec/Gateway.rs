// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A gateway associated with an IPsec public key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gateway<'label, N: Name<'label>>
{
	/// As an Internet Protocol version 6 address.
	InternetProtocolVersion4(Ipv4Addr),

	/// As an Internet Protocol version 6 address.
	InternetProtocolVersion6(Ipv6Addr),

	/// As a domain name.
	DomainName(N, PhantomData<&'label N>)
}
