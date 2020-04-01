// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Log a fatal signal number to syslog before exiting.
#[inline(always)]
pub fn log_exit_signalled_to_syslog(signal_number: Option<Signal>)
{
	match signal_number
	{
		None => unsafe { syslog(LOG_NOTICE, b"ExitSignalled:Other\0".as_ptr() as *const _) },

		Some(signal_number) => unsafe { syslog(LOG_NOTICE, b"ExitSignalled:%s\0".as_ptr() as *const _, signal_number.c_library_name().as_ptr()) },
	}
}
