// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Certificate type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CertificateType<'a>
{
	/// `PKIX`.
	X509ASPerPkixCertificate(&'a [u8]),

	/// `SPKI`.
	///
	/// SPKI certificate.
	SpkiCertificate(&'a [u8]),

	/// `PGP`.
	///
	/// OpenPGP packet.
	OpenPgpPacket(&'a [u8]),

	/// `IPKIX`.
	///
	/// `I` stands for Indirect `PKIX`.
	///
	/// The URL of a X.509 data object.
	UrlOfAX509DataObject(&'a [u8]),

	/// `ISPKI`.
	///
	/// `I` stands for Indirect `SPKI`.
	///
	/// The URL of a SPKI certificate.
	UrlOfASpkiCertificate(&'a [u8]),

	/// `IPGP`.
	///
	/// `I` stands for Indirect `PGP`.
	///
	/// The fingerprint and URL of an OpenPGP packet.
	FingerprintAndUrlOfAnOpenPgpPacket(&'a [u8]),

	/// `ACPKIX`.
	///
	/// Attribute certificate.
	AttributeCertificate(&'a [u8]),

	/// `IACPKIX`.
	///
	/// `I` stands for Indirect `ACPKIX`.
	///
	/// The URL of an Attribute Certificate.
	UrlOfAnAttributeCertificate(&'a [u8]),
}
