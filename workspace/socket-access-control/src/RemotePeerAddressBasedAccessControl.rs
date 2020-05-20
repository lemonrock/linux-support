// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Remote peer address-based access control.
///
/// Holds whitelists.
pub struct RemotePeerAddressBasedAccessControl<Value>
{
	permitted_protocol_version_4_subnets: LongestPrefixMatchTable<in_addr, Value>,
	permitted_protocol_version_6_subnets: LongestPrefixMatchTable<in6_addr, Value>,
	permitted_unix_domain_group_identifiers: HashMap<GroupIdentifier, Value>,
	permitted_unix_domain_user_identifiers: HashMap<UserIdentifier, Value>,
}

impl<Value> RemotePeerAddressBasedAccessControl<Value>
{
	/// Creates a new instance.
	///
	/// Permitted lists are `Option`s.
	/// If they are `None`, then the permitted list is not checked and all possible values are permitted (as long as the accompanying deny list does not deny them).
	#[inline(always)]
	pub fn new
	(
		permitted_protocol_version_4_subnets: &BTreeMap<InternetProtocolAddressWithMask<in_addr>, Rc<Value>>,
		permitted_protocol_version_6_subnets: &BTreeMap<InternetProtocolAddressWithMask<in6_addr>, Rc<Value>>,
		permitted_unix_domain_group_identifiers: HashMap<GroupIdentifier, Value>,
		permitted_unix_domain_user_identifiers: HashMap<UserIdentifier, Value>,
	) -> Self
	{
		Self
		{
			permitted_protocol_version_4_subnets: LongestPrefixMatchTable::new(permitted_protocol_version_4_subnets),
			permitted_protocol_version_6_subnets: LongestPrefixMatchTable::new(permitted_protocol_version_6_subnets),
			permitted_unix_domain_group_identifiers,
			permitted_unix_domain_user_identifiers,
		}
	}
}

impl<Value> AccessControl<sockaddr_in, Value> for RemotePeerAddressBasedAccessControl<Value>
{
	#[inline(always)]
	fn is_remote_peer_allowed(&self, remote_peer_address: sockaddr_in, _streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<sockaddr_in>) -> Option<&Value>
	{
		self.permitted_protocol_version_4_subnets.longest_match(&remote_peer_address.sin_addr)
	}
}

impl<Value> AccessControl<sockaddr_in6, Value> for RemotePeerAddressBasedAccessControl<Value>
{
	#[inline(always)]
	fn is_remote_peer_allowed(&self, remote_peer_address: sockaddr_in6, _streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<sockaddr_in6>) -> Option<&Value>
	{
		self.permitted_protocol_version_6_subnets.longest_match(&remote_peer_address.sin6_addr)
	}
}

impl<Value> AccessControl<sockaddr_un, Value> for RemotePeerAddressBasedAccessControl<Value>
{
	#[inline(always)]
	fn is_remote_peer_allowed(&self, _remote_peer_address: sockaddr_un, streaming_socket_file_descriptor: &StreamingSocketFileDescriptor<sockaddr_un>) -> Option<&Value>
	{
		let credentials = streaming_socket_file_descriptor.remote_peer_credentials();
		
		if let Some(value) = self.permitted_unix_domain_group_identifiers.get(&credentials.group_identifier)
		{
			return Some(value)
		}
		
		self.permitted_unix_domain_user_identifiers.get(&credentials.user_identifier)
	}
}
