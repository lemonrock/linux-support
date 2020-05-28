// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Netlink get request message flags.
	pub struct NetlinkGetRequestMessageFlags: u16
	{
		/// Get Request: specify tree root.
		const Root = NLM_F_ROOT as u16;
		
		/// Get Request: return all matching.
		const Match = NLM_F_MATCH as u16;
		
		/// Get Request: atomic get.
		#[deprecated]
		const Atomic = NLM_F_ATOMIC as u16;
		
		/// Get Request: specify tree root and return all matching.
		const Dump = Self::Root.bits | Self::Match.bits;
	}
}
