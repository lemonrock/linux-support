// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Processing state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetExpressDataPathDiagnosticsProcessingMessageState
{
	/// eg `SOCK_STREAM`.
	///
	/// Values are in internal Linux enum `enum sock_type` in the source file `include/linux/net.h`.
	socket_type: u8,
	
	inode: u32,
	
	socket_cookie: u64,
	
	pub(crate) basic_information: Option<xdp_diag_info>,
	
	pub(crate) user_identifier: Option<UserIdentifier>,
	
	pub(crate) receive_ring_number_of_descriptors: Option<u32>,
	
	pub(crate) transmit_ring_number_of_descriptors: Option<u32>,
	
	pub(crate) fill_ring_number_of_descriptors: Option<u32>,
	
	pub(crate) completion_ring_number_of_descriptors: Option<u32>,
	
	pub(crate) user_memory_information: Option<xdp_diag_umem>,
	
	pub(crate) socket_memory_information: Option<HashMap<SK_MEMINFO_, u32>>,
	
	pub(crate) statistics: Option<xdp_diag_stats>,
}

#[allow(missing_docs)]
impl GetExpressDataPathDiagnosticsProcessingMessageState
{
	#[inline(always)]
	pub(crate) fn new(message_header: &xdp_diag_msg) -> Result<Self, String>
	{
		if unlikely!(message_header.xdiag_family != AF_XDP as u8)
		{
			return Err(format!("Linux kernel bug - xdiag_family is not AF_XDP"))
		}
		Ok
		(
			Self
			{
				socket_type: message_header.xdiag_type,
				
				inode: message_header.xdiag_ino,
				
				socket_cookie:
				{
					let cookie = message_header.xdiag_cookie;
					((cookie[1] as u64) << 32)| (cookie[0] as u64)
				},
				
				basic_information: None,
			
				user_identifier: None,
				
				receive_ring_number_of_descriptors: None,
				
				transmit_ring_number_of_descriptors: None,
				
				fill_ring_number_of_descriptors: None,
				
				completion_ring_number_of_descriptors: None,
				
				user_memory_information: None,
				
				socket_memory_information: None,
				
				statistics: None,
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn to_processed_message(self) -> Result<GetExpressDataPathDiagnosticsMessageData, String>
	{
		Ok
		(
			GetExpressDataPathDiagnosticsMessageData
			{
				socket_type: self.socket_type,
				
				inode: Inode::from(self.inode as ino_t),
				
				socket_cookie: self.socket_cookie,
				
				basic_information: self.basic_information,
				
				user_identifier: self.user_identifier,
				
				receive_ring_number_of_descriptors: self.receive_ring_number_of_descriptors,
				
				transmit_ring_number_of_descriptors: self.transmit_ring_number_of_descriptors,
				
				fill_ring_number_of_descriptors: self.fill_ring_number_of_descriptors,
				
				completion_ring_number_of_descriptors: self.completion_ring_number_of_descriptors,
				
				user_memory_information: self.user_memory_information,
				
				socket_memory_information: self.socket_memory_information,
				
				statistics: self.statistics,
			}
		)
	}
}
