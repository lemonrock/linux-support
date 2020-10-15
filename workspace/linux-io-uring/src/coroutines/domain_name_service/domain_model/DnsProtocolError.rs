// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DNS protocol error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DnsProtocolError
{
	/// A message is shorter than the minimum.
	MessageIsTooShort,

	/// A message, once parsed, had bytes remaining in the TCP buffer.
	MessageHadUnparsedBytesAtEnd,

	/// A nameserver (`NS`) record in the authority section is not for the final name in a canonical name chain.
	NameServerRecordInAuthoritySectionIsNotForFinalNameInCanonicalNameChain,

	/// A start-of-authority (`SOA`) record in the authority section is not for the final name in a canonical name chain.
	StartOfAuthorityRecordInAuthoritySectionIsNotForFinalNameInCanonicalNameChain,

	/// A response was for an unknown request.
	ResponseWasForAnUnknownRequest(MessageIdentifier),

	/// A resource record was a duplicate (the same name, data type and resource data).
	DuplicateResourceRecord(DataType),

	/// Response was a query.
	ResponseWasAQuery,

	/// Invalid response opcode.
	InvalidResponseOpcode(u8),

	/// Response `QNAME` did not match.
	ResponseWasForADifferentName,

	/// Response `QTYPE` did not match.
	ResponseWasForADifferentDataType,

	/// Unassigned response opcode.
	UnassignedResponseOpcode(u8),

	/// Response used reserved header bits (`Z`).
	ResponseUsedReservedHeaderBits,

	/// Response is truncated (`TC`).
	ResponseIsTruncated,

	/// Response failed to copy the recursion desired (`RD`) bit.
	ResponseFailedToCopyRecursionDesiredBit,

	/// Response failed to copy the checking disabled (`CD`) bit.
	ResponseFailedToCopyCheckingDisabledBit,

	/// Response does not support Extended DNS.
	ResponseDoesNotSupportExtendedDns,

	/// Response did not contain an Extended DNS `OPT` meta resource record.
	ResponseDidNotContainAnExtendedDnsOptMetaResourceRecord,

	/// Response did not have the EDNS(0) DNSSEC OK (`DO`) bit set.
	ResponseIgnoredDnsSec,

	/// Response was authoritative (`AA` bit is set) but also has the authenticated data (`AD`) bit set; this is not possible, as an authoritative name server can not authenticate its own signatures!
	ResponseWasAuthoritativeButHasTheAuthoritativeDataBitSet,

	/// Response was authoritative (`AA` bit is set), the error code (`RCODE`) was `NXDOMAIN` but the answer section contained one or more answers (excluding `CNAME` and `DNAME` resource records).
	ResponseWasAuthoritativeWithNoSuchDomainErrorCodeButContainsAnAnswer,

	/// We produced a bad query; we didn't.
	MessageResponseCodeWasFormatError,

	/// This is NOT returned for data that failed validation when using DNSSEC.
	MessageResponseCodeWasServerFailure,

	/// This should not occur.
	MessageResponseCodeWasNonExistentDomainForANonAuthoritativeServer,

	/// Rare; indicates a server does not support a particular DNS OpCode.
	///
	/// Since every server should support the `Query` OpCde, this is pretty fatal.
	///
	/// Can occur also when using a server that doesn't support DNSSEC.
	MessageResponseCodeWasNotImplemented,

	/// Permission denied, effectively.
	MessageResponseCodeWasRefused,

	/// Message response code should not be dynamic DNS associated.
	MessageResponseCodeShouldNotBeDynamicDnsAssociated(u8),

	/// Message response code should not be DNS stateful operation type not implemented.
	MessageResponseCodeShouldNotBeDnsStatefulOperationsTypeNotImplemented,

	/// Message response code unassigned.
	MessageResponseCodeUnassigned(u8),

	/// Response does not contain exactly one question.
	ResponseDoesNotContainExactlyOneQuestion(u16),

	/// Too many resource records in the answer section for the size of the message.
	ResourceRecordsOverflowAnswerSection,

	/// Too many resource records in the authority section for the size of the message.
	ResourceRecordsOverflowAuthoritySection,

	/// Too many resource records in the additional section for the size of the message.
	ResourceRecordsOverflowAdditionalSection,

	/// DNS `QCLASS` is reserved (including for private use), unassigned or obsolete (ie Chaos or Hesiod).
	///
	/// Tuple contains value.
	///
	/// See [IANA](https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-2) and RFC 6895 for further details.
	ClassIsReservedUnassignedOrObsolete([u8; 2]),

	/// A text record string length was longer than that permitted by the resource data (`RDATA`) length (`RDLEN`).
	TextRecordStringLengthIncorrect,

	/// A resource record is shorter than the minimum size.
	ResourceRecordIsShorterThanMinimumSize,

	/// A resource record is shorter than the minimum size (after parsing the Name).
	ResourceRecordIsShorterThanMinimumSizeAfterParsingName,

	/// Resource data length overflows the space available.
	ResourceDataLengthOverflows,

	/// A record type was present in the answer section which should not have been (eg it was not queried for and is not `CNAME` or `DNAME`).
	ResourceRecordTypeIsNotValidInAnswerSection(DataType),

	/// A record type was present in the authority section which should not have been (only `SOA` records are allowed).
	ResourceRecordTypeIsNotValidInAuthoritySection(DataType),

	/// More than one `CNAME` record exists in an answer section.
	MoreThanOneCNAMERecordIsNotValidInAnswerSection,

	/// More than one `DNAME` record exists in an answer section.
	MoreThanOneDNAMERecordIsNotValidInAnswerSection,

	/// More than one `SOA` resource records.
	MoreThanOneStatementOfAuthorityResourceRecord,

	/// A `SOA` record type was present a section it should not have been in.
	StartOfAuthorityResourceRecordTypeIsNotPermittedInThisSection,

	/// An `OPT` record type was present a section it should not have been in.
	ExtendedDnsOptResourceRecordTypeIsNotPermittedInThisSection,

	/// A very obsolete data type was present.
	VeryObsoleteResourceRecordType(DataType),

	/// An unknown query or meta type was present; contains upper 8 bits and lower 8 bits.
	UnknownQueryTypeOrMetaType(u8, u8),

	/// A reserved record type was present; contains upper 8 bits and lower 8 bits.
	ReservedRecordType(u8, u8),

	/// Query type (`QTYPE`) `IXFR` is in a resource record.
	QueryTypeIXFRShouldNotOccurOutsideOfAQuestionSectionEntry,

	/// Query type (`QTYPE`) `AXFR` is in a resource record.
	QueryTypeAXFRShouldNotOccurOutsideOfAQuestionSectionEntry,

	/// Query type (`QTYPE`) `MAILB` is in a resource record.
	QueryTypeMAILBShouldNotOccurOutsideOfAQuestionSectionEntry,

	/// Query type (`QTYPE`) `MAILA` is in a resource record.
	QueryTypeMAILAShouldNotOccurOutsideOfAQuestionSectionEntry,

	/// Query type (`QTYPE`) `*` is in a resource record.
	QueryTypeAsteriskShouldNotOccurOutsideOfAQuestionSectionEntry,

	/// Resource data for resource record type `A` or `AAAA` has an incorrect length (value in tuple).
	ResourceDataForTypeAOrAAAAHasAnIncorrectLength(usize),

	/// Resource data for resource record type `LOC` has an incorrect length (value in tuple).
	ResourceDataForTypeLOCHasAnIncorrectLength(usize),

	/// Resource data for resource record type `LOC` has an incorrect version (value in tuple).
	ResourceDataForTypeLOCHasAnIncorrectVersion(u8),

	/// Resource data for resource record type `TLSA` or `SMIMEA` has an incorrect length (value in tuple).
	ResourceDataForTypeTLSAOrSMIMEAHasAnIncorrectLength(usize),

	/// Resource data for resource record type `TLSA` or `SMIMEA` has an incorrect digest length (value in tuple).
	ResourceDataForTypeTLSAOrSMIMEAHasADigestLengthThatIsIncorrectForTheMatchingType(usize),

	/// Resource data for resource record type `SRV` has an incorrect length (value in tuple).
	ResourceDataForTypeSRVHasAnIncorrectLength(usize),

	/// Resource data for resource record type `NAPTR` has an incorrect length (value in tuple).
	ResourceDataForTypeNAPTRHasAnIncorrectLength(usize),

	/// Resource data for resource record type `NAPTR` is missing the flags field.
	ResourceDataForTypeNAPTRIsMissingFlags,

	/// Resource data for resource record type `NAPTR` is missing the services field.
	ResourceDataForTypeNAPTRIsMissingServices,

	/// Resource data for resource record type `NAPTR` is missing the regular expression field.
	ResourceDataForTypeNAPTRIsMissingRegularExpression,

	/// Resource data for resource record type `NAPTR` has data left over.
	ResourceDataForTypeNAPTRHasDataLeftOver,

	/// Resource data for resource record type `IPSECKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeIPSECKEYHasTooShortALength(usize),

	/// Resource data for resource record type `IPSECKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeIPSECKEYHasTooShortALengthForAnEmptyDomainNameGateway(usize),

	/// Resource data for resource record type `IPSECKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeIPSECKEYHasTooShortALengthForAnInternetProtocolVersion4Gateway(usize),

	/// Resource data for resource record type `IPSECKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeIPSECKEYHasTooShortALengthForAnInternetProtocolVersion6Gateway(usize),

	/// Resource data for resource record type `IPSECKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeIPSECKEYHasTooShortALengthForDomainNameGateway(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for no public key (value in tuple).
	ResourceDataForTypeIPSECKEYOrHIPHasWrongLengthForNoPublicKey(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a DSA public key (value in tuple).
	ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForADSAPublicKey(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a DSA public key (value in tuple).
	ResourceDataForTypeIPSECKEYOrHIPHasWrongLengthForADSAPublicKeyOnceTIsKnown(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key (value in tuple).
	ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForRSAPublicKey(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key (value in tuple).
	ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForRSAPublicKeyForAThreeByteExponentLength(usize),

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key exponent.
	ResourceDataForTypeIPSECKEYOrHIPHasAZeroExponentForARSAPublicKey,

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key modulus.
	ResourceDataForTypeIPSECKEYOrHIPHasAZeroModulusForARSAPublicKey,

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key exponent.
	ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForARSAPublicKeyForExponentLength,

	/// Resource data for resource record type `IPSECKEY` or `HIP` has an unusual length for a ECDSA public key (ie it does not seem to be for `P-256` or `P-384`).
	ResourceDataForTypeIPSECKEYOrHIPHasAUnusualLengthForAnECDSAPublicKey(usize),

	/// Resource data for resource record type `NAPTR` has both a regular expression and a domain name.
	ResourceDataForTypeNAPTRHasBothARegularExpressionAndADomainName,

	/// Resource data for resource record type `DS` or `CDS` has an incorrect length (value in tuple).
	ResourceDataForTypeDSOrCDSHasAnIncorrectLength(usize),

	/// Resource data for resource record type `DHCID` has an incorrect length (value in tuple).
	ResourceDataForTypeDHCIDHasAnIncorrectLength(usize),

	/// Resource data for resource record type `DHCID` has reserved identifier type code.
	ResourceDataForTypeDHCIDHasAReservedIdentifierTypeCode,

	/// Resource data for resource record type `DHCID` has reserved digest type code.
	ResourceDataForTypeDHCIDHasAReservedDigestTypeCode,

	/// Resource data for resource record type `DHCID` has an incorrect digest length (value in tuple).
	ResourceDataForTypeDHCIDHasADigestLengthThatIsIncorrectForTheMatchingType(usize),

	/// Resource data for resource record type `SSHFP` has an incorrect length (value in tuple).
	ResourceDataForTypeSSHFPHasAnIncorrectLength(usize),

	/// Resource data for resource record type `SSHFP` has a reserved public key algorithm.
	ResourceDataForTypeSSHFPHasAReservedPublicKeyAlgorithm,

	/// Resource data for resource record type `SSHFP` has a reserved digest algorithm.
	ResourceDataForTypeSSHFPHasAReservedDigestAlgorithm,

	/// Resource data for resource record type `SSHFP` has an incorrect digest length (value in tuple).
	ResourceDataForTypeSSHFPAHasADigestLengthThatIsIncorrectForTheMatchingType(usize),

	/// Resource data for resource record type `NID` has an incorrect length (value in tuple).
	ResourceDataForTypeNIDHasAnIncorrectLength(usize),

	/// Resource data for resource record type `L32` has an incorrect length (value in tuple).
	ResourceDataForTypeL32HasAnIncorrectLength(usize),

	/// Resource data for resource record type `L64` has an incorrect length (value in tuple).
	ResourceDataForTypeL64HasAnIncorrectLength(usize),

	/// Resource data for resource record type `LP` has too short a length (value in tuple).
	ResourceDataForTypeLPHasTooShortALength(usize),

	/// Resource data for resource record type `LP` has data left over after parsing the domain name.
	ResourceDataForTypeLPHasDataLeftOver,

	/// Resource data for resource record type `EUI48` has an incorrect length (value in tuple).
	ResourceDataForTypeEUI48HasAnIncorrectLength(usize),

	/// Resource data for resource record type `EUI64` has an incorrect length (value in tuple).
	ResourceDataForTypeEUI64HasAnIncorrectLength(usize),

	/// Resource data for resource record type `URI` has an incorrect length (value in tuple).
	ResourceDataForTypeURIHasAnIncorrectLength(usize),

	/// Resource data for resource record type `CAA` has an incorrect length (value in tuple).
	ResourceDataForTypeCAAHasAnIncorrectLength(usize),

	/// Resource data for resource record type `CAA` has a zero tag length.
	ResourceDataForTypeCAAHasAZeroTagLength,

	/// Resource data for resource record type `CERT` has too short a length (value in tuple).
	ResourceDataForTypeCERTHasTooShortALength(usize),

	/// Resource data for resource record type `CERT` uses a reserved certificate type value (value in tuple).
	ResourceDataForTypeCERTUsesAReservedCertificateTypeValue(u16),

	/// Resource data for resource record type `CERT` uses an experimental certificate type value (value in tuple).
	ResourceDataForTypeCERTUsesAnExperimentalCertificateTypeValue(u16),

	/// Resource data for resource record type `DNSKEY` or `CDNSKEY` has an incorrect length (value in tuple).
	ResourceDataForTypeDNSKEYOrCDNSKEYHasAnIncorrectLength(usize),

	/// Resource data for resource record type `CSYNC` has an incorrect length (value in tuple).
	ResourceDataForTypeCSYNCHasAnIncorrectLength(usize),

	/// Resource data for resource record type `NSEC` has an incorrect length (value in tuple).
	ResourceDataForTypeNSECHasAnIncorrectLength(usize),

	/// Resource data for resource record type `CSYNC`, `NSEC` or `NSEC3` has an incorrect length (value in tuple).
	ResourceDataForTypeCSYNCOrNSECOrNSEC3HasAnOverflowingBlockLength(usize),

	/// Resource data for resource record type `CSYNC`, `NSEC` or `NSEC3` has a repeated or decreasing window number.
	ResourceDataForTypeCSYNCOrNSECOrNSEC3HasARepeatedOrDecreasingWindowNumber,

	/// Resource data for resource record type `CSYNC`, `NSEC` or `NSEC3` has a zero bitmap length (value in tuple).
	ResourceDataForTypeCSYNCOrNSECOrNSEC3HasAZeroBitmapLength,

	/// Resource data for resource record type `CSYNC`, `NSEC` or `NSEC3` has an incorrect bitmap length (value in tuple).
	ResourceDataForTypeCSYNCOrNSECOrNSEC3HasAnIncorrectBitmapLength(usize),

	/// Resource data for resource record type `CSYNC`, `NSEC` or `NSEC3` has an incorrect bitmap length (value in tuple).
	ResourceDataForTypeCSYNCOrNSECOrNSEC3HasAnOverflowingBitmapLength(usize),

	/// Resource data for resource record type `NSEC3` has an incorrect length (value in tuple).
	ResourceDataForTypeNSEC3HasAnIncorrectLength(usize),

	/// Resource data for resource record type `NSEC3PARAM` has an incorrect length (value in tuple).
	ResourceDataForTypeNSEC3PARAMHasAnIncorrectLength(usize),

	/// Resource data for resource record type `NSEC3` has a reserved hash algorithm.
	ResourceDataForTypeNSEC3HasAReservedHashAlgorithm,

	/// Resource data for resource record type `NSEC3PARAM` has a reserved hash algorithm.
	ResourceDataForTypeNSEC3PARAMHasAReservedHashAlgorithm,

	/// Resource data for resource record type `NSEC3` has an incorrect hash length for a SHA-1 hash.
	ResourceDataForTypeNSEC3HasAnIncorrectHashLengthForASha1Hash(usize),

	/// Resource data for resource record type `NSEC3` has an overflowing salt length.
	ResourceDataForTypeNSEC3HasAnOverflowingSaltLength(usize),

	/// Resource data for resource record type `NSEC3PARAM` has an overflowing salt length.
	ResourceDataForTypeNSEC3PARAMHasAnOverflowingSaltLength(usize),

	/// Resource data for resource record type `NSEC3` has an overflowing hash length.
	ResourceDataForTypeNSEC3HasAnOverflowingHashLength(usize),

	/// Resource data for resource record type `RRSIG` has an incorrect length (value in tuple).
	ResourceDataForTypeRRSIGHasAnIncorrectLength(usize),

	/// Resource data for resource record type `RRSIG` has more than 126 labels (including root, only 127 labels are allowed and root is not allowed to be counted in this instance).
	ResourceDataForTypeRRSIGHasMoreThan126Labels(u8),

	/// Resource data for resource record type `HIP` has an incorrect length (value in tuple).
	ResourceDataForTypeHIPHasAnIncorrectLength(usize),

	/// Resource data for resource record type `HINFO` has too short a length (value in tuple).
	ResourceDataForTypeHINFOHasTooShortALength(usize),

	/// Resource data for resource record type `HINFO` has too short a length after checking length of CPU field (value in tuple).
	ResourceDataForTypeHINFOWouldHaveCpuDataOverflow(usize),

	/// Resource data for resource record type `HINFO` has too short a length after checking length of OS field (value in tuple).
	ResourceDataForTypeHINFOWouldHaveOsDataOverflow(usize),

	/// After parsing resource data in a record of type `HINFO`, there is unattributed data remaining.
	ResourceDataForTypeHINFOWouldHaveUnusuedDataRemaining,

	/// Resource data for resource record type `MX` has too short a length (value in tuple).
	ResourceDataForTypeMXHasTooShortALength(usize),

	/// Resource data for resource record type `KX` has too short a length (value in tuple).
	ResourceDataForTypeKXHasTooShortALength(usize),

	/// Resource data for resource record type `DNAME` has too short a length (value in tuple).
	ResourceDataForTypeDNAMEHasTooShortALength(usize),

	/// Resource data for resource record type `KX` has data remaining after the key exchange server name.
	ResourceDataForTypeKXDataRemainingAfterKeyExchangeServerName,

	/// Resource data for resource record type `DNAME` has data remaining after the key exchange server name.
	ResourceDataForTypeDNAMEDataRemainingAfterDName,

	/// Resource data for resource record type `TXT` has no text strings (and thus has a length of zero).
	ResourceRecordForTypeTXTHasNoCharacterStrings,

	/// After parsing resource data in a record of type `TXT`, there is unattributed data remaining.
	ResourceDataForTypeTXTWouldHaveUnusuedDataRemaining,

	/// The security algorithm DS-Delete should not be used for this resource record.
	SecurityAlgorithmShouldNotBeUsedForThisResourceRecordType(u8),

	/// The security alogrithm type is reserved (number in tuple).
	SecurityAlgorithmTypeIsReservedByRfc6725(u8),

	/// A reserved security algorithm type (number in tuple).
	SecurityAlgorithmTypeIsReservedByRfc6014(u8),

	/// Reserved.
	SecurityAlgorithmTypeIsReservedByRfc4034,

	/// Reserved.
	DigestAlgorithmTypeIsReservedByRfc3658,

	/// A `DS` or `CDS` resource record has digest data that has an incorrect length for the digest type.
	ResourceDataForTypeDSOrCDSHasADigestLengthThatIsIncorrectForTheDigestType(usize),

	/// Resource data for resource record type `SOA` has an invalid length after parsing `MNAME` and `RNAME`.
	StartOfAuthorityIsIncorrectSizeAfterParsingMNAMEAndRNAME,

	/// A resource record of the psuedo-type `OPT` is present other than in the additional record section.
	ExtendedDnsOptRecordOutsideOfAdditionalDataSection,

	/// More than one resource record of the psuedo-type `OPT` is present in the additional record section.
	MoreThanOneExtendedDnsOptResourceRecord,

	/// The OPT extended error code bits are non-zero.
	ExtendedDnsOptUpper8BitsOfErrorNonZero(u8),

	/// A resource record of the psuedo-type `OPT` is present with a name other than ''.
	ExtendedDnsOptRecordNameTooLong,

	/// A resource record of the psuedo-type `OPT` is present with a name other than ''.
	ExtendedDnsOptRecordNameNotRoot,

	/// An unsupported EDNS version; unsupported version in tuple.
	UnsupportedExtendedDnsVersion(u8),

	/// EDNS(0) `Z` field not zero.
	ExtendedDnsZFieldNotZero,

	/// EDNS(0) Option field has a length less than 4.
	ExtendedDnsOptionTooShort,

	/// EDNS(0) Option code was in the reserved set (0, 65001-65534 and 65535); code is in tuple.
	///
	/// Code 4 is ignored as the draft it pertains sees occasionaly progress; it might come into being.
	ExtendedDnsOptionCodeWasReserved((u8, u8)),

	/// EDNS(0) Option length field indicates an option data field whose length would exceed that remaining in the resource data of the `OPT` resource record.
	ExtendedDnsOptionDataOverflows,

	/// Some EDNS(0) options must only be set in a request, or be the result of sending a request
	ExtendedDnsOptionMustOnlyBeSetInARequest((u8, u8)),

	/// The name was not long enough.
	///
	/// Typically this occurs when a name is shorter than the `RLEN/RDATA` space allocated for it in, say, a `CNAME` resource record.
	NameWasNotLongEnough,

	/// The name occupies no bytes at all.
	NameIsEmpty,

	/// The extended name labels are unused.
	ExtendedNameLabelsAreUnused,

	/// The unallocated name labels are unused.
	UnallocatedNameLabelsAreUnused,

	/// Compressed name labels are disallowed in this resource record.
	///
	/// See RFC 3597, Section 4 for some confusing rules.
	CompressedNameLabelsAreDisallowedInThisResourceRecord,

	/// When finishing a name combined from uncompressed labels and pointers, it creates a name longer than 127 labels.
	LabelPointerCreatesADnsNameLongerThan127Labels,

	/// When finishing a name combined from uncompressed labels and pointers, it creates a name longer than 255 bytes (including periods, including the trailing root period).
	LabelPointerCreatesADnsNameLongerThan255Bytes,

	/// The label pointer offset does not point to a previously parsed label.
	///
	/// Note that this includes pointers to pointers.
	///
	/// The tuple contains the offset.
	LabelPointerPointsToALabelThatWasNotPreviouslyParsed(usize),

	/// There is not a terminal root label in a Name.
	NoTerminalRootLabel,

	/// A label length would cause overflow (ie it is too long).
	LabelLengthOverflows,

	/// A label pointer overflows (ie there isn't another byte for bottom 8 bits).
	LabelPointerOverflows,

	/// A label pointer points to data that is after the start of the currently being parsed name.
	///
	/// This check prevents forward-pointers and circular pointers.
	LabelPointerPointsToDataAfterTheStartOfTheCurrentlyBeingParsedName,
}

impl Display for DnsProtocolError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DnsProtocolError
{
}
