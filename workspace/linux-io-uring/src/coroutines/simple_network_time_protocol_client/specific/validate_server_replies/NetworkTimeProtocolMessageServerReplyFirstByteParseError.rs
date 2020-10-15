// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum NetworkTimeProtocolMessageServerReplyFirstByteParseError
{
	/// AssociationMode was not `Server`.
	InvalidAssociationMode(AssociationMode),
	
	/// For versions `1` to `3` inclusive.
	ObsoleteVersionNumber(VersionNumber),
	
	/// For values `0` and `5` to `7` inclusive.
	UnknownVersionNumber(u8),
}

impl Display for NetworkTimeProtocolMessageServerReplyFirstByteParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formmater) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NetworkTimeProtocolMessageServerReplyFirstByteParseError
{
}
