// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HandleRecordTypeError<E: error::Error>
{
	/// A very obsolete data type was present.
	VeryObsoleteResourceRecordType(DataType),
	
	/// Miscellaneous.
	ValidateClassIsInternetAndGetTimeToLiveAndResourceData(ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError),
	
	/// Resource record visitor returned an error handling the record data type.
	ResourceRecordVisitor(DataType, E),
	
	/// Resource data for resource record type `A` has an incorrect length (value in tuple).
	AHasAnIncorrectLength(usize),
	
	/// Error parsing a name for a `NS` record type.
	NS(ParsedNameParserError),
	
	/// Error parsing a name for a `CNAME` record type.
	CNAME(ParsedNameParserError),
	
	/// `SOA`.
	SOA(SOAHandleRecordTypeError),
	
	/// Error parsing a name for a `PTR` record type.
	PTR(ParsedNameParserError),
	
	/// `HINFO`.
	HINFO(HINFOHandleRecordTypeError),
	
	/// `MX`.
	MX(MXHandleRecordTypeError),
	
	/// `TXT`.
	TXT(TXTHandleRecordTypeError),
	
	/// Resource data for resource record type `AAAA` has an incorrect length (value in tuple).
	AAAAHasAnIncorrectLength(usize),
	
	/// `LOC`.
	LOC(LOCHandleRecordTypeError),
	
	/// `SRV`.
	SRV(SRVHandleRecordTypeError),
	
	/// `NAPTR`.
	NAPTR(NAPTRHandleRecordTypeError),
	
	/// `KX`.
	KX(KXHandleRecordTypeError),
	
	/// `CERT`.
	CERT(CERTHandleRecordTypeError),
	
	/// `DNAME`.
	DNAME(DNAMEHandleRecordTypeError),
	
	/// `DS`.
	DS(DelegationSignerHandleRecordTypeError),
	
	/// `SSHFP`.
	SSHFP(SSHFPHandleRecordTypeError),
	
	/// `IPSECKEY`.
	IPSECKEY(IPSECKEYHandleRecordTypeError),
	
	/// `NSEC`.
	NSEC(NSECHandleRecordTypeError),
	
	/// `RRSIG`.
	RRSIG(RRSIGHandleRecordTypeError),
	
	/// `DNSKEY`.
	DNSKEY(DnsKeyHandleRecordTypeError),
	
	/// `DHCID`.
	DHCID(DHCIDHandleRecordTypeError),
	
	/// `NSEC3`.
	NSEC3(NSEC3HandleRecordTypeError),
	
	/// `NSEC3PARAM`.
	NSEC3PARAM(NSEC3PARAMHandleRecordTypeError),
	
	/// `TLSA`.
	TLSA(X509CertificateHandleRecordTypeError),
	
	/// `SMIMEA`.
	SMIMEA(X509CertificateHandleRecordTypeError),
	
	/// `HIP`.
	HIP(HIPHandleRecordTypeError),
	
	/// `CDS`.
	CDS(DelegationSignerHandleRecordTypeError),
	
	/// `CDNSKEY`.
	CDNSKEY(DnsKeyHandleRecordTypeError),
}

impl<E: error::Error> Display for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: error::Error> error::Error for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::HandleRecordTypeError::*;
		
		match self
		{
			&ValidateClassIsInternetAndGetTimeToLiveAndResourceData(ref error) => Some(error),
			
			&ResourceRecordVisitor(_data_type, ref error) => Some(error),
			
			&NS(ref error) => Some(error),
			
			&CNAME(ref error) => Some(error),
			
			&SOA(ref error) => Some(error),
			
			&PTR(ref error) => Some(error),
			
			&HINFO(ref error) => Some(error),
			
			&MX(ref error) => Some(error),
			
			&LOC(ref error) => Some(error),
			
			&SRV(ref error) => Some(error),
			
			&NAPTR(ref error) => Some(error),
			
			&KX(ref error) => Some(error),
			
			&CERT(ref error) => Some(error),
			
			&DNAME(ref error) => Some(error),
			
			&DS(ref error) => Some(error),
			
			&SSHFP(ref error) => Some(error),
			
			&IPSECKEY(ref error) => Some(error),
			
			&NSEC(ref error) => Some(error),
			
			&RRSIG(ref error) => Some(error),
			
			&DNSKEY(ref error) => Some(error),
			
			&DHCID(ref error) => Some(error),
			
			&NSEC3(ref error) => Some(error),
			
			&NSEC3PARAM(ref error) => Some(error),
			
			&TLSA(ref error) => Some(error),
			
			&SMIMEA(ref error) => Some(error),
			
			&HIP(ref error) => Some(error),
			
			&CDS(ref error) => Some(error),
			
			&CDNSKEY(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl<E: error::Error> From<ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError) -> Self
	{
		HandleRecordTypeError::ValidateClassIsInternetAndGetTimeToLiveAndResourceData(value)
	}
}

impl<E: error::Error> From<SOAHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: SOAHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::SOA(value)
	}
}

impl<E: error::Error> From<HINFOHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: HINFOHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::HINFO(value)
	}
}

impl<E: error::Error> From<MXHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: MXHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::MX(value)
	}
}

impl<E: error::Error> From<TXTHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: TXTHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::TXT(value)
	}
}

impl<E: error::Error> From<LOCHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: LOCHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::LOC(value)
	}
}

impl<E: error::Error> From<SRVHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: SRVHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::SRV(value)
	}
}

impl<E: error::Error> From<NAPTRHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: NAPTRHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::NAPTR(value)
	}
}

impl<E: error::Error> From<KXHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: KXHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::KX(value)
	}
}

impl<E: error::Error> From<CERTHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: CERTHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::CERT(value)
	}
}

impl<E: error::Error> From<DNAMEHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: DNAMEHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::DNAME(value)
	}
}

impl<E: error::Error> From<SSHFPHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: SSHFPHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::SSHFP(value)
	}
}

impl<E: error::Error> From<IPSECKEYHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: IPSECKEYHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::IPSECKEY(value)
	}
}

impl<E: error::Error> From<NSECHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: NSECHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::NSEC(value)
	}
}

impl<E: error::Error> From<RRSIGHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: RRSIGHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::RRSIG(value)
	}
}

impl<E: error::Error> From<DHCIDHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: DHCIDHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::DHCID(value)
	}
}

impl<E: error::Error> From<NSEC3HandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: NSEC3HandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::NSEC3(value)
	}
}

impl<E: error::Error> From<NSEC3PARAMHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: NSEC3PARAMHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::NSEC3PARAM(value)
	}
}

impl<E: error::Error> From<HIPHandleRecordTypeError> for HandleRecordTypeError<E>
{
	#[inline(always)]
	fn from(value: HIPHandleRecordTypeError) -> Self
	{
		HandleRecordTypeError::HIP(value)
	}
}