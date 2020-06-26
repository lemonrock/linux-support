// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Used for command `BPF_LINK_CREATE`.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct BpfCommandLinkCreate
{
	/// eBPF program to attach.
	pub(crate) prog_fd: u32,
	
	/// object to attach to.
	pub(crate) target_fd: u32,
	
	/// attach type.
	pub(crate) attach_type: u32,
	
	/// extra flags.
	pub(crate) flags: u32,
}
