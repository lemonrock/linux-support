// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An attempt to synthesize the various ways a transport protocol is represent by NAPTR records!
pub trait ToNamingAuthorityCommonTransportProtocol
{
	/// An attempt to synthesize the various ways a transport protocol is represent by NAPTR records!
	fn to_naming_authority_common_transport_protocol(self) -> NamingAuthorityCommonTransportProtocol;
}
