// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Diagnostic information.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct xdp_diag_info
{
	/// ?Duplicates `xdp_diag_umem.ifindex`.
	pub ifindex: Option<NetworkInterfaceIndex>,
	
	/// ?Duplicates `xdp_diag_umem.queue_id`.
	pub queue_id: QueueIdentifier,
}
