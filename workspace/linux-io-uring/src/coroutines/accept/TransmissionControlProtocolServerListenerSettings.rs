// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Settings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransmissionControlProtocolServerListenerSettings<SA: SocketAddress>
{
	/// Socket address (Internet Protocol address and port number).
	pub socket_address: SA,
	
	/// Internet Protocol (IP) socket settings.
	#[serde(default)] pub internet_protocol: InternetProtocolSocketSettings,
	
	/// Transmission Control Protocol (TCP) socket settings.
	#[serde(default)] pub transmission_control_protocol: TransmissionControlProtocolSocketSettings,
	
	/// Back log.
	#[serde(default)] pub back_log: BackLog,
}

impl<SA: SocketAddress> TransmissionControlProtocolServerListenerSettings<SA>
{
	/// Default-ish.
	#[inline(always)]
	pub fn defaultish(socket_address: SA) -> Self
	{
		Self
		{
			socket_address,
			internet_protocol: InternetProtocolSocketSettings::default(),
			transmission_control_protocol: TransmissionControlProtocolSocketSettings::default(),
			back_log: BackLog::default(),
		}
	}
	
	/// Must be run on the thread the socket is to be created on.
	#[inline(always)]
	pub(crate) fn new_socket(self, our_hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<SA::SD>, NewSocketServerListenerError>
	{
		self.socket_address.new_transmission_control_protocol_server_listener
		(
			&self.internet_protocol,
			&self.transmission_control_protocol,
			self.back_log,
			&Blocking::Blocking
			{
				send: BlockingDuration::BlocksForever,
				receive: BlockingDuration::BlocksForever,
			},
			our_hyper_thread,
		)
	}
}
