// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct AcceptServiceConfiguration
{
	pub internet_protocol_version_4: Vec<TransmissionControlProtocolServerListenerSettings<SocketAddrV4>>,
	pub permitted_internet_protocol_version_4_subnets: BTreeMap<InternetProtocolAddressWithMask<in_addr>, Arc<AccessControlValue>>,
	
	pub internet_protocol_version_6: Vec<TransmissionControlProtocolServerListenerSettings<SocketAddrV6>>,
	pub permitted_internet_protocol_version_6_subnets: BTreeMap<InternetProtocolAddressWithMask<in6_addr>, Arc<AccessControlValue>>,

	pub unix_domain_socket: Vec<TransmissionControlProtocolServerListenerSettings<UnixSocketAddress<PathBuf>>>,
	pub permitted_unix_domain_group_identifiers: HashMap<GroupIdentifier, Arc<AccessControlValue>>,
	pub permitted_unix_domain_user_identifiers: HashMap<UserIdentifier, Arc<AccessControlValue>>,
}

impl AcceptServiceConfiguration
{
	pub fn server_listeners
	(
		&mut self,
		service_protocol_identifier: ServiceProtocolIdentifier,
		transmission_control_protocol_over_internet_protocol_version_4_server_listeners: &mut Vec<AcceptConnectionsCoroutineSettings<SocketAddrV4, InternetProtocolVersion4AccessControl<AccessControlValue>>>,
		transmission_control_protocol_over_internet_protocol_version_6_server_listeners: &mut Vec<AcceptConnectionsCoroutineSettings<SocketAddrV6, InternetProtocolVersion6AccessControl<AccessControlValue>>>,
		streaming_unix_domain_socket_server_listener_server_listeners: &mut Vec<AcceptConnectionsCoroutineSettings<UnixSocketAddress<PathBuf>, UnixDomainSocketAccessControl<AccessControlValue>>>,
	)
	{
		for transmission_control_protocol_service_listener_settings in self.internet_protocol_version_4.drain(..)
		{
			transmission_control_protocol_over_internet_protocol_version_4_server_listeners.push
			(
				AcceptConnectionsCoroutineSettings
				{
					service_protocol_identifier,
					transmission_control_protocol_service_listener_settings,
					access_control: InternetProtocolVersion4AccessControl::new(&self.permitted_internet_protocol_version_4_subnets)
				}
			)
		}
		
		for transmission_control_protocol_service_listener_settings in self.internet_protocol_version_6.drain(..)
		{
			transmission_control_protocol_over_internet_protocol_version_6_server_listeners.push
			(
				AcceptConnectionsCoroutineSettings
				{
					service_protocol_identifier,
					transmission_control_protocol_service_listener_settings,
					access_control: InternetProtocolVersion6AccessControl::new(&self.permitted_internet_protocol_version_6_subnets)
				}
			)
		}
		
		for transmission_control_protocol_service_listener_settings in self.unix_domain_socket.drain(..)
		{
			streaming_unix_domain_socket_server_listener_server_listeners.push
			(
				AcceptConnectionsCoroutineSettings
				{
					service_protocol_identifier,
					transmission_control_protocol_service_listener_settings,
					access_control: UnixDomainSocketAccessControl::new(self.permitted_unix_domain_group_identifiers.clone(), self.permitted_unix_domain_user_identifiers.clone())
				}
			)
		}
	}
}
