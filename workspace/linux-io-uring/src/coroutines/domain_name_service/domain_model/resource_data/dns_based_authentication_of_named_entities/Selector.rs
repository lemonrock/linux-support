// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See <https://www.iana.org/assignments/dane-parameters/dane-parameters.xhtml>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Selector
{
	/// 'Cert'.
	///
	/// Full certificate: the Certificate binary structure as defined by RFC 5280.
	///
	/// Defined by RFC 6698.
	FullCertificate = 0,

	/// 'SPKI'.
	///
	/// Subject public key information: DER-encoded binary structure as defined by RFC 5280.
	///
	/// Defined by RFC 6698.
	SubjectPublicKeyInformation = 1,
}
