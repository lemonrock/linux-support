// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
#[derive(Deserialize, Serialize)]
pub struct xdp_diag_umem
{
	size: u64,
	
	id: u32,
	
	num_pages: u32,
	
	chunk_size: u32,
	
	headroom: u32,
	
	/// ?Duplicates `xdp_diag_info.ifindex`.
	ifindex: Option<NetworkInterfaceIndex>,
	
	/// ?Duplicates `xdp_diag_info.queue_id`.
	queue_id: QueueIdentifier,
	
	/// `XDP_DU_F_ZEROCOPY` is the only valid flag.
	flags: XdpDiagnosticUserMemoryFlags,
	
	refs: u32,
}
