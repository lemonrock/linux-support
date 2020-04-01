// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// UTS stands for Unix Time-Sharing.
#[inline(always)]
pub fn setup_uts_namespace()
{
	// NOTE: Unusually, this C function does not require a string with a terminating NUL byte.
	static FixedHostName: &'static [u8] = b"stormmq";
	unsafe { sethostname(FixedHostName.as_ptr() as *const c_char, FixedHostName.len()) };

	// NOTE: Unusually, this C function does not require a string with a terminating NUL byte.
	static FixedNetworkInformationServiceDomainName: &'static [u8] = b"com";
	unsafe { setdomainname(FixedNetworkInformationServiceDomainName.as_ptr() as *const c_char, FixedNetworkInformationServiceDomainName.len()) };
}
