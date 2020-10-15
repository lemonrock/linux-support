// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Settings for a listening server.
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct AcceptConnectionsCoroutineSettings<SA: SocketAddress, AC: AccessControl<SA::SD, AccessControlValue>>
{
	/// Service.
	pub service_protocol_identifier: ServiceProtocolIdentifier,
	
	/// Listener socket settings.
	pub transmission_control_protocol_service_listener_settings: TransmissionControlProtocolServerListenerSettings<SA>,
	
	/// Access control.
	pub access_control: AC,
}
