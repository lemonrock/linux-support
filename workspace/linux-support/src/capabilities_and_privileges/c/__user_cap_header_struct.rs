// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
#[repr(C)]
pub(super) struct __user_cap_header_struct
{
	version: u32,
	pid: pid_t,
}

impl __user_cap_header_struct
{
	#[inline(always)]
	pub(super) fn new(thread_identifier: ThreadIdentifier) -> Self
	{
		Self
		{
			version: _LINUX_CAPABILITY_VERSION_3,
			pid: thread_identifier.into(),
		}
	}
}
