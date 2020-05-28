// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Netlink new (or set) request message flags.
	pub(super) struct NetlinkNewRequestMessageFlags: u16
	{
		/// New Request: Override existing.
		const Replace = NLM_F_REPLACE as u16;
		
		/// New Request: do not touch, if it exists.
		const Exclusive = NLM_F_EXCL as u16;
		
		/// New Request: create, if it does not exist.
		const Create = NLM_F_CREATE as u16;
		
		/// New Request: add to end of list.
		const Append = NLM_F_APPEND as u16;
	}
}
