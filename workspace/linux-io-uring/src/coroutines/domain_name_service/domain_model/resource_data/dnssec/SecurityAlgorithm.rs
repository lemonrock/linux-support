// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The security algorithm in use.
///
/// The list here is deliberately incomplete; only algorithms considered strong as of RFC 8624 are included irrespective of deployment status.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SecurityAlgorithm
{
	/// Only valid for `CDS` and `CDNSKEY` records.
	Delete,
	
	/// RSA-SHA-1.
	///
	/// Widely deployed but insecure.
	RivestShamirAdlemanSha1,
	
	/// RSA-SHA-1 NSEC3 SHA-1.
	///
	/// Widely deployed but insecure.
	RivestShamirAdlemanSha1Nsec3Sha1,

	/// RSA-SHA2-256.
	RivestShamirAdlemanSha256,

	/// RSA-SHA2-512.
	RivestShamirAdlemanSha512,

	/// ECDSA with curve P-256 and SHA-256.
	EllipticCurveDigitalSignatureAlgorithmWithCurveP256AndSha256,

	/// ECDSA with curve P-384 and SHA-384.
	EllipticCurveDigitalSignatureAlgorithmWithCurveP384AndSha384,

	/// Edwards curve 25519 as defined by Daniel J Bernstein.
	Ed25519,

	/// Edwards curve 448 as defined by Daniel J Bernstein.
	Ed448,
}

impl SecurityAlgorithm
{
	/// `Delete DS`.
	///
	/// Defined in RFCs 4034, 4398 and 8078.
	///
	/// Only valid for resource record types `CDS` and `CDNSKEY`.
	const DELETE: u8 = 0;

	/// `RSA/MD5`.
	///
	/// Defined in RFC 3110 and deprecated in RFC 4034; replaced by `RSASHA1`.
	///
	/// RFC 6944 states this is 'Must Not Implement'.
	const RSAMD5: u8 = 1;

	/// `Diffie-Hellman`.
	///
	/// Defined in RFC 2539.
	///
	/// Only a proposed standard.
	const DH: u8 = 2;

	/// `DSA/SHA1`.
	///
	/// Defined in RFC 2536 with reference to FIPS-186 and FIPS-180-1.
	///
	/// Only a proposed standard.
	const DSA: u8 = 3;

	/// `RSA/SHA-1`.
	///
	/// Defined in RFCs 3110 and 4034.
	///
	/// RFC 6944 states this is 'Must Implement'.
	/// However, given that [SHA-1 is now broken in practice](https://shattered.it/) (admittedly, not when combined with signing), this must be considered obsolete.
	const RSASHA1: u8 = 5;

	/// `DSA-NSEC3-SHA1`.
	///
	/// Defined in RFC 5155.
	///
	/// Only a proposed standard.
	const DSA_NSEC3_SHA1: u8 = 6;

	/// `RSASHA1-NSEC3-SHA1`.
	///
	/// Defined in RFC 5155.
	///
	/// Only a proposed standard.
	///
	/// RFC 6944 states this is 'Recommended to Implement'.
	const RSASHA1_NSEC3_SHA1: u8 = 7;

	/// `RSA/SHA-256`.
	///
	/// Defined in RFC 5702.
	///
	/// Only a proposed standard.
	///
	/// RFC 6944 states this is 'Recommended to Implement'.
	const RSASHA256: u8 = 8;

	/// `RSA/SHA-512`.
	///
	/// Defined in RFC 5702.
	///
	/// Only a proposed standard.
	///
	/// RFC 6944 states this is 'Recommended to Implement'.
	const RSASHA512: u8 = 10;

	/// `GOST R 34.10-2001`.
	///
	/// Defined in RFC 5933.
	///
	/// Supposed to be a required standard, but effectively obsolete.
	const ECC_GOST: u8 = 12;

	/// `ECDSA Curve P-256 with SHA-256`.
	///
	/// Defined in RFC 6605.
	///
	/// Standard.
	///
	/// RFC 6944 states this is 'Recommended to Implement'.
	const ECDSAP256SHA256: u8 = 13;

	/// `ECDSA Curve P-384 with SHA-384`.
	///
	/// Defined in RFC 6605.
	///
	/// Standard.
	///
	/// RFC 6944 states this is 'Recommended to Implement'.
	const ECDSAP384SHA384: u8 = 14;

	/// `Ed25519`.
	///
	/// Defined in RFC 8080.
	///
	/// Standard.
	///
	/// Edwards-curve Digital Signature Algorithm using SHA-512 and Curve 25519.
	///
	/// Created by Daniel J. Bernstein et al.
	const ED25519: u8 = 15;

	/// `Ed448`.
	///
	/// Defined in RFC 8080.
	///
	/// Standard.
	///
	/// Known as Edwards-curve Digital Signature Algorithm using `SHAKE256` and Curve 448.
	///
	/// Created by Daniel J. Bernstein et al.
	const ED448: u8 = 16;

	const INDIRECT: u8 = 252;

	const PRIVATEDNS: u8 = 253;

	const PRIVATEOID: u8 = 254;

	#[inline(always)]
	pub(crate) fn parse_security_algorithm(security_algorithm_type: u8, permit_delete: bool, permit_widely_deployed_but_insecure: bool, data_type: DataType) -> Result<Either<Self, SecurityAlgorithmRejectedBecauseReason>, SecurityAlgorithmHandleRecordTypeError>
	{
		use self::SecurityAlgorithm::*;
		use self::SecurityAlgorithmHandleRecordTypeError::*;
		use self::SecurityAlgorithmRejectedBecauseReason::*;

		match security_algorithm_type
		{
			Self::DELETE => if unlikely!(permit_delete)
			{
				Ok(Left(Delete))
			}
			else
			{
				Err(ShouldNotBeUsedForThisResourceRecordType(data_type, security_algorithm_type))
			}

			Self::RSAMD5 => Ok(Right(DeprecatedSecurityAlgorithm_RSA_MD5)),

			Self::DH => Ok(Right(EffectivelyObsoleteSecurityAlgorithm_Diffie_Hellman)),

			Self::DSA => Ok(Right(VulnerableSecurityAlgorithm_DSA)),

			4 => Err(TypeIsReservedByRfc6725(data_type, 4)),

			Self::RSASHA1 => if unlikely!(permit_widely_deployed_but_insecure)
			{
				Ok(Left(RivestShamirAdlemanSha1))
			}
			else
			{
				Ok(Right(WidelyDeployedButInsecureSecurityAlogrithm(security_algorithm_type)))
			},

			Self::DSA_NSEC3_SHA1 => Ok(Right(VulnerableSecurityAlgorithm_DSA)),

			Self::RSASHA1_NSEC3_SHA1 => if unlikely!(permit_widely_deployed_but_insecure)
			{
				Ok(Left(RivestShamirAdlemanSha1Nsec3Sha1))
			}
			else
			{
				Ok(Right(WidelyDeployedButInsecureSecurityAlogrithm(security_algorithm_type)))
			},

			Self::RSASHA256 => Ok(Left(RivestShamirAdlemanSha256)),

			9 => Err(TypeIsReservedByRfc6725(data_type, 9)),

			Self::RSASHA512 => Ok(Left(RivestShamirAdlemanSha512)),

			11 => Err(TypeIsReservedByRfc6725(data_type, 11)),

			Self::ECC_GOST => Ok(Right(EffectivelyObsoleteSecurityAlgorithm_GOST_R_34_10_2001)),

			Self::ECDSAP256SHA256 => Ok(Left(EllipticCurveDigitalSignatureAlgorithmWithCurveP256AndSha256)),

			Self::ECDSAP384SHA384 => Ok(Left(EllipticCurveDigitalSignatureAlgorithmWithCurveP384AndSha384)),

			Self::ED25519 => Ok(Left(Ed25519)),

			Self::ED448 => Ok(Left(Ed448)),

			17 ..= 122 => Ok(Right(Unassigned(security_algorithm_type))),

			123 ..= 251 => Err(TypeIsReservedByRfc6014(data_type, security_algorithm_type)),

			Self::INDIRECT => Ok(Right(ReservedByRfc4034ForIndirectKeys)),

			Self::PRIVATEDNS => Ok(Right(ReservedByRfc4034ForPrivateAlgorithms)),

			Self::PRIVATEOID => Ok(Right(ReservedByRfc4034ForPrivateAlgorithmOids)),

			255 => Err(TypeIsReservedByRfc4034(data_type)),
		}
	}
}
