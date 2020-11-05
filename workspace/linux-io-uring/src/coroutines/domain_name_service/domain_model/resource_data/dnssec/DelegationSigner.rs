// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A delegation signer.
pub struct DelegationSigner<SHA2_256: OwnedOrParsed<Sha2_256, TypeEquality=TE>, SHA2_384: OwnedOrParsed<Sha2_384, TypeEquality=TE>, TE: OwnedOrParsedTypeEquality>
{
	/// Key tag.
	pub key_tag: KeyTag,

	/// Security algorithm.
	pub security_algorithm: SecurityAlgorithm,

	/// Digest.
	pub digest: DnsSecDigest<SHA2_256, SHA2_384, TE>,
}
