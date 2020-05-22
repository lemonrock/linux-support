// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 5424, Section 6.2.2: "The VERSION field denotes the version of the syslog protocol specification.
/// The version number MUST be incremented for any new syslog protocol specification that changes any part of the HEADER format.
/// Changes include the addition or removal of fields, or a change of syntax or semantics of existing fields.
/// This document uses a VERSION value of "1"".
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Version
{
	_1 = 1,
}
