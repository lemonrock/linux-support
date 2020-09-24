// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Layout of structure passed to eXpress Data Path (XDP) programs' `main()` method.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct xdp_md
{
	#[allow(missing_docs)]
	data: u32,
	
	#[allow(missing_docs)]
	data_end: u32,
	
	#[allow(missing_docs)]
	data_meta: u32,
	
	#[allow(missing_docs)]
	ingress_ifindex: Option<NetworkInterfaceIndex>,
	
	#[allow(missing_docs)]
	rx_queue_index: ExpressDataPathQueueIdentifier,
}

impl xdp_md
{
	/// Offset of `data` field for use with eBPF assembler instructions that access struct fields.
	pub const data: MemoryOffset<'static> = memory_offset_of!(xdp_md, data);
	
	/// Offset of `data_end` field for use with eBPF assembler instructions that access struct fields.
	pub const data_end: MemoryOffset<'static> = memory_offset_of!(xdp_md, data_end);
	
	/// Offset of `data_meta` field for use with eBPF assembler instructions that access struct fields.
	pub const data_meta: MemoryOffset<'static> = memory_offset_of!(xdp_md, data_meta);
	
	/// Offset of `ingress_ifindex` field for use with eBPF assembler instructions that access struct fields.
	pub const ingress_ifindex: MemoryOffset<'static> = memory_offset_of!(xdp_md, ingress_ifindex);
	
	/// Offset of `rx_queue_index` field for use with eBPF assembler instructions that access struct fields.
	pub const rx_queue_index: MemoryOffset<'static> = memory_offset_of!(xdp_md, rx_queue_index);
}
