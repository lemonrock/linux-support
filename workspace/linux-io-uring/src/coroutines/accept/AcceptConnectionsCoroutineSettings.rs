// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct AcceptConnectionsCoroutineSettings<SA: SocketAddress, AC: AccessControl<SA::SD, AccessControlValue>>
{
	pub transmission_control_protocol_service_listener_settings: TransmissionControlProtocolServerListenerSettings<SA>,
	
	pub remote_peer_address_based_access_control: AC,
	
	pub service_protocol_identifier: ServiceProtocolIdentifier,
}

impl<SA: SocketAddress, AC: AccessControl<SA::SD, AccessControlValue>> AcceptConnectionsCoroutineSettings<SA, AC>
{
	#[inline(always)]
	pub(crate) fn new_socket(self, our_hyper_thread: HyperThread) -> Result<StreamingServerListenerSocketFileDescriptor<SA::SD>, NewSocketServerListenerError>
	{
		self.transmission_control_protocol_service_listener_settings.new_socket(our_hyper_thread)
	}
	
	#[inline(always)]
	pub(crate) fn remote_peer_adddress_based_access_control(self) -> AC
	{
		self.remote_peer_address_based_access_control
	}
	
	#[inline(always)]
	pub(crate) fn service_protocol_identifier(self) -> ServiceProtocolIdentifier
	{
		self.service_protocol_identifier
	}
}
