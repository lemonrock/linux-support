// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See <https://www.iana.org/assignments/dns-sshfp-rr-parameters/dns-sshfp-rr-parameters.xhtml>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SshPublicKeyAlgorithm
{
	/// RSA.
	///
	/// Defined by RFC 4255.
	RSA = 1,

	/// DSA.
	///
	/// Defined by RFC 4255.
	DSA = 2,

	/// ECDSA.
	///
	/// Defined by RFC 6594.
	ECDSA = 3,

	/// Ed25519.
	///
	/// Defined by RFC 7479.
	Ed25519 = 4,
}
