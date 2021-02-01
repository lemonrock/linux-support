// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C, packed)]
pub(super) struct uffd_msg
{
	pub(super) event: UserFaultEvent,
	
	reserved1: u8,
	
	reserved2: u16,
	
	reserved3: u32,
	
	pub(super) arg: uffd_msg_arg,
}
