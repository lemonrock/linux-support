// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The following property tags are defined by <https://www.iana.org/assignments/pkix-parameters/pkix-parameters.xhtml>.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CertificateAuthorityAuthorizationPropertyTag
{
	/// `issue`.
	///
	/// Authorization Entry by Domain.
	///
	/// Defined by RFC 6844.
	AuthorizationEntryByDomain,

	/// `issuewild`.
	///
	/// Authorization Entry by Wildcard Domain.
	///
	/// Defined by RFC 6844.
	AuthorizationEntryByWildcardDomain,

	/// `iodef`.
	///
	/// Report incident by IODEF report.
	///
	/// Defined by RFC 6844.
	ReportIncidentByIodefReport,

	/// `contactemail`.
	///
	/// Authorized e-mail contact for domain validation.
	///
	/// Defined by CA/Browser Forum 1.6.3.
	AuthorizedEMailContactForDomainValidation,
}

impl CertificateAuthorityAuthorizationPropertyTag
{
	#[inline(always)]
	pub fn known_tag(tag_name: &[u8]) -> Option<&Option<Self>>
	{
		use self::CertificateAuthorityAuthorizationPropertyTag::*;
		static KnownTags: Map<&'static [u8], Option<CertificateAuthorityAuthorizationPropertyTag>> = phf_map!
		{
    		b"issue" => Some(AuthorizationEntryByDomain),
    		b"issuewild" => Some(AuthorizationEntryByWildcardDomain),
    		b"iodef" => Some(ReportIncidentByIodefReport),
    		b"contactemail" => Some(AuthorizedEMailContactForDomainValidation),
    		b"auth" => None,
    		b"path" => None,
    		b"policy" => None,
		};
		
		KnownTags.get(tag_name)
	}
}
