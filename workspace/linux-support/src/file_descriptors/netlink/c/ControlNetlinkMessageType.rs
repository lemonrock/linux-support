// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ControlNetlinkMessageType(u8);

impl ControlNetlinkMessageType
{
	pub(crate) const NoOp: Self = Self(NLMSG_NOOP as u8);
	
	pub(crate) const Error: Self = Self(NLMSG_ERROR as u8);
	
	pub(crate) const Done: Self = Self(NLMSG_DONE as u8);
	
	pub(crate) const OverRun: Self = Self(NLMSG_OVERRUN as u8);
}
