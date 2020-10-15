// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Start of Authority (`SOA`) data.
#[derive(Debug, Clone)]
pub struct StartOfAuthority<'a>
{
	/// `MNAME`.
	///
	/// This is the FQDN of the primary name server.
	pub primary_name_server: WithCompressionParsedName<'a>,

	/// `RNAME`.
	///
	/// First label is the name `@`, eg `hostmaster.example.com.` is the email address `hostmaster@example.com`.
	pub responsible_person_email_address: WithCompressionParsedName<'a>,

	/// All other fields.
	///
	/// Not necesarily aligned, so may fault on 32-bit ARM.
	pub footer: &'a StartOfAuthorityFooter,
}
