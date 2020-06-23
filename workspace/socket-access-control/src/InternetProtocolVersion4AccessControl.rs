// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Remote peer address-based access control for Internet Protocol (IP) version 4.
///
/// Holds whitelists.
#[derive(Debug, Clone)]
pub struct InternetProtocolVersion4AccessControl<Value>(LongestPrefixMatchTable<in_addr, Value>);

impl<Value> AccessControl<sockaddr_in, Value> for InternetProtocolVersion4AccessControl<Value>
{
	#[inline(always)]
	fn is_remote_peer_allowed(&self, accepted_connection: &AcceptedConnection<sockaddr_in>) -> Option<&Arc<Value>>
	{
		self.0.longest_match(&accepted_connection.peer.address.sin_addr)
	}
}

impl<Value> InternetProtocolVersion4AccessControl<Value>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(permitted_internet_protocol_version_4_subnets: &BTreeMap<InternetProtocolAddressWithMask<in_addr>, Arc<Value>>) -> Self
	{
		Self(LongestPrefixMatchTable::new(permitted_internet_protocol_version_4_subnets))
	}
}
