// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C, packed)]
pub(crate) struct ethhdr
{
	/// destination eth addr.
	pub(crate) h_dest: [c_uchar; ETH_ALEN],
	
	/// source ether addr.
	pub(crate) h_source: [c_uchar; ETH_ALEN],
	
	/// packet type ID field.
	pub(crate) h_proto: BigEndianU16,
}

impl FlowSpecification for ethhdr
{
}
