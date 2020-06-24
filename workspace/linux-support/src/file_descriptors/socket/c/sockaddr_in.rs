// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Whilst this is present in libc, it does not support useful derives.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct sockaddr_in
{
	/// Socket address family.
	sin_family: sa_family_t,

	/// Must a 16-bit integer in Network Endian form, not Native Endian form.
	pub sin_port: in_port_t,

	/// Address.
	pub sin_addr: in_addr,

	/// Must always be zero.
	pub sin_zero: [u8; 8],
}

impl Default for sockaddr_in
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			sin_family: AF_INET as sa_family_t,
			sin_port: 0,
			sin_addr: Default::default(),
			sin_zero: unsafe { zeroed() },
		}
	}
}

impl AsRef<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn as_ref(&self) -> &SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl AsMut<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl Borrow<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn borrow(&self) -> &SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl BorrowMut<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl Deref for sockaddr_in
{
	type Target = SocketAddrV4;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { transmute(self) }
	}
}

impl DerefMut for sockaddr_in
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { transmute(self) }
	}
}

impl From<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn from(value: SocketAddrV4) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl<'a> From<&'a SocketAddrV4> for &'a sockaddr_in
{
	#[inline(always)]
	fn from(value: &'a SocketAddrV4) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl Into<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn into(self) -> SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl<'a> Into<&'a SocketAddrV4> for &'a sockaddr_in
{
	#[inline(always)]
	fn into(self) -> &'a SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl Into<SocketAddr> for sockaddr_in
{
	#[inline(always)]
	fn into(self) -> SocketAddr
	{
		SocketAddr::V4(self.into())
	}
}

impl SocketData for sockaddr_in
{
	type Address = in_addr;
	
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		self.sin_family
	}
	
	#[inline(always)]
	fn address(&self) -> &Self::Address
	{
		&self.sin_addr
	}
	
	#[inline(always)]
	fn display_format(&self, f: &mut Formatter, _address_length: usize) -> fmt::Result
	{
		write!(f, "ipv4:{}:{}", self.sin_addr, self.sin_port)
	}
}

impl SocketAddress for sockaddr_in
{
	type SD = Self;
	
	#[inline(always)]
	fn new_transmission_control_protocol_server_listener(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, back_log: BackLog, blocking: &Blocking, hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<Self::SD>::new_transmission_control_protocol_over_internet_protocol_version_4_server_listener(self, internet_protocol_socket_settings, transmission_control_protocol_socket_settings, back_log, blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_transmission_control_protocol_client(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, transmission_control_protocol_socket_settings: &TransmissionControlProtocolSocketSettings, writes_before_reading: bool, blocking: &Blocking) -> Result<StreamingSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		SocketFileDescriptor::<Self::SD>::new_transmission_control_protocol_over_internet_protocol_version_4_client(self, internet_protocol_socket_settings, transmission_control_protocol_socket_settings, writes_before_reading, blocking)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_server_listener(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking, hyper_thread: HyperThread) -> Result<DatagramServerListenerSocketFileDescriptor<Self::SD>, NewSocketServerListenerError>
	{
		SocketFileDescriptor::<Self::SD>::new_user_datagram_protocol_over_internet_protocol_version_4_server_listener(self, internet_protocol_socket_settings, blocking, hyper_thread)
	}
	
	#[inline(always)]
	fn new_user_datagram_protocol_client(&self, internet_protocol_socket_settings: &InternetProtocolSocketSettings, blocking: &Blocking) -> Result<DatagramClientSocketFileDescriptor<Self::SD>, NewSocketClientError>
	{
		SocketFileDescriptor::<Self::SD>::new_user_datagram_protocol_over_internet_protocol_version_4_client(self, internet_protocol_socket_settings, blocking)
	}
}
