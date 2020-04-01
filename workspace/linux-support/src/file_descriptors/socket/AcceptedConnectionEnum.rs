// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// One of three possible types.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AcceptedConnectionEnum
{
	/// An Internet Protocol (IP) version 4 accepted connection.
	InternetProtocolVersion4(AcceptedConnection<sockaddr_in>),

	/// An Internet Protocol (IP) version 6 accepted connection.
	InternetProtocolVersion6(AcceptedConnection<sockaddr_in6>),

	/// An Unix Domain connection.
	UnixDomain(AcceptedConnection<sockaddr_un>),
}
