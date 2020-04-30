// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
struct seccomp_notif_sizes
{
	/// Actual `size_of()` for `seccomp_notif`.
	seccomp_notif: u16,

	/// Actual `size_of()` for `seccomp_notif_resp`.
	seccomp_notif_resp: u16,

	/// Actual `size_of()` for `seccomp_data`.
	seccomp_data: u16,
}

impl seccomp_notif_sizes
{
	/// Used to get actual sizes.
	#[inline(always)]
	#[allow(deprecated)]
	fn get_listener_notification_sizes() -> &'static seccomp_notif_sizes
	{
		lazy_static!
		{
    		static ref Sizes: seccomp_notif_sizes =
    		{
				let mut sizes: seccomp_notif_sizes = unsafe { uninitialized() };
				let result = seccomp(SECCOMP_GET_NOTIF_SIZES, 0, &mut sizes as *mut seccomp_notif_sizes as *mut _);
				if result == 0
				{
					sizes
				}
				else if unlikely!(result == -1)
				{
					panic!("seccomp() failed with {}", errno())
				}
				else
				{
					unreachable!("seccomp() returned unexpected result {}", result)
				}
    		};
    	}
		&Sizes
	}

}
