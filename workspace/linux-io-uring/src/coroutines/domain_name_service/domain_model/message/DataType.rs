// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Data type.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub struct DataType(pub(crate) BigEndianU16);

impl DataTypeOrMetaType for DataType
{
	#[inline(always)]
	fn into_big_endian_u16(self) -> BigEndianU16
	{
		self.0
	}
}

impl DataType
{
	/// Internet Protocol version 4 address.
	///
	/// Defined in RFC 1035.
	pub(crate) const A_higher: u8 = 0x00;
	pub(crate) const A_lower: u8 = 1;
	pub(crate) const A: Self = Self([Self::A_higher, Self::A_lower]);

	/// Name server.
	///
	/// Defined in RFC 1035.
	pub(crate) const NS_higher: u8 = 0x00;
	pub(crate) const NS_lower: u8 = 2;
	pub(crate) const NS: Self = Self([Self::NS_higher, Self::NS_lower]);

	/// Defined in RFC 883 and made obsolete in RFC 973.
	pub(crate) const MD_higher: u8 = 0x00;
	pub(crate) const MD_lower: u8 = 3;
	pub(crate) const MD: Self = Self([Self::MD_higher, Self::MD_lower]);

	/// Defined in RFC 883 and made obsolete in RFC 973.
	pub(crate) const MF_higher: u8 = 0x00;
	pub(crate) const MF_lower: u8 = 4;
	pub(crate) const MF: Self = Self([Self::MF_higher, Self::MF_lower]);

	/// Canonical name.
	///
	/// Defined in RFC 1035.
	pub(crate) const CNAME_higher: u8 = 0x00;
	pub(crate) const CNAME_lower: u8 = 5;
	pub(crate) const CNAME: Self = Self([Self::CNAME_higher, Self::CNAME_lower]);

	/// Marks the start of a zone of authority.
	///
	/// Defined in RFC 1035 and RFC 2308.
	pub(crate) const SOA_higher: u8 = 0x00;
	pub(crate) const SOA_lower: u8 = 6;
	pub(crate) const SOA: Self = Self([Self::SOA_higher, Self::SOA_lower]);

	/// Defined in RFC 883 and made effectively obsolete by RFC 2505.
	pub(crate) const MB_higher: u8 = 0x00;
	pub(crate) const MB_lower: u8 = 7;
	pub(crate) const MB: Self = Self([Self::MB_higher, Self::MB_lower]);

	/// Defined in RFC 883 and made effectively obsolete by RFC 2505.
	pub(crate) const MG_higher: u8 = 0x00;
	pub(crate) const MG_lower: u8 = 8;
	pub(crate) const MG: Self = Self([Self::MG_higher, Self::MG_lower]);

	/// Defined in RFC 883 and made effectively obsolete by RFC 2505.
	pub(crate) const MR_higher: u8 = 0x00;
	pub(crate) const MR_lower: u8 = 9;
	pub(crate) const MR: Self = Self([Self::MR_higher, Self::MR_lower]);

	/// Effectively made obsolete by RFC 1035.
	pub(crate) const NULL_higher: u8 = 0x00;
	pub(crate) const NULL_lower: u8 = 10;
	pub(crate) const NULL: Self = Self([Self::NULL_higher, Self::NULL_lower]);

	/// Defined in RFC 883 and RFC 1035 and made effectively obsolete by RFC 1123 and RFC 1127.
	pub(crate) const WKS_higher: u8 = 0x00;
	pub(crate) const WKS_lower: u8 = 11;
	pub(crate) const WKS: Self = Self([Self::WKS_higher, Self::WKS_lower]);

	/// Pointer.
	///
	/// Defined in RFC 1035.
	pub(crate) const PTR_higher: u8 = 0x00;
	pub(crate) const PTR_lower: u8 = 12;
	pub(crate) const PTR: Self = Self([Self::PTR_higher, Self::PTR_lower]);

	/// Host Information.
	///
	/// Defined in RFC 883, repurposed by RFC 8482 for replies to `*` (`ANY`) `QTYPE` queries.
	pub(crate) const HINFO_higher: u8 = 0x00;
	pub(crate) const HINFO_lower: u8 = 13;
	pub(crate) const HINFO: Self = Self([Self::HINFO_higher, Self::HINFO_lower]);

	/// Defined in RFC 883 and made effectively obsolete by RFC 2505.
	pub(crate) const MINFO_higher: u8 = 0x00;
	pub(crate) const MINFO_lower: u8 = 14;
	pub(crate) const MINFO: Self = Self([Self::MINFO_higher, Self::MINFO_lower]);

	/// Mail exchange.
	///
	/// Defined in RFC 1035.
	pub(crate) const MX_higher: u8 = 0x00;
	pub(crate) const MX_lower: u8 = 15;
	pub(crate) const MX: Self = Self([Self::MX_higher, Self::MX_lower]);

	/// Text string.
	///
	/// Defined in RFC 1035 and more fully specified in RFC 1464.
	pub(crate) const TXT_higher: u8 = 0x00;
	pub(crate) const TXT_lower: u8 = 16;
	pub(crate) const TXT: Self = Self([Self::TXT_higher, Self::TXT_lower]);

	/// Responsible Person.
	///
	/// Defined in RFC 1183; effectively obsolete.
	pub(crate) const RP_higher: u8 = 0x00;
	pub(crate) const RP_lower: u8 = 17;
	pub(crate) const RP: Self = Self([Self::RP_higher, Self::RP_lower]);

	/// Andrew File System (AFS) Data Base location
	///
	/// Defined in RFC 1183; made obsolete by RFC 5864.
	pub(crate) const AFSDB_higher: u8 = 0x00;
	pub(crate) const AFSDB_lower: u8 = 18;
	pub(crate) const AFSDB: Self = Self([Self::AFSDB_higher, Self::AFSDB_lower]);

	/// X.25 PSDN address.
	///
	/// Defined in RFC 1183; effectively obsolete.
	pub(crate) const X25_higher: u8 = 0x00;
	pub(crate) const X25_lower: u8 = 19;
	pub(crate) const X25: Self = Self([Self::X25_higher, Self::X25_lower]);

	/// ISDN address.
	///
	/// Defined in RFC 1183; effectively obsolete.
	pub(crate) const ISDN_higher: u8 = 0x00;
	pub(crate) const ISDN_lower: u8 = 20;
	pub(crate) const ISDN: Self = Self([Self::ISDN_higher, Self::ISDN_lower]);

	/// Route Through.
	///
	/// Defined in RFC 1183; effectively obsolete.
	pub(crate) const RT_higher: u8 = 0x00;
	pub(crate) const RT_lower: u8 = 21;
	pub(crate) const RT: Self = Self([Self::RT_higher, Self::RT_lower]);

	/// NSAP style A (address) record.
	///
	/// Defined in RFC 1706; effectively obsolete.
	pub(crate) const NSAP_higher: u8 = 0x00;
	pub(crate) const NSAP_lower: u8 = 22;
	pub(crate) const NSAP: Self = Self([Self::RT_higher, Self::RT_lower]);

	/// NSAP domain name pointer.
	///
	/// Defined in RFC 1348, RFC 1637 and RFC 1706; effectively obsolete.
	pub(crate) const NSAP_PTR_higher: u8 = 0x00;
	pub(crate) const NSAP_PTR_lower: u8 = 23;
	pub(crate) const NSAP_PTR: Self = Self([Self::NSAP_PTR_higher, Self::NSAP_PTR_lower]);

	/// Older DNSSEC Signature.
	///
	/// Defined in RFC 4034 amongst many; abandoned.
	pub(crate) const SIG_higher: u8 = 0x00;
	pub(crate) const SIG_lower: u8 = 24;
	pub(crate) const SIG: Self = Self([Self::SIG_higher, Self::SIG_lower]);

	/// Older DNSSEC Key.
	///
	/// Defined in RFC 4034 amongst many; abandoned.
	///
	/// Used only for SIG(0) (RFC 2931) and TKEY (RFC 2930).
	/// RFC 3445 eliminated use for application keys and limited their use to DNSSEC.
	/// RFC 3755 designates DNSKEY as the replacement within DNSSEC.
	/// RFC 4025 designates IPSECKEY as the replacement for use with IPsec.
	pub(crate) const KEY_higher: u8 = 0x00;
	pub(crate) const KEY_lower: u8 = 25;
	pub(crate) const KEY: Self = Self([Self::KEY_higher, Self::KEY_lower]);

	/// X.400 mail mapping information.
	///
	/// Defined in RFC 2163; effectively obsolete.
	pub(crate) const PX_higher: u8 = 0x00;
	pub(crate) const PX_lower: u8 = 26;
	pub(crate) const PX: Self = Self([Self::PX_higher, Self::PX_lower]);

	/// X.400 mail mapping information.
	///
	/// Defined in RFC 1712; effectively made obsolete by RFC 1876 which defined `LOC`.
	pub(crate) const GPOS_higher: u8 = 0x00;
	pub(crate) const GPOS_lower: u8 = 27;
	pub(crate) const GPOS: Self = Self([Self::GPOS_higher, Self::GPOS_lower]);

    /// Internet Protocol version 6 address.
	///
	/// Defined in RFC 3596.
	pub(crate) const AAAA_higher: u8 = 0x00;
	pub(crate) const AAAA_lower: u8 = 28;
	pub(crate) const AAAA: Self = Self([Self::AAAA_higher, Self::AAAA_lower]);

	/// Location (similar to Geographic Position, but with size and accuracy also encoded).
	///
	/// Defined in RFC 1876.
	pub(crate) const LOC_higher: u8 = 0x00;
	pub(crate) const LOC_lower: u8 = 29;
	pub(crate) const LOC: Self = Self([Self::LOC_higher, Self::LOC_lower]);

	/// Older DNSSEC concept.
	///
	/// Defined in RFC 3755 and RFC 2535; abandoned.
	pub(crate) const NXT_higher: u8 = 0x00;
	pub(crate) const NXT_lower: u8 = 30;
	pub(crate) const NXT: Self = Self([Self::NXT_higher, Self::NXT_lower]);

	/// Nimrod Endpoint Identifier
	///
	/// Defined by [Michael_Patton](http://ana-3.lcs.mit.edu/~jnc/nimrod/dns.txt); effectively obsolete.
	pub(crate) const EID_higher: u8 = 0x00;
	pub(crate) const EID_lower: u8 = 31;
	pub(crate) const EID: Self = Self([Self::EID_higher, Self::EID_lower]);

	/// Nimrod Locator
	///
	/// Defined by [Michael_Patton](http://ana-3.lcs.mit.edu/~jnc/nimrod/dns.txt); effectively obsolete.
	///
	/// Also has the misfortune of overlapping with RFC 1002 which defined `NB` for the same code.
	pub(crate) const NIMLOC_higher: u8 = 0x00;
	pub(crate) const NIMLOC_lower: u8 = 32;
	pub(crate) const NIMLOC: Self = Self([Self::EID_higher, Self::EID_lower]);

	/// Server locations, port numbers and preference for a particular service, eg http.
	///
	/// Defined in RFC 2782.
	///
	/// Also has the misfortune of overlapping with RFC 1002 which defined `NBSTAT` for the same code.
	pub(crate) const SRV_higher: u8 = 0x00;
	pub(crate) const SRV_lower: u8 = 33;
	pub(crate) const SRV: Self = Self([Self::SRV_higher, Self::SRV_lower]);

	/// ATM Address
	///
	/// Defined by [ATM Forum Technical Committee, "ATM Name System, V2.0", Doc ID: AF-DANS-0152.000, July 2000](http://www.broadband-forum.org/ftp/pub/approved-specs/af-dans-0152.000.pdf); effectively obsolete.
	pub(crate) const ATMA_higher: u8 = 0x00;
	pub(crate) const ATMA_lower: u8 = 34;
	pub(crate) const ATMA: Self = Self([Self::ATMA_higher, Self::ATMA_lower]);

	/// Naming Authority Pointer.
	///
	/// Used by Session Initiation Protocol (SIP) in various complex ways in conjunction with `SRV` records.
	///
	/// Defined by RFC 2915 (as redefined by RFCs 3401 to 3404 inclusive), RFC 2168 and RFC 3403.
	pub(crate) const NAPTR_higher: u8 = 0x00;
	pub(crate) const NAPTR_lower: u8 = 35;
	pub(crate) const NAPTR: Self = Self([Self::NAPTR_higher, Self::NAPTR_lower]);

	/// Key Exchanger.
	///
	/// Identifiers a key management agent for the domain name; not used by DNSSEC.
	///
	/// The intended usage seems to be mostly for IPSec.
	///
	/// Defined by RFC 2230.
	pub(crate) const KX_higher: u8 = 0x00;
	pub(crate) const KX_lower: u8 = 36;
	pub(crate) const KX: Self = Self([Self::KX_higher, Self::KX_lower]);

	/// Certificate.
	///
	/// Defined by RFC 4398.
	pub(crate) const CERT_higher: u8 = 0x00;
	pub(crate) const CERT_lower: u8 = 37;
	pub(crate) const CERT: Self = Self([Self::CERT_higher, Self::CERT_lower]);

	/// Older Internet Protocol version 6 address; abandoned.
	///
	/// Defined in RFC 3226, RFC 2874 and made obsolete in RFC 6563.
	pub(crate) const A6_higher: u8 = 0x00;
	pub(crate) const A6_lower: u8 = 38;
	pub(crate) const A6: Self = Self([Self::A6_higher, Self::A6_lower]);

	/// Delegation name record.
	///
	/// Defined in RFC 6672.
	pub(crate) const DNAME_higher: u8 = 0x00;
	pub(crate) const DNAME_lower: u8 = 39;
	pub(crate) const DNAME: Self = Self([Self::DNAME_higher, Self::DNAME_lower]);

	/// Kitchen Sink record.
	///
	/// Defined in a somewhat sarcastic internet draft [Donald_E_Eastlake](http://tools.ietf.org/html/draft-eastlake-kitchen-sink); abandoned.
	pub(crate) const SINK_higher: u8 = 0x00;
	pub(crate) const SINK_lower: u8 = 40;
	pub(crate) const SINK: Self = Self([Self::SINK_higher, Self::SINK_lower]);

	/// Address Prefix List.
	///
	/// Defined in RFC 3123; effectively obsolete.
	pub(crate) const APL_higher: u8 = 0x00;
	pub(crate) const APL_lower: u8 = 42;
	pub(crate) const APL: Self = Self([Self::APL_higher, Self::APL_lower]);

	/// Delegation Signer.
	///
	/// Part of DNSSEC.
	///
	/// Defined in RFC 4034 and 3658.
	pub(crate) const DS_higher: u8 = 0x00;
	pub(crate) const DS_lower: u8 = 43;
	pub(crate) const DS: Self = Self([Self::DS_higher, Self::DS_lower]);

	/// SSH (Secure Shell Protocol) public key fingerprint.
	///
	/// Defined in RFC 4255.
	pub(crate) const SSHFP_higher: u8 = 0x00;
	pub(crate) const SSHFP_lower: u8 = 44;
	pub(crate) const SSHFP: Self = Self([Self::SSHFP_higher, Self::SSHFP_lower]);

	/// IPSec Key.
	///
	/// Defined in RFC 4025.
	pub(crate) const IPSECKEY_higher: u8 = 0x00;
	pub(crate) const IPSECKEY_lower: u8 = 45;
	pub(crate) const IPSECKEY: Self = Self([Self::IPSECKEY_higher, Self::IPSECKEY_lower]);

	/// Resource Record Set Signature.
	///
	/// Part of DNSSEC.
	///
	/// Defined in RFC 4034 and RFC 3755.
	pub(crate) const RRSIG_higher: u8 = 0x00;
	pub(crate) const RRSIG_lower: u8 = 46;
	pub(crate) const RRSIG: Self = Self([Self::RRSIG_higher, Self::RRSIG_lower]);

	/// NSEC.
	///
	/// Part of DNSSEC.
	///
	/// Defined in RFC 4034 and RFC 3755.
	pub(crate) const NSEC_higher: u8 = 0x00;
	pub(crate) const NSEC_lower: u8 = 47;
	pub(crate) const NSEC: Self = Self([Self::NSEC_higher, Self::NSEC_lower]);

	/// DNSKEY.
	///
	/// Part of DNSSEC.
	///
	/// Defined in RFC 4034 and RFC 3755.
	pub(crate) const DNSKEY_higher: u8 = 0x00;
	pub(crate) const DNSKEY_lower: u8 = 48;
	pub(crate) const DNSKEY: Self = Self([Self::DNSKEY_higher, Self::DNSKEY_lower]);

	/// DHCP Identifier.
	///
	/// Defined in RFC 4701.
	pub(crate) const DHCID_higher: u8 = 0x00;
	pub(crate) const DHCID_lower: u8 = 49;
	pub(crate) const DHCID: Self = Self([Self::DHCID_higher, Self::DHCID_lower]);

	/// NSEC3.
	///
	/// Part of DNSSEC.
	///
	/// Defined in RFC 5155.
	pub(crate) const NSEC3_higher: u8 = 0x00;
	pub(crate) const NSEC3_lower: u8 = 50;
	pub(crate) const NSEC3: Self = Self([Self::NSEC3_higher, Self::NSEC3_lower]);

	/// NSEC3PARAM.
	///
	/// Part of DNSSEC.
	///
	/// Defined in RFC 5155.
	pub(crate) const NSEC3PARAM_higher: u8 = 0x00;
	pub(crate) const NSEC3PARAM_lower: u8 = 51;
	pub(crate) const NSEC3PARAM: Self = Self([Self::NSEC3PARAM_higher, Self::NSEC3PARAM_lower]);

	/// DNS-Based Authentication of Named Entities (DANE) for TLS.
	///
	/// Data represents a certificate association.
	///
	/// Defined in RFC 6698.
	pub(crate) const TLSA_higher: u8 = 0x00;
	pub(crate) const TLSA_lower: u8 = 52;
	pub(crate) const TLSA: Self = Self([Self::TLSA_higher, Self::TLSA_lower]);

	/// S/MIME certificate association.
	///
	/// Data represents a certificate association.
	///
	/// Defined in RFC 8162.
	pub(crate) const SMIMEA_higher: u8 = 0x00;
	pub(crate) const SMIMEA_lower: u8 = 53;
	pub(crate) const SMIMEA: Self = Self([Self::SMIMEA_higher, Self::SMIMEA_lower]);

	// 54 undefined.

	/// Host Identifier Protocol (HIP).
	///
	/// Defined in RFC 8005.
	pub(crate) const HIP_higher: u8 = 0x00;
	pub(crate) const HIP_lower: u8 = 55;
	pub(crate) const HIP: Self = Self([Self::SMIMEA_higher, Self::SMIMEA_lower]);

	/// ?
	///
	/// Defined in [Jim Reid](draft-reid-dnsext-zs-01.txt).
	pub(crate) const NINFO_higher: u8 = 0x00;
	pub(crate) const NINFO_lower: u8 = 56;
	pub(crate) const NINFO: Self = Self([Self::NINFO_higher, Self::NINFO_lower]);

	/// ?
	///
	/// Defined in [Jim Reid](draft-reid-dnsext-rkey-00.txt).
	pub(crate) const RKEY_higher: u8 = 0x00;
	pub(crate) const RKEY_lower: u8 = 57;
	pub(crate) const RKEY: Self = Self([Self::RKEY_higher, Self::RKEY_lower]);

	/// Trust Anchor LINK.
	///
	/// Defined in [Wouter Wijngaards](draft-reid-dnsext-rkey-00.txt).
	pub(crate) const TALINK_higher: u8 = 0x00;
	pub(crate) const TALINK_lower: u8 = 58;
	pub(crate) const TALINK: Self = Self([Self::TALINK_higher, Self::TALINK_lower]);

	/// Child DNSKEY.
	///
	/// Part of DNSSEC.
	///
	/// Defined in RFC 7344 and RFC 8078.
	pub(crate) const CDS_higher: u8 = 0x00;
	pub(crate) const CDS_lower: u8 = 59;
	pub(crate) const CDS: Self = Self([Self::CDS_higher, Self::CDS_lower]);

	/// Child Delegation Signer.
	///
	/// Part of DNSSEC.
	///
	/// Defined in RFC 7344 and RFC 8078.
	pub(crate) const CDNSKEY_higher: u8 = 0x00;
	pub(crate) const CDNSKEY_lower: u8 = 60;
	pub(crate) const CDNSKEY: Self = Self([Self::CDS_higher, Self::CDS_lower]);

    /// OpenPGP public key.
    ///
    /// Defined in RFC 7929.
	pub(crate) const OPENPGPKEY_higher: u8 = 0x00;
	pub(crate) const OPENPGPKEY_lower: u8 = 61;
	pub(crate) const OPENPGPKEY: Self = Self([Self::OPENPGPKEY_higher, Self::OPENPGPKEY_lower]);

    /// Child-To-Parent Synchronization.
    ///
    /// Defined in RFC 7477.
	pub(crate) const CSYNC_higher: u8 = 0x00;
	pub(crate) const CSYNC_lower: u8 = 62;
	pub(crate) const CSYNC: Self = Self([Self::CSYNC_higher, Self::CSYNC_lower]);

    /// Message Digest for DNS Zone.
    ///
    /// Defined in an [internet draft](https://datatracker.ietf.org/doc/draft-wessels-dns-zone-digest/).
	pub(crate) const ZONEMD_higher: u8 = 0x00;
	pub(crate) const ZONEMD_lower: u8 = 63;
	pub(crate) const ZONEMD: Self = Self([Self::ZONEMD_higher, Self::ZONEMD_lower]);

	// 64 - 98 are unassigned.

	/// Sender Policy Framework
	///
	/// Obsoleted in RFC 7208.
	pub(crate) const SPF_higher: u8 = 0x00;
	pub(crate) const SPF_lower: u8 = 99;
	pub(crate) const SPF: Self = Self([Self::SPF_higher, Self::SPF_lower]);

	/// UINFO.
	///
	/// Never formally defined by RFC and unsupported by BIND from the early 1990s.
	///
	/// Obsolete.
	pub(crate) const UINFO_higher: u8 = 0x00;
	pub(crate) const UINFO_lower: u8 = 100;
	pub(crate) const UINFO: Self = Self([Self::UINFO_higher, Self::UINFO_lower]);

	/// UID.
	///
	/// Never formally defined by RFC and unsupported by BIND from the early 1990s.
	///
	/// Obsolete.
	pub(crate) const UID_higher: u8 = 0x00;
	pub(crate) const UID_lower: u8 = 101;
	pub(crate) const UID: Self = Self([Self::UID_higher, Self::UID_lower]);

	/// GID.
	///
	/// Never formally defined by RFC and unsupported by BIND from the early 1990s.
	///
	/// Obsolete.
	pub(crate) const GID_higher: u8 = 0x00;
	pub(crate) const GID_lower: u8 = 102;
	pub(crate) const GID: Self = Self([Self::GID_higher, Self::GID_lower]);

	/// UNSPEC.
	///
	/// Never formally defined by RFC and unsupported by BIND from the early 1990s.
	///
	/// Obsolete.
	pub(crate) const UNSPEC_higher: u8 = 0x00;
	pub(crate) const UNSPEC_lower: u8 = 103;
	pub(crate) const UNSPEC: Self = Self([Self::UNSPEC_higher, Self::UNSPEC_lower]);

	/// Node Identifier for Identifier-Locator Network Protocol (ILNP).
	///
	/// Defined in RFC 6742.
	pub(crate) const NID_higher: u8 = 0x00;
	pub(crate) const NID_lower: u8 = 104;
	pub(crate) const NID: Self = Self([Self::NID_higher, Self::NID_lower]);

	/// 32-bit Locator for Identifier-Locator Network Protocol (ILNP) over Internet Protocol version 4.
	///
	/// Defined in RFC 6742.
	pub(crate) const L32_higher: u8 = 0x00;
	pub(crate) const L32_lower: u8 = 105;
	pub(crate) const L32: Self = Self([Self::L32_higher, Self::L32_lower]);

	/// 64-bit Locator for Identifier-Locator Network Protocol (ILNP) over Internet Protocol version 6.
	///
	/// Defined in RFC 6742.
	pub(crate) const L64_higher: u8 = 0x00;
	pub(crate) const L64_lower: u8 = 106;
	pub(crate) const L64: Self = Self([Self::L64_higher, Self::L64_lower]);

	/// FQDN for Identifier-Locator Network Protocol (ILNP).
	///
	/// Defined in RFC 6742.
	pub(crate) const LP_higher: u8 = 0x00;
	pub(crate) const LP_lower: u8 = 107;
	pub(crate) const LP: Self = Self([Self::LP_higher, Self::LP_lower]);

	/// An EUI-48 address.
	///
	/// Defined in RFC 7043.
	pub(crate) const EUI48_higher: u8 = 0x00;
	pub(crate) const EUI48_lower: u8 = 108;
	pub(crate) const EUI48: Self = Self([Self::EUI48_higher, Self::EUI48_lower]);

	/// An EUI-64 address.
	///
	/// Defined in RFC 7043.
	pub(crate) const EUI64_higher: u8 = 0x00;
	pub(crate) const EUI64_lower: u8 = 109;
	pub(crate) const EUI64: Self = Self([Self::EUI64_higher, Self::EUI64_lower]);

    /// URI.
    ///
    /// Defined in RFC 7553.
	pub(crate) const URI_higher: u8 = 0x01;
	pub(crate) const URI_lower: u8 = 0x00;
	pub(crate) const URI: Self = Self([Self::URI_higher, Self::URI_lower]);

    /// Certification Authority Authorization.
    ///
    /// Defined in RFC 6844.
	pub(crate) const CAA_higher: u8 = 0x01;
	pub(crate) const CAA_lower: u8 = 0x01;
	pub(crate) const CAA: Self = Self([Self::CAA_higher, Self::CAA_lower]);

    /// Application Visibility and Control.
	pub(crate) const AVC_higher: u8 = 0x01;
	pub(crate) const AVC_lower: u8 = 0x02;
	pub(crate) const AVC: Self = Self([Self::AVC_higher, Self::AVC_lower]);

    /// Digital Object Architecture.
	pub(crate) const DOA_higher: u8 = 0x01;
	pub(crate) const DOA_lower: u8 = 0x03;
	pub(crate) const DOA: Self = Self([Self::DOA_higher, Self::DOA_lower]);

    /// Automatic Multicast Tunneling Relay.
	pub(crate) const AMTRELAY_higher: u8 = 0x01;
	pub(crate) const AMTRELAY_lower: u8 = 0x04;
	pub(crate) const AMTRELAY: Self = Self([Self::AMTRELAY_higher, Self::AMTRELAY_lower]);

    /// DNSSEC Trust Authorities; never implemented.
	pub(crate) const TA_higher: u8 = 0x80;
	pub(crate) const TA_lower: u8 = 0x00;
	pub(crate) const TA: Self = Self([Self::TA_higher, Self::TA_lower]);

    /// DNSSEC Lookaside Validation.
    ///
    /// Defined in RFC 4431.
	pub(crate) const DLV_higher: u8 = 0x80;
	pub(crate) const DLV_lower: u8 = 0x01;
	pub(crate) const DLV: Self = Self([Self::DLV_higher, Self::DLV_lower]);

	#[inline(always)]
	pub(crate) fn upper_and_lower(self) -> (u8, u8)
	{
		unsafe { transmute(self) }
	}
	
	// "Well-known" types as defind by RFC 3597, Section 4 Domain Name Compression, paragraph 2 by reference to RFC 1035: "it is hereby specified that only the RR types defined in RFC1035 are to be considered "well-known"".
	#[inline(always)]
	pub(crate) const fn is_well_known(self) -> bool
	{
		match self
		{
			Self::CNAME => true,
			
			Self::HINFO => true,
			
			Self::MB => true,
			
			Self::MD => true,
			
			Self::MF => true,
			
			Self::MG => true,
			
			Self::MINFO => true,
			
			Self::MR => true,
			
			Self::MX => true,
			
			Self::NULL => true,
			
			Self::NS => true,
			
			Self::SOA => true,
			
			Self::TXT => true,
			
			_ => false,
		}
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) const fn rfc_3597_permits_legacy_compression_of_domain_names_in_sent_resource_data(self) -> bool
	{
		match self
		{
			// Previously permitted, but disallowed by RFC 3597, Section 4 Domain Name Compression, paragraph 3.
			
			// Was previously permited by RFC 2163, Section 4 The new DNS resource record for MIXER mapping rules: PX.
			Self::PX => false,
			
			// Was previously permited by RFC 2535, Section 4.1.7 Signer's Name Field.
			Self::SIG => false,
			
			// Was previously permited by RFC 2535, Section 5.2 NXT RDATA Format.
			Self::NXT => false,
			
			// "Well-known" types as defind by RFC 3597, Section 4 Domain Name Compression, paragraph 2 by reference to RFC 1035: "it is hereby specified that only the RR types defined in RFC1035 are to be considered "well-known"".
			//  RFC 3597, Section 4 Domain Name Compression, paragraph 2 defines the permitted set by using a confusing double-negative: "… servers MUST NOT compress domain names embedded in the RDATA of types that are … not well-known".
			// Note that not all of the "well-known" types actually can contain domain names.
			_ => self.is_well_known(),
		}
	}
	#[inline(always)]
	pub(crate) const fn rfc_3597_permits_legacy_compression_of_domain_names_in_received_resource_data(self) -> bool
	{
		match self
		{
			// RFC 3597, Section 4 Domain Name Compression, paragraph 4: "Receiving servers … SHOULD also decompress RRs of type RP, AFSDB, RT, SIG, PX, NXT, NAPTR, and SRV (although the current specification of the SRV RR in RFC2782 prohibits compression, RFC2052 mandated it, and some servers following that earlier specification are still in use)".
			
			Self::RP => true,
			
			Self::AFSDB => true,
			
			Self::RT => true,
			
			Self::SIG => true,
			
			Self::PX => true,
			
			Self::NXT => true,
			
			Self::NAPTR => true,
			
			Self::SRV => true,
			
			// "Well-known" types as defind by RFC 3597, Section 4 Domain Name Compression, paragraph 2 by reference to RFC 1035: "it is hereby specified that only the RR types defined in RFC1035 are to be considered "well-known"".
			// RFC 3597, Section 4 Domain Name Compression, paragraph 4: "Receiving servers MUST decompress domain names in RRs of well-known type"/
			// Note that not all of the "well-known" types actually can contain domain names.
			_ => self.is_well_known()
		}
	}
}
