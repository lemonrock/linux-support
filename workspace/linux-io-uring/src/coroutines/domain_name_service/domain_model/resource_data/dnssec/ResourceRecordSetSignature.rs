// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A resource record set signature (`RRSIG`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceRecordSetSignature<N: Name<TypeEquality=TE>, OOPB: OwnedOrParsedBytes<TypeEquality=TE>, TE: OwnedOrParsedTypeEquality>
{
	/// Type covered.
	///
	/// This is not validated to be a valid DataType; it could be a meta type or query type, or unassigned or reserved.
	pub type_covered: DataType,

	/// Security algorithm.
	pub security_algorithm: SecurityAlgorithm,

	/// Labels (validated to be 126 or less).
	pub labels: u8,

	/// Original time to live.
	pub original_time_to_live: TimeInSeconds,

	/// A key tag.
	pub key_tag: KeyTag,

	/// Signer's name.
	pub signers_name: N,

	/// Signature.
	pub signature: OOPB,

	/// Required for verifying a signature.
	pub rrsig_rdata_excluding_signature_field: OOPB,
}
