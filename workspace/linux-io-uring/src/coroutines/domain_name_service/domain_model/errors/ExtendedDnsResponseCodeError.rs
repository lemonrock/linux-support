// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Extended DNS (`EDNS(0)`) response code error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ExtendedDnsResponseCodeError
{
	/// Format Error ('FormErr' or `FORMERR` in older RFCs).
	///
	/// The name server was unable to interpret the query.
	/// Can occur if the name server does not support any version of `EDNS`.
	/// See also `NotImplemented`.
	///
	/// Defined in RFC 1035.
	/// Restated in RFC 6895, Section 2.3 RCODE Assignment.
	Format,
	
	/// Server Failure ('ServFail' or `SERVFAIL` in older RFCs).
	///
	/// The name server was unable to process this query due to a problem with the name server.
	///
	/// Defined in RFC 1035.
	/// Restated in RFC 6895, Section 2.3 RCODE Assignment.
	///
	/// This is sometimes returned for data that failed validation when using DNSSEC, or, in violation of RFC 1035 (see <https://letsencrypt.org/docs/caa/#servfail> ) if an authoritative nameserver replies `NOTIMP` when generating an empty response.
	/// Hence we include the outcome of checking whether the server was authoritative or the data authenticated.
	ServerFailure(AuthoritativeOrAuthenticatedOrNeither),
	
	/// Not Implemented ('NotImp' or `NOTIMP` in older RFCs).
	///
	/// The name server does not support the requested kind of query.
	/// Can occur if the name server does not support any version of `EDNS`.
	/// See also `Format`.
	///
	/// Defined in RFC 1035.
	/// Restated in RFC 6895, Section 2.3 RCODE Assignment.
	///
	/// Rare; indicates a server does not support a particular DNS OpCode.
	///
	/// Since every server should support the `Query` OpCde, this is pretty fatal.
	///
	/// Can occur also when using a server that doesn't support DNSSEC, or, in violation of RFC 1035 (see <https://letsencrypt.org/docs/caa/#servfail> ) if an authoritative nameserver replies `NOTIMP` when generating an empty response.
	NotImplemented,
	
	/// Query Refused ('Refused' or `REFUSED` in older RFCs).
	///
	/// The name server refuses to perform the specified operation for policy reasons.
	/// For example, a name server may not wish to provide the information to the particular requester, or a name server may not wish to perform a particular operation (eg a zone transfer) for particular data.
	///
	/// Defined in RFC 1035.
	///
	/// Permission denied, effectively.
	///
	/// The name server refuses to perform the specified operation for policy reasons.
	/// For example, a name server may not wish to provide the information to the particular requester, or a name server may not wish to perform a particular operation (eg a zone transfer) for particular data.
	Refused,
	
	/// Name exists when it should not ('YXDomain' or `YXDOMAIN` in older RFCs).
	///
	/// Defined in RFC 2136 and RFC 6672.
	/// Restated in RFC 6895, Section 2.3 RCODE Assignment.
	///
	/// Defined originally in RFC 2136 as a Dynamic DNS error.
	/// RFC 6672, Section 2.2 Final Paragraph allows this code to occur if DNAME substitution would produce a FQDN longer than 255 bytes.
	NameExistsWhenItShouldNot,
	
	/// RR Set Exists when it should not ('YXRRSet' or `YXRRSET` in older RFCs).
	///
	/// RR standard for Resource Record.
	///
	/// Defined in RFC 2136.
	/// Restated in RFC 6895, Section 2.3 RCODE Assignment.
	///
	/// Defined originally in RFC 2136 as a Dynamic DNS error.
	ResourceRecordSetExistsWhenItShouldNot,
	
	/// RR Set that should exist does not ('NXRRSet' or `NXRRSET` in older RFCs).
	///
	/// RR standard for Resource Record.
	///
	/// Defined in RFC 2136.
	/// Restated in RFC 6895, Section 2.3 RCODE Assignment.
	///
	/// Defined originally in RFC 2136 as a Dynamic DNS error.
	ResourceRecordSetThatShouldExistDoesNot,
	
	/// Two different definitions are in use:-
	///
	/// * Server Not Authoritative for zone ('NotAuth' or `NOTAUTH` in older RFCs) in RFC 2136.
	/// * Not Authorized  ('NotAuth') in RFC 2845.
	///
	/// Clarified in RFC 6895, Section 2.3 RCODE Assignment:-
	///
	/// * If it is the `RCODE` of a DNS reponse without a `TSIG` record then it means 'Server Not Authoritative for zone'.
	/// * If it is the `RCODE` of a DNS reponse with a `TSIG` record then:-
	/// 	* If the `TSIG` record has its own error field of `0`, it means 'Server Not Authoritative for zone'.
	/// 	* If the `TSIG` record has its own error field which is not `0`, it means 'Not Authorized'.
	///
	/// In our implementation, we never check for the presence of a `TSIG` record or the value of its error code field.
	ServerNotAuthoritativeForZone,
	
	/// Name not contained in zone ('NotZone' or `NOTZONE` in older RFCs).
	///
	/// Defined in RFC 2136.
	/// Restated in RFC 6895, Section 2.3 RCODE Assignment.
	///
	/// Defined originally in RFC 2136 as a Dynamic DNS error.
	NameNotContainedInZone,
	
	/// DNS Stateful Operations TYPE (DSO-TYPE) not implemented ('DSOTYPENI').
	///
	/// Defined in RFC 8490.
	/// Unassigned in RFC 6895, Section 2.3 RCODE Assignment.
	DnsStatefulOperationsTypeNotImplemented,
	
	/// Message response code 12 is unassigned.
	Unassigned12,
	
	/// Message response code 13 is unassigned.
	Unassigned13,
	
	/// Message response code 14 is unassigned.
	Unassigned14,
	
	/// Message response code 15 is unassigned.
	Unassigned15,
	
	/// Bad `OPT` version ('BADVERS').
	///
	/// Can occur if the name server does not version 0 of `EDNS` (`EDNS(0)`).
	///
	/// Defined in RFC 6891.
	///
	/// Clarified in RFC 6895, Section 2.3 RCODE Assignment paragraph 2 that this is differs in meaning the error space between `TSIG` and `EDNS(0)`.
	/// In RFC 2845 for `TSIG` records this means `TSIG` Signature Failure ('BADSIG').
	BadVersion,

	/// Key not recongized ('BADKEY').
	///
	/// Defined in RFC 2845.
	/// Clarified in RFC 6895, Section 2.3 RCODE Assignment.
	KeyNotRecognized,

	/// Signature out of time window ('BADTIME').
	///
	/// Defined in RFC 2845.
	/// Clarified in RFC 6895, Section 2.3 RCODE Assignment.
	SignatureOutOfTimeWindow,

	/// Bad `TKEY` mode ('BADMODE').
	///
	/// Defined in RFC 2930.
	/// Clarified in RFC 6895, Section 2.3 RCODE Assignment.
	BadTkeyMode,

	/// Duplicate key name ('BADNAME').
	///
	/// Defined in RFC 2930.
	/// Clarified in RFC 6895, Section 2.3 RCODE Assignment.
	DuplicateKeyName,

	/// Algorithm not supported ('BADALG').
	///
	/// Defined in RFC 2930.
	/// Clarified in RFC 6895, Section 2.3 RCODE Assignment.
	AlgorithmNotSupported,

	/// Bad Truncation ('BADTRUNC').
	///
	/// Defined in RFC 4635.
	/// Clarified in RFC 6895, Section 2.3 RCODE Assignment.
	BadTruncation,

	/// Bad/missing Server Cookie.
	///
	/// Defined in RFC 7873.
	BadOrMissingServerCookie,
	
	/// Unassigned (see <https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml>).
	///
	/// Currently for values 24 to 3840 inclusive.
	Unassigned(u16),

	/// Reserved by RFC 6895, Section 2.3 RCODE Assignment for private use.
	///
	/// Values 3841 to 4095 inclusive.
	ReservedForPrivateUse(u16),
}

impl Display for ExtendedDnsResponseCodeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ExtendedDnsResponseCodeError
{
}

impl ExtendedDnsResponseCodeError
{
	#[inline(always)]
	pub(crate) fn parse_error_code(extended_response_code_upper_8_bits: u8, authoritative_or_authenticated_or_neither: AuthoritativeOrAuthenticatedOrNeither, rcode_lower_4_bits: RCodeLower4Bits) -> Result<AnswerExistence, Self>
	{
		use self::AnswerExistence::*;
		use self::ExtendedDnsResponseCodeError::*;
		
		match ((extended_response_code_upper_8_bits as u16) << 4) | rcode_lower_4_bits.into_u16()
		{
			// No Error ('NoError' or `NOERROR` in older RFCs).
			//
			// Defined in RFC 1035.
			// Restated in RFC 6895, Section 2.3 RCODE Assignment.
			0 => Ok(NoError(authoritative_or_authenticated_or_neither)),
			
			1 => Err(Format),
		
			2 => Err(ServerFailure(authoritative_or_authenticated_or_neither)),
			
			// Non-Existent Domain ('NXDomain' or `NXDOMAIN` in RFC 2038).
			//
			// Supposedly meaningful only for responses from an authoritative name server, this code signifies that the domain name referenced in the query does not exist.
			// However, in practice, we need to just accept this reponse.
			//
			// Defined in RFC 1035.
			// Restated in RFC 6895, Section 2.3 RCODE Assignment.
			3 => Ok(NoDomain(authoritative_or_authenticated_or_neither)),
		
			4 => Err(NotImplemented),
		
			5 => Err(Refused),
		
			// RFC 6672, Section 2.2 Final Paragraph allows this code to occur if DNAME substitution would produce a FQDN longer than 255 bytes.
			6 => Err(NameExistsWhenItShouldNot),
		
			7 => Err(ResourceRecordSetExistsWhenItShouldNot),
		
			8 => Err(ResourceRecordSetThatShouldExistDoesNot),
		
			9=> Err(ServerNotAuthoritativeForZone),
		
			10 => Err(NameNotContainedInZone),
		
			11 => Err(DnsStatefulOperationsTypeNotImplemented),
		
			12 => Err(Unassigned12),
		
			13 => Err(Unassigned13),
		
			14 => Err(Unassigned14),
		
			15 => Err(Unassigned15),
			
			16 => Err(BadVersion),
			
			17 => Err(KeyNotRecognized),
			
			18 => Err(SignatureOutOfTimeWindow),
			
			19 => Err(BadTkeyMode),
			
			20 => Err(DuplicateKeyName),
			
			21 => Err(AlgorithmNotSupported),
			
			22 => Err(BadTruncation),
			
			23 => Err(BadOrMissingServerCookie),
			
			error_code @ 24 ..= 3840 => Err(Unassigned(error_code)),
			
			error_code @ 3841 ..= 4095 => Err(ReservedForPrivateUse(error_code)),
			
			_ => unreachable_code(format_args!("")),
		}
	}
}
