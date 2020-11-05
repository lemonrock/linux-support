// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Certificate type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CertificateType<OOPB: OwnedOrParsedBytes>
{
	/// `PKIX`.
	X509ASPerPkixCertificate(OOPB),

	/// `SPKI`.
	///
	/// SPKI certificate.
	SpkiCertificate(OOPB),

	/// `PGP`.
	///
	/// OpenPGP packet.
	OpenPgpPacket(OOPB),

	/// `IPKIX`.
	///
	/// `I` stands for Indirect `PKIX`.
	///
	/// The URL of a X.509 data object.
	UrlOfAX509DataObject(OOPB),

	/// `ISPKI`.
	///
	/// `I` stands for Indirect `SPKI`.
	///
	/// The URL of a SPKI certificate.
	UrlOfASpkiCertificate(OOPB),

	/// `IPGP`.
	///
	/// `I` stands for Indirect `PGP`.
	///
	/// The fingerprint and URL of an OpenPGP packet.
	FingerprintAndUrlOfAnOpenPgpPacket(OOPB),

	/// `ACPKIX`.
	///
	/// Attribute certificate.
	AttributeCertificate(OOPB),

	/// `IACPKIX`.
	///
	/// `I` stands for Indirect `ACPKIX`.
	///
	/// The URL of an Attribute Certificate.
	UrlOfAnAttributeCertificate(OOPB),
}

impl<'message> Into<CertificateType<OwnedBytes>> for CertificateType<ParsedBytes<'message>>
{
	#[inline(always)]
	fn into(self) -> CertificateType<OwnedBytes>
	{
		use self::CertificateType::*;
		
		match self
		{
			X509ASPerPkixCertificate(parsed_arbitrary_bytes) => X509ASPerPkixCertificate(parsed_arbitrary_bytes.into()),
			
			SpkiCertificate(parsed_arbitrary_bytes) => SpkiCertificate(parsed_arbitrary_bytes.into()),
			
			OpenPgpPacket(parsed_arbitrary_bytes) => OpenPgpPacket(parsed_arbitrary_bytes.into()),
			
			UrlOfAX509DataObject(parsed_arbitrary_bytes) => UrlOfAX509DataObject(parsed_arbitrary_bytes.into()),
			
			UrlOfASpkiCertificate(parsed_arbitrary_bytes) => UrlOfASpkiCertificate(parsed_arbitrary_bytes.into()),
			
			FingerprintAndUrlOfAnOpenPgpPacket(parsed_arbitrary_bytes) => FingerprintAndUrlOfAnOpenPgpPacket(parsed_arbitrary_bytes.into()),
			
			AttributeCertificate(parsed_arbitrary_bytes) => AttributeCertificate(parsed_arbitrary_bytes.into()),
			
			UrlOfAnAttributeCertificate(parsed_arbitrary_bytes) => UrlOfAnAttributeCertificate(parsed_arbitrary_bytes.into()),
		}
	}
}
