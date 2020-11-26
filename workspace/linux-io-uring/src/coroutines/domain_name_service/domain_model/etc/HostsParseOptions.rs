// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Options for parsing `/etc/hosts` and similar files.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct HostsParseOptions
{
	/// Treats the unspecified Internet Protocol addresses (eg `0.0.0.0`) as 'never valid', ie a domain with child domains that can never exist.
	///
	/// The logic for turning `0.0.0.0` into a denied domain is to support <https://raw.githubusercontent.com/Ultimate-Hosts-Blacklist/Ultimate.Hosts.Blacklist/master/hosts/hosts0>.
	///
	/// The default is true.
	///
	/// Making this false will allow parsing of hosts files that otherwise would fail.
	pub treat_unspecified_internet_protocol_version_4_address_as_never_valid: bool,
	
	/// Instead of treating second and subsequent domain names as aliases, treat them as canonical names.
	///
	/// This permits parsing of the default Alpine Linux `/etc/hosts` file and matches the behaviour of dnsmasq for files like:-
	///
	/// ```bash
	/// 127.0.0.1	alpine.localdomain alpine localhost.localdomain localhost
	/// ::1		localhost localhost.localdomain
	/// ```
	///
	/// The default is false.
	pub everything_is_canonical: bool,

	/// Override the default domain name when parsing a hosts file.
	pub default_domain_name_override: Option<FullyQualifiedDomainName>
}

impl Default for HostsParseOptions
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			treat_unspecified_internet_protocol_version_4_address_as_never_valid: true,
			
			everything_is_canonical: false,
			
			default_domain_name_override: None,
		}
	}
}

impl HostsParseOptions
{
	#[inline(always)]
	fn default_domain_name<'a, 'b: 'a>(&self, default_domain_name: &'b FullyQualifiedDomainName) -> &'a FullyQualifiedDomainName
	{
		if let Some(ref default_domain_name_override) = self.default_domain_name_override
		{
			default_domain_name_override
		}
		else
		{
			default_domain_name
		}
	}
}
