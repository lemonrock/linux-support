// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An Elliptic Curve Digital Signature Algorithm (ECDSA) public key in the format stated in RFC 6605 and RFC 8005.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EcDsaPublicKey<OOPB: OwnedOrParsedBytes>
{
	/// `Q` is a simple bit string that represents the uncompressed form of a curve point, `(x, y)`.
	///
	/// See FIPS 186-3 for a more detailed explanation of the value `Q`.
	///
	/// This value may be for either the curve `P-256` or the curve `P-384`; the RFCs do not explicitly state how to differentiate.
	///
	/// Conversion of the integers to bit strings is described in Section C.2 of FIPS 186-3.
	///
	/// * If 64 bytes long, then this is for a `P-256` curve and each co-ordinate (`x` or `y`) is an integer encoded as 32 octets;
	/// * If 96 bytes long, then this is for a `P-384` curve and each co-ordinate (`x` or `y`) is an integer encoded as 48 octets.
	///
	/// These lengths are currently validated.
	pub Q: OOPB,
}
