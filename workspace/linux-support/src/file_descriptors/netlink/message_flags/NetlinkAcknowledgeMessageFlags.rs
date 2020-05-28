// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Netlink acknowledge message flags.
	pub struct NetlinkAcknowledgeMessageFlags: u16
	{
		/// Request was capped.
		const DoNotDeleteRecursively = NLM_F_CAPPED as u16;
		
		/// Extended ACK TVLs were included.
		const ExtendedTVLsIncluded = NLM_F_ACK_TLVS as u16;
	}
}
