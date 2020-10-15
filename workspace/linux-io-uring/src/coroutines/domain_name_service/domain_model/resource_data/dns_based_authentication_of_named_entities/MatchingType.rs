// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See <https://www.iana.org/assignments/dane-parameters/dane-parameters.xhtml>
pub enum MatchingType<'a>
{
	/// 'Full'.
	///
	/// No hash used; an exact match is required.
	///
	/// Defined by RFC 6698.
	NoHashUsed(&'a [u8]),

	/// 'SHA2-256'.
	///
	/// 256 bit hash by SHA2; an exact match of SHA2-256 hash digests is required.
	///
	/// Defined by RFC 6234.
	Sha2_256(&'a [u8; 256 / 8]),

	/// 'SHA2-512'.
	///
	/// 512 bit hash by SHA2; an exact match of SHA2-512 hash digests is required.
	///
	/// Defined by RFC 6234.
	Sha2_512(&'a [u8; 512 / 8]),
}
