// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See <https://www.iana.org/assignments/dane-parameters/dane-parameters.xhtml>
pub enum MatchingType<OOPB: OwnedOrParsedBytes<TypeEquality=TE>, SHA2_256: OwnedOrParsed<Sha2_256, TypeEquality=TE>, SHA2_512: OwnedOrParsed<Sha2_512, TypeEquality=TE>, TE: OwnedOrParsedTypeEquality>
{
	/// 'Full'.
	///
	/// No hash used; an exact match is required.
	///
	/// Defined by RFC 6698.
	NoHashUsed(OOPB),

	/// 'SHA2-256'.
	///
	/// 256 bit hash by SHA2; an exact match of SHA2-256 hash digests is required.
	///
	/// Defined by RFC 6234.
	Sha2_256(SHA2_256),

	/// 'SHA2-512'.
	///
	/// 512 bit hash by SHA2; an exact match of SHA2-512 hash digests is required.
	///
	/// Defined by RFC 6234.
	Sha2_512(SHA2_512),
}
