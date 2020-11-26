// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A name to search for.
///
/// Using anything other than `FullyQualified` is often a bad idea; see the [SSAC Advisory on Search List Processing (SSAC064)](https://www.icann.org/en/system/files/files/sac-064-en.pdf).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SearchName
{
	/// A fully qualified name.
	FullyQualified(FullyQualifiedDomainName),

	/// A relative name; can also effectively be equivalent to `FullyQualified` or `Host`.
	Relative(RelativeDomainName),

	/// A host name.
	Host(HostNameLabel),
}

impl SearchName
{
	/// Parses a name as might be found in `/etc/hosts` or used with the traditional libc resolver functions.
	pub fn parse(byte_string: &[u8]) -> Result<Self, EfficientCaseFoldedNameParseError>
	{
		use self::SearchName::*;
		
		if byte_string.is_empty()
		{
			return Ok(FullyQualified(FullyQualifiedDomainName::root()))
		}
		
		let parsed_name = EfficientCaseFoldedName::from_byte_string_ending_with_optional_trailing_period(byte_string)?;
		
		if etc_host_name.get_unchecked_value_safe(etc_host_name.len() - 1) == b'.'
		{
			return Ok(FullyQualified(parsed_name))
		}
		else if parsed_name.is_top_level()
		{
			Ok(Host(parsed_name.host_name().unwrap()))
		}
		else
		{
			Ok(Relative(RelativeDomainName(parsed_name)))
		}
	}
	
	/// Iterate over all possible fully qualified domain names (FQDNs).
	///
	/// Note that if using `Self::Host()` and `harden_using_iana_ssac_advisory_on_search_list_processing` is true, the iterator will not produce any domain names.
	#[inline(always)]
	pub fn iterate_fully_qualified_domain_names<'resolv_conf, 'search_name, 'next>(&'search_name self, resolv_conf: &'resolv_conf ResolvConf, harden_using_iana_ssac_advisory_on_search_list_processing: bool) -> SearchListIterator<'resolv_conf, 'search_name, 'next>
	{
		SearchListIterator::new(resolv_conf, self, harden_using_iana_ssac_advisory_on_search_list_processing)
	}
	
	/// To a Fully Qualified Domain Name (FQDN).
	pub fn to_fully_qualified_domain_name(self, parent_domain_name: &FullyQualifiedDomainName) -> Result<FullyQualifiedDomainName, EfficientCaseFoldedNameParseError>
	{
		use self::SearchName::*;
		
		match self
		{
			FullyQualified(fully_qualified_domain_name) => Ok(fully_qualified_domain_name),
			
			Relative(relative_domain_name) => parent_domain_name.prepend_relative_name(&relative_domain_name),
			
			Host(host_name) => parent_domain_name.prepend_host_name(&host_name),
		}
	}
}
