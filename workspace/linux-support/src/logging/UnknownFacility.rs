// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 5424, Section 6.2.1: "Facility values MUST be in the range of 0 to 23 inclusive".
///
/// However, facilities 24 to 31 inclusive are not permissible but have historically been used, eg on Mac OS X.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum UnknownFacility
{
	_24 = 24,
	_25 = 25,
	_26 = 26,
	_27 = 27,
	_28 = 28,
	_29 = 29,
	_30 = 30,
	_31 = 31,
}
