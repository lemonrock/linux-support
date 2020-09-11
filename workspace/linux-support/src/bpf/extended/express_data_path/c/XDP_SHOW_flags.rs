// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	pub(crate) struct XDP_SHOW_flags: u32
	{
		const BasicInformation = XDP_SHOW_INFO;
		
		const RingConfiguration = XDP_SHOW_RING_CFG;
		
		const UserMemory = XDP_SHOW_UMEM;
		
		const SocketMemoryInformation = XDP_SHOW_MEMINFO;
		
		const Statistics = XDP_SHOW_STATS;
	}
}
