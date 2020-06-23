// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Remote peer address-based access control.
///
/// Holds whitelists.
#[derive(Debug, Clone)]
pub struct UnixDomainSocketAccessControl<Value>
{
	permitted_unix_domain_group_identifiers: HashMap<GroupIdentifier, Arc<Value>>,
	permitted_unix_domain_user_identifiers: HashMap<UserIdentifier, Arc<Value>>,
}

impl<Value> UnixDomainSocketAccessControl<Value>
{
	/// Creates a new instance.
	///
	/// Permitted lists are `Option`s.
	/// If they are `None`, then the permitted list is not checked and all possible values are permitted (as long as the accompanying deny list does not deny them).
	#[inline(always)]
	pub fn new
	(
		permitted_unix_domain_group_identifiers: HashMap<GroupIdentifier, Arc<Value>>,
		permitted_unix_domain_user_identifiers: HashMap<UserIdentifier, Arc<Value>>,
	) -> Self
	{
		Self
		{
			permitted_unix_domain_group_identifiers,
			permitted_unix_domain_user_identifiers,
		}
	}
}

impl<Value> AccessControl<sockaddr_un, Value> for UnixDomainSocketAccessControl<Value>
{
	#[inline(always)]
	fn is_remote_peer_allowed(&self, accepted_connection: &AcceptedConnection<sockaddr_un>) -> Option<&Arc<Value>>
	{
		let credentials = accepted_connection.streaming_socket_file_descriptor.remote_peer_credentials();
		
		if let Some(value) = self.permitted_unix_domain_group_identifiers.get(&credentials.group_identifier)
		{
			return Some(value)
		}
		
		self.permitted_unix_domain_user_identifiers.get(&credentials.user_identifier)
	}
}
