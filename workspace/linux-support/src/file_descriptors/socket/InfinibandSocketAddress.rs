// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents an infiniband socket address.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InfinibandSocketAddress(ib_addr);

impl SocketAddress for InfinibandSocketAddress
{
	#[inline(always)]
	fn new_transmission_control_protocol_server_listener(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16, back_log: u32, non_blocking: bool, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self>, NewSocketServerListenerError>
	{
		unimplemented!("Not yet needed")
	}
	
	#[inline(always)]
	fn new_transmission_control_protocol_client(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16, non_blocking: bool) -> Result<StreamingSocketFileDescriptor<Self>, NewSocketClientError>
	{
		unimplemented!("Not yet needed")
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_server_listener(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, non_blocking: bool) -> Result<DatagramServerListenerSocketFileDescriptor<Self>, NewSocketServerListenerError>
	{
		unimplemented!("Not yet needed")
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_client(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, non_blocking: bool) -> Result<DatagramClientSocketFileDescriptor<Self>, NewSocketClientError>
	{
		unimplemented!("Not yet needed")
	}
}

impl InfinibandSocketAddress
{
	/// `sib_subnet_prefix` and `sib_interface_id` must be a 64-bit integer in Network Endian form, not Native Endian form.
	#[inline(always)]
	pub const fn new(sib_subnet_prefix: u64, sib_interface_id: u64) -> Self
	{
		Self
		(
			ib_addr
			{
				sib_subnet_prefix,
				sib_interface_id
			}
		)
	}

	/// The 'any' address.
	pub const Any: Self = Self(ib_addr::Any);

	/// The 'loopback' address.
	pub const Loopback: Self = Self(ib_addr::Loopback);

	/// Is 'any' address.
	#[inline(always)]
	pub fn is_any(&self) -> bool
	{
		self.0.is_any()
	}

	/// Is 'loopback' address.
	#[inline(always)]
	pub fn is_loopback(&self) -> bool
	{
		self.0.is_loopback()
	}
}
