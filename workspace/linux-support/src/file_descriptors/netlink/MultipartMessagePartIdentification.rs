// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Identifies a set of messages.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MultipartMessagePartIdentification
{
	sequence_number: SequenceNumber,
	process_identifier: PortIdentifier,
}

impl MultipartMessagePartIdentification
{
	#[inline(always)]
	fn new(reply_message: &nlmsghdr) -> Self
	{
		Self
		{
			sequence_number: reply_message.nlmsg_seq,
			process_identifier: reply_message.nlmsg_pid,
		}
	}
	
	/// From Linux kernel.
	#[inline(always)]
	pub const fn from_linux_kernel(sequence_number: SequenceNumber) -> Self
	{
		Self
		{
			sequence_number,
			process_identifier: PortIdentifier::LinuxKernel,
		}
	}
}
