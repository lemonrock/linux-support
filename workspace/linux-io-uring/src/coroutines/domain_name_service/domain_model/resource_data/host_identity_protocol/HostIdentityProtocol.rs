// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Host Identity Protocol (`HIP`) resource record data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HostIdentityProtocol<'message>
{
	/// Host identitiy tag (HIT).
	pub host_identity_tag: &'message [u8],

	/// Public key.
	pub public_key: Option<PublicKey<'message>>,

	/// At least one rendezvous server is present.
	pub first_rendezvous_server_domain_name: ParsedName<'message>,

	/// May be empty.
	///
	/// Recipients *SHOULD* parse this to make sure the names are valid.
	pub remaining_rendezvous_server_domain_names: &'message [u8],
}
