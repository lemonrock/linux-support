// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::syscall::SystemCallResult;

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
	fn get_listener_notification_sizes() -> &'static seccomp_notif_sizes
	{
		lazy_static!
		{
    		static ref Sizes: seccomp_notif_sizes =
    		{
				let mut sizes: MaybeUninit<seccomp_notif_sizes> = MaybeUninit::uninit();
				match system_call_seccomp(SECCOMP_GET_NOTIF_SIZES, 0, &mut sizes as *mut seccomp_notif_sizes as *mut _).as_usize()
				{
					0 => unsafe { sizes.assume_init() },
					
					error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(seccomp, SystemCallResult::usize_to_system_call_error_number(error)),
					
					unexpected @ _ => unexpected_error!(seccomp, unexpected),
				}
    		};
    	}
		&Sizes
	}

}
