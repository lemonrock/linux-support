// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// DNS-based Authentication of Named Entities (DANE) record data.
pub struct DnsBasedAuthenticationOfNamedEntities<OOPB: OwnedOrParsedBytes<TypeEquality=TE>, SHA2_256: OwnedOrParsed<Sha2_256, TypeEquality=TE>, SHA2_512: OwnedOrParsed<Sha2_512, TypeEquality=TE>, TE: OwnedOrParsedTypeEquality>
{
	/// Certificate usage.
	pub certificate_usage: CertificateUsage,

	/// Selector.
	pub selector: Selector,

	/// Matching type.
	pub matching_type: MatchingType<OOPB, SHA2_256, SHA2_512, TE>,
}
