// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Common Netlink message flags.
	pub(super) struct NetlinkCommonMessageFlags: u16
	{
		/// Request message.
		const Request = NLM_F_REQUEST as u16;
		
		/// Multipart message, terminated by `NLMSG_DONE`.
		///
		/// Set by kernel in messages to user space.
		const Multipart = NLM_F_MULTI as u16;
		
		/// Reply with ack, with zero or error code.
		const Acknowledge = NLM_F_ACK as u16;
		
		/// Echo this request.
		const Echo = NLM_F_ECHO as u16;
		
		/// Dump was inconsistent due to sequence change.
		///
		/// Set by kernel in messages to user space.
		const DumpInterrupted = NLM_F_DUMP_INTR as u16;
		
		/// Dump was filtered as requested.
		///
		/// ?Set by kernel in messages to user space?
		const DumpFiltered = NLM_F_DUMP_FILTERED as u16;
	}
}

impl NetlinkCommonMessageFlags
{
	#[inline(always)]
	pub(super) fn is_multipart(self) -> bool
	{
		self.contains(NetlinkCommonMessageFlags::Multipart)
	}
	
	#[inline(always)]
	pub(super) fn was_dump_interrupted(self) -> bool
	{
		self.contains(NetlinkCommonMessageFlags::DumpInterrupted)
	}
	
	#[inline(always)]
	pub(super) fn acknowledgment_required(self) -> bool
	{
		self.contains(NetlinkCommonMessageFlags::Acknowledge)
	}
}
