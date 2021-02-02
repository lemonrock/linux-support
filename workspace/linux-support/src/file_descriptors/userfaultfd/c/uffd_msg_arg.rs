// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(super) union uffd_msg_arg
{
	pub(super) pagefault: uffd_msg_arg_pagefault,
	
	pub(super) fork: uffd_msg_arg_fork,
	
	pub(super) remap: uffd_msg_arg_remap,
	
	pub(super) remove: uffd_msg_arg_remove,
	
	pub(super) reserved: uffd_msg_arg_reserved,
}
