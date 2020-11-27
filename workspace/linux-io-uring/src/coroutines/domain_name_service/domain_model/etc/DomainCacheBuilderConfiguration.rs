// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Domain caceh builder configuration.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct DomainCacheBuilderConfiguration
{
	/// Default is to try to use `LOCALDOMAIN`, then `domain` in `/etc/resolv.conf`, then the first `search` in `/etc/resolv.conf`, then extract a domain name from `uname.nodename`, then fallback to root (`.`).
	///
	/// This may return a different result to the `dnsdomainname` binary, as that does a DNS search for `uname.nodename`.
	#[serde(default = "DomainCacheBuilderConfiguration::default_domain_name_choices_default")] pub default_domain_name_choices: Vec<DefaultDomainNameChoice>,
	
	/// How to parse `/etc/hosts`.
	///
	/// Default is to parse and do so strictly, but may need to be lenient for Alpine Linux.
	#[serde(default = "DomainCacheBuilderConfiguration::etc_hosts_file_parse_options_default")] pub etc_hosts_file_parse_options: Option<HostsParseOptions>,

	/// Parse all hosts file in these folder paths.
	#[serde(default)] pub hosts_folder_paths: HashMap<PathBuf, HostsParseOptions>,

	/// Parse these specific hosts files with these specifc parse options.
	#[serde(default)] pub hosts_file_paths: HashMap<PathBuf, HostsParseOptions>,

	/// Parse these specific host aliases files with these specifc parse options.
	///
	/// These are rare.
	#[serde(default)] pub host_aliases_file_paths: HashSet<PathBuf>,

	/// Never valid.
	#[serde(default = "DomainCacheBuilderConfiguration::never_valid_domain_names_default")] pub never_valid_domain_names: HashSet<FullyQualifiedDomainName>,

	/// Always valid.
	#[serde(default = "EfficientCaseFoldedName::top_level_domain_names")] pub top_level_domain_names: HashSet<FullyQualifiedDomainName>,
	
	/// Always valid.
	#[serde(default = "DomainCacheBuilderConfiguration::always_valid_domain_names_but_subdomains_are_not_valid_domain_names_default")] pub always_valid_domain_names_but_subdomains_are_not_valid_domain_names: HashMap<FullyQualifiedDomainName, QueryTypesCache>,
	
	/// Always valid.
	#[serde(default = "DomainCacheBuilderConfiguration::local_machine_only_domain_names_default")] pub local_machine_only_domain_names: HashSet<FullyQualifiedDomainName>,
	
	/// If anything in this list is not locally administered then it should not leave the box.
	#[serde(default = "DomainCacheBuilderConfiguration::local_domain_names_default")] pub local_domain_names: HashSet<FullyQualifiedDomainName>,
}

impl DomainCacheBuilderConfiguration
{
	/// Configure.
	#[inline(always)]
	pub fn configure<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineHeapSize: MemorySize>(&self, proc_path: &ProcPath, etc_path: &EtcPath, gtacsa: &'static GTACSA) -> Result<(DomainCache<GTACSA>, ResolvConf), DomainCacheBuilderConfigurationError>
	{
		use self::DomainCacheBuilderConfigurationError::*;
		
		let default_domain_name = DefaultDomainNameChoice::resolve_until_one_works(self.default_domain_name_choices.iter(), proc_path, etc_path);
	
		let mut domain_cache_builder = DomainCacheBuilder::new(&default_domain_name);
		
		if let Some(ref parse_options) = self.etc_hosts_file_parse_options
		{
			domain_cache_builder.parse_etc_hosts_file(etc_path, parse_options).map_err(|error| ParseHosts { error, file_path: etc_path.hosts() })?;
		}
		
		for (folder_path, parse_options) in self.hosts_folder_paths.iter()
		{
			Self::parse_hosts_files_in_folder_path(&folder_path, parse_options)
		}
		
		for (file_path, parse_options) in self.hosts_folder_paths.iter()
		{
			domain_cache_builder.parse_hosts_file(&file_path, parse_options).map_err(|error| ParseHosts { error, file_path: file_path.clone() })?;
		}
		
		domain_cache_builder.parse_HOSTALIASES_file().map_err(|error| CouldNotParseHostAliasesFromEnvironment { error })?;
		
		for file_path in self.host_aliases_file_paths.iter()
		{
			domain_cache_builder.parse_hostaliases_file(&file_path).map_err(|error| CouldNotParseHostAliasesFromFile { error, file_path: file_path.clone() })?;
		}
		
		let resolv_conf = domain_cache_builder.parse_resolv_conf(etc_path)?;
		let always_valid_locally_administered_domains = resolv_conf.always_valid_locally_administered_domains();
		domain_cache_builder.never_valid_domain_names(&self.never_valid_domain_names);
		domain_cache_builder.always_valid_domain_names(&self.top_level_domain_names, &always_valid_locally_administered_domains);
		domain_cache_builder.always_valid_domain_names_but_subdomains_are_not_valid_domain_names(&self.always_valid_domain_names_but_subdomains_are_not_valid_domain_names);
		domain_cache_builder.local_machine_only_domain_names(&self.local_machine_only_domain_names);
		domain_cache_builder.never_valid_local_domain_names(&self.local_domain_names, &always_valid_locally_administered_domains);
		
		let domain_cache = domain_cache_builder.finish(gtacsa);
		
		Ok((domain_cache, resolv_conf))
	}
	
	#[inline(always)]
	fn parse_hosts_files_in_folder_path(folder_path: &Path, parse_options: &HostsParseOptions) -> Result<(), DomainCacheBuilderConfigurationError>
	{
		use self::DomainCacheBuilderConfigurationError::*;
		
		for entry in folder_path.read_dir().map_err(|error| ParseHostsFileInFolderCouldNotReadDirectory { error, folder_path: folder_path.to_path_buf() })
		{
			let entry = entry.map_err(|error| ParseHostsFileInFolderCouldNotReadEntry { error, folder_path: folder_path.to_path_buf() })?;
			
			if entry.file_type().map_err(|error| ParseHostsFileInFolderCouldNotReadEntryFileType { error, file_path: entry.path() })?.is_file()
			{
				let file_path = entry.path();
				domain_cache_builder.parse_hosts_file(&file_path, parse_options).map_err(|error| ParseHosts { error, file_path })?;
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn default_domain_name_choices_default() -> Vec<DefaultDomainNameChoice>
	{
		vec!
		[
			DefaultDomainNameChoice::EtcResolvConf
		]
	}
	
	#[inline(always)]
	fn etc_hosts_file_parse_options_default() -> Option<HostsParseOptions>
	{
		Some(HostsParseOptions::default())
	}
	
	#[inline(always)]
	fn never_valid_domain_names_default() -> HashSet<FullyQualifiedDomainName>
	{
		fast_secure_hash_set!
		[
			// RFC 6761, Section 6.1, Domain Name Reservation Considerations for Private Addresses.
			EfficientCaseFoldedName::third_level(EfficientCaseFoldedLabel::byte(10), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(16), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(17), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(18), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(19), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(20), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(21), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(22), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(23), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(24), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(25), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(26), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(27), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(28), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(29), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(30), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(31), EfficientCaseFoldedLabel::byte(172), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(168), EfficientCaseFoldedLabel::byte(192), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			
			// RFC 6762, Section 12, Special Characteristics of Multicast DNS Domains.
			EfficientCaseFoldedName::fourth_level_unwrap(EfficientCaseFoldedLabel::byte(254), EfficientCaseFoldedLabel::byte(169), EfficientCaseFoldedLabel::in_addr, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fifth_level_unwrap(EfficientCaseFoldedLabel::nibble(0x8), EfficientCaseFoldedLabel::nibble(0xE), EfficientCaseFoldedLabel::nibble(0xF), EfficientCaseFoldedLabel::ip6, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fifth_level_unwrap(EfficientCaseFoldedLabel::nibble(0x9), EfficientCaseFoldedLabel::nibble(0xE), EfficientCaseFoldedLabel::nibble(0xF), EfficientCaseFoldedLabel::ip6, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fifth_level_unwrap(EfficientCaseFoldedLabel::nibble(0xA), EfficientCaseFoldedLabel::nibble(0xE), EfficientCaseFoldedLabel::nibble(0xF), EfficientCaseFoldedLabel::ip6, EfficientCaseFoldedLabel::arpa),
			EfficientCaseFoldedName::fifth_level_unwrap(EfficientCaseFoldedLabel::nibble(0xB), EfficientCaseFoldedLabel::nibble(0xE), EfficientCaseFoldedLabel::nibble(0xF), EfficientCaseFoldedLabel::ip6, EfficientCaseFoldedLabel::arpa),
			
			// RFC 6761, Section 6.5 Domain Name Reservation Considerations for Example Domains.
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::example),
			EfficientCaseFoldedName::second_level(EfficientCaseFoldedLabel::example, EfficientCaseFoldedLabel::com),
			EfficientCaseFoldedName::second_level(EfficientCaseFoldedLabel::example, EfficientCaseFoldedLabel::net),
			EfficientCaseFoldedName::second_level(EfficientCaseFoldedLabel::example, EfficientCaseFoldedLabel::org),
			
			// RFC 6761, Section 6.4, Domain Name Reservation Considerations for "invalid.".
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::invalid),
			
			// RFC 7686, Section 2, The ".onion" Special-Use Domain Name.
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::onion),
			
			// RFC 6761, Section 6.2, Domain Name Reservation Considerations for "test.".
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::test),
			
			// See <https://www.iana.org/domains/reserved>.
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Arabic_Arabic),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Persian_Arabic),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Chinese_Han_Simplified),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Chinese_Han_Traditional),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Russion_Cyrillic),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Hindi_Devangari),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Greek_Greek),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Korean_Hangul),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Yiddish_Hebrew),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Japanese_Katakana),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::Tamil_Tamil),
			
			// Legacy ip6 PTR resolution.
			EfficientCaseFoldedName::second_level(EfficientCaseFoldedLabel::ip6, EfficientCaseFoldedLabel::int),
		]
	}
	
	#[inline(always)]
	fn always_valid_domain_names_but_subdomains_are_not_valid_domain_names_default() -> HashMap<FullyQualifiedDomainName, QueryTypesCache>
	{
		fast_secure_hash_map!
		[
			// RFC-ietf-6tisch-minimal-security-15.
			EfficientCaseFoldedName::second_level(EfficientCaseFoldedLabel::_6tisch, EfficientCaseFoldedLabel::arpa) => QueryTypesCache::default(),
			
			// RFC 8880, Section 7.1, Special Use Domain Name 'ipv4only.arpa'.
			//
			// See also RFC 7050.
			// Furthermore, for Internet Protocol version 4, it has the fixed `A` records `192.0.0.170` and `192.0.0.171`; these are defined in the [IANA IPv5 Special-Purpose Address Registry](https://www.iana.org/assignments/iana-ipv4-special-registry/iana-ipv4-special-registry.xhtml).
			EfficientCaseFoldedName::second_level(EfficientCaseFoldedLabel::ipv4only, EfficientCaseFoldedLabel::arpa) => QueryTypesCache::for_ipv4only_arpa(),
			
			// RFC 8880, Section 7.2, Names '170.0.0.192.in-addr.arpa' and '171.0.0.192.in-addr.arpa'.
			EfficientCaseFoldedName::internet_protocol_version_4_pointer_unchecked(Ipv4Addr::new(170, 0, 0, 192)) => QueryTypesCache::pointer_for_ipv4only_arpa(),
			EfficientCaseFoldedName::internet_protocol_version_4_pointer_unchecked(Ipv4Addr::new(171, 0, 0, 192)) => QueryTypesCache::pointer_for_ipv4only_arpa(),
		]
	}
	
	#[inline(always)]
	fn local_machine_only_domain_names_default() -> HashSet<FullyQualifiedDomainName>
	{
		fast_secure_hash_set!
		[
			// RFC 6761, Section 6.3, Domain Name Reservation Considerations for "localhost.".
			//
			// All records above this domain should resolve to `127.0.0.1` / `::1`.
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::localhost),
			
			// Commonly used on Linux systems, and special-cased by systemd.
			//
			// All records above this domain should resolve to `127.0.0.1` / `::1`.
			EfficientCaseFoldedName::second_level(EfficientCaseFoldedLabel::localhost, EfficientCaseFoldedLabel::localdomain),
		]
	}
	
	#[inline(always)]
	fn local_domain_names_default() -> HashSet<FullyQualifiedDomainName>
	{
		fast_secure_hash_set!
		[
			// RFC 6762, Section 12, Special Characteristics of Multicast DNS Domains.
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::local),
			
			// Commonly used on Linux systems, and special-cased by systemd.
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::localdomain),
			
			// RFC 8375.
			EfficientCaseFoldedName::second_level(EfficientCaseFoldedLabel::home, EfficientCaseFoldedLabel::arpa),
			
			// Based on RFC 7788.
			//
			// This usage has been deprecated by RFC 8375 replacing ".home." with ".home.arpa.".
			// However, it is also used by RFC 6762, Appendix G.
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::home),
			
			// Based on RFC 6762, Appendix G: Private DNS Namespace.
			//
			// This contains 'sepcial use' multicast DNS domain names in common use.
			// It excludes ".local." and ".home.".
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::intranet),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::internal),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::private),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::corp),
			EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::lan),
		]
	}
}
