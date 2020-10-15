// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Version Number (VN).
///
/// 3 bits in size (0 to 7 inclusive; only values 3 and 4 have meaning as of RFC 4330).
///
/// Values 0, 1 and 2 were never used (prior version of NTP, ed RFC 958, did not define this field value).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum VersionNumber
{
	/// Version 1.
	///
	/// Obsolete.
	#[deprecated]
	Version1 = 1,
	
	/// Version 2.
	///
	/// Obsolete.
	#[deprecated]
	Version2 = 2,
	
	/// Version 3 as defined in RFC 1305.
	///
	/// Obsolete.
	#[deprecated]
	Version3 = 3,
	
	/// Version 4 as defined in RFC 4330 and updated by RFC 5905 Section 14.
	Version4 = 4,
}
