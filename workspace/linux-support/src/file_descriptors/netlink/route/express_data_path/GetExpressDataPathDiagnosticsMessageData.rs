// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Message data.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetExpressDataPathDiagnosticsMessageData
{
	/// eg `SOCK_STREAM`.
	///
	/// Values are in internal Linux enum `enum sock_type` in the source file `include/linux/net.h`.
	pub socket_type: u8,
	
	/// Inode.
	pub inode: Inode,
	
	/// Cookie.
	pub socket_cookie: u64,
	
	#[allow(missing_docs)]
	pub basic_information: Option<xdp_diag_info>,
	
	#[allow(missing_docs)]
	pub user_identifier: Option<UserIdentifier>,
	
	#[allow(missing_docs)]
	pub receive_ring_number_of_descriptors: Option<u32>,
	
	#[allow(missing_docs)]
	pub transmit_ring_number_of_descriptors: Option<u32>,
	
	#[allow(missing_docs)]
	pub fill_ring_number_of_descriptors: Option<u32>,
	
	#[allow(missing_docs)]
	pub completion_ring_number_of_descriptors: Option<u32>,
	
	#[allow(missing_docs)]
	pub user_memory_information: Option<xdp_diag_umem>,
	
	#[allow(missing_docs)]
	pub socket_memory_information: Option<HashMap<SK_MEMINFO_, u32>>,
	
	#[allow(missing_docs)]
	pub statistics: Option<xdp_diag_stats>,
}
