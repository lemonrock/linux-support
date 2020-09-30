// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents cache information.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct ifa_cacheinfo
{
	/// Temporary preferred address lifetime.
	///
	/// `Infinite` for a permanent address.
	pub(crate) ifa_prefered: InternetProtocolAddressLifetime,
	
	/// Temporary valid address lifetime.
	///
	/// `Infinite` for a permanent address.
	pub(crate) ifa_valid: InternetProtocolAddressLifetime,
	
	/// Created timestamp in hundredths of seconds.
	pub(crate) cstamp: CacheTimestampInHundrethsOfSeconds,
	
	/// Updated timestamp in hundredths of seconds.
	pub(crate) tstamp: CacheTimestampInHundrethsOfSeconds,
}
