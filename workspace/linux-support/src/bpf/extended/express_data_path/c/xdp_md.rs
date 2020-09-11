// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct xdp_md
{
	data: u32,
	data_end: u32,
	data_meta: u32,
	ingress_ifindex: u32,
	rx_queue_index: u32,
}

impl xdp_md
{
	pub(crate) const data: MemoryOffset<'static> = memory_offset_of!(xdp_md, data);
	
	pub(crate) const data_end: MemoryOffset<'static> = memory_offset_of!(xdp_md, data_end);
	
	pub(crate) const data_meta: MemoryOffset<'static> = memory_offset_of!(xdp_md, data_meta);
	
	pub(crate) const ingress_ifindex: MemoryOffset<'static> = memory_offset_of!(xdp_md, ingress_ifindex);
	
	pub(crate) const rx_queue_index: MemoryOffset<'static> = memory_offset_of!(xdp_md, rx_queue_index);
}
