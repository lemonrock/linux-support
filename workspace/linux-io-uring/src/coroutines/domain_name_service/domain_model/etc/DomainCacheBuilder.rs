// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Builds a `DomainCache`.
pub struct DomainCacheBuilder<'a>
{
	default_domain_name: &'a FullyQualifiedDomainName,
	
	map: HashMap<AliasOrDomainTarget, DomainCacheEntry>,
}

impl<'a> DomainCacheBuilder<'a>
{
	#[inline(always)]
	pub fn new(default_domain_name: &'a FullyQualifiedDomainName) -> Self
	{
		Self
		{
			default_domain_name,
			
			map: HashMap::new(),
		}
	}
	
	/// Finish building.
	#[inline(always)]
	pub fn finish<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, CoroutineHeapSize: MemorySize>(self, gtacsa: &'static GTACSA) -> DomainCache<GTACSA>
	{
		self.map.shrink_to_fit();
		DomainCache::new(self.map, gtacsa)
	}
	
	/// Does nothing if the environment variable `HOSTALIASES` is not defined.
	#[inline(always)]
	pub fn parse_HOSTALIASES_file(&mut self) -> Result<(), ParseHostsError>
	{
		if let Some(host_aliases_file_path) = var_os("HOSTALIASES")
		{
			let file_path = PathBuf::from(host_aliases_file_path);
			self.parse_hostaliases_file(&file_path)
		}
		else
		{
			Ok(())
		}
	}
	
	/// Parse a `HOSTALIASES` file.
	#[inline(always)]
	pub fn parse_hostaliases_file(&mut self, file_path: &Path) -> Result<(), ParseHostsError>
	{
		let parse_options = HostsParseOptions::default();
		
		let raw = file_path.read_raw()?;
		let string: String = String::from_utf8(raw.into_vec())?;
		
		for (line_index, line) in string.split_terminator(b'\n').enumerate()
		{
			self.parse_hostaliases_line(line_index, line, &parse_options)?;
		}
		
		Ok(())
	}
	
	/// Parse an `/etc/hosts` file.
	#[inline(always)]
	pub fn parse_etc_hosts_file(&mut self, etc_path: &EtcPath, parse_options: &HostsParseOptions) -> Result<(), ParseHostsError>
	{
		let file_path = etc_path.hosts();
		self.parse_hosts_file(&file_path, parse_options)
	}
	
	/// Parse a `hosts` file.
	pub fn parse_hosts_file(&mut self, file_path: &Path, parse_options: &HostsParseOptions) -> Result<(), ParseHostsError>
	{
		let raw = file_path.read_raw()?;
		let string: String = String::from_utf8(raw.into_vec())?;
		
		for (line_index, line) in string.split_terminator(b'\n').enumerate()
		{
			self.parse_hosts_line(line_index, line, parse_options)?;
		}
		
		Ok(())
	}
	
	/// Parse `/etc/resolv.conf`, to use with `finish()`.
	///
	/// Ensures the same default domain is used as the DomainCacheBuilder.
	pub fn parse_resolv_conf(&self, etc_path: &EtcPath) -> Result<ResolvConf, ParseEtcResolvConfError>
	{
		ResolvConf::new_from_environment(etc_path, self.default_domain_name)
	}
	
	#[inline(always)]
	pub(crate) fn try_potentially_blocked_internet_protocol_address(&mut self, internet_protocol_address: Option<IpAddr>, canonical_name: &DomainTarget, line_index: usize) -> Result<(), DomainCacheBuilderError>
	{
		use self::DomainCacheEntry::*;
		use self::FastSecureHashMapEntry::*;
		use self::DomainCacheBuilderError::*;
		
		match self.map.entry(canonical_name.clone())
		{
			Vacant(vacant) =>
			{
				vacant.insert(DomainCacheEntry::fixed_internet_protocol_address_potentially_blocked(internet_protocol_address));
			}
			
			Occupied(occupied) =>
			{
				match (internet_protocol_address, occupied.get_mut())
				{
					(None, NeverValid) => Ok(()),
					
					(Some(_), NeverValid) => return Err(CanonicalNameWasPreviouslyBlockedButNowIsDefined { canonical_name: occupied.replace_key() }),
					
					(Some(internet_protocol_address), Fixed { fixed_domain_cache_entry: FixedDomainCacheEntry::QueryTypesFixed(query_types_fixed), .. }) =>
					{
						query_types_fixed.add_internet_protocol_address(internet_protocol_address);
					}
					
					(None, Fixed { fixed_domain_cache_entry: FixedDomainCacheEntry::QueryTypesFixed(_), .. }) => return Err(CanonicalNameWasPreviouslyDefinedButNowIsBlocked { canonical_name: occupied.replace_key() }),
					
					(_, Fixed { fixed_domain_cache_entry: FixedDomainCacheEntry::Alias(alias), .. }) => return Err(CanonicalNameWasPreviouslyDefinedAsAnAlias { canonical_name: occupied.replace_key(), previously_defined_alias: alias.clone() }),
					
					_ => panic!("Unexpected former entry"),
				}
			}
		}
		
		if let Some(internet_protocol_address) = internet_protocol_address
		{
			self.add_pointer_record(canonical_name, internet_protocol_address)?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	pub(crate) fn add_pointer_record(&mut self, canonical_name: &DomainTarget, internet_protocol_address: IpAddr) -> Result<(), DomainCacheBuilderError>
	{
		use self::FastSecureHashMapEntry::*;
		use self::DomainCacheBuilderError::*;
		
		use self::IpAddr::*;
		
		let domain_name = match internet_protocol_address
		{
			V4(v4) => EfficientCaseFoldedName::internet_protocol_version_4_pointer_unchecked(v4),
			
			V6(v6) => EfficientCaseFoldedName::internet_protocol_version_6_pointer_unchecked(v6),
		};
		
		match self.map.entry(domain_name)
		{
			Vacant(vacant) =>
			{
				vacant.insert(DomainCacheEntry::fixed_pointer(canonical_name));
				
				Ok(())
			}
			
			Occupied(occupied) => if let DomainCacheEntry::Fixed { fixed_domain_cache_entry: FixedDomainCacheEntry::QueryTypesFixed(query_types_fixed), .. } = occupied.get_mut()
			{
				let pointers = query_types_fixed.PTR.as_mut().ok_or(InternetProtocolAddressPointerNameWasPreviouslyDefinedAsSomethingElse { internet_protocol_address_pointer_name: occupied.replace_key() })?;
				pointers.add_inefficient(PointerName::new(canonical_name.clone()))
			}
			else
			{
				Err(InternetProtocolAddressPointerNameWasPreviouslyDefinedAsSomethingElse { internet_protocol_address_pointer_name: occupied.replace_key() })
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn try_alias(&mut self, canonical_name: &DomainTarget, alias_name: Alias) -> Result<(), DomainCacheBuilderError>
	{
		use self::FastSecureHashMapEntry::*;
		
		match self.map.entry(alias_name)
		{
			Vacant(vacant) =>
			{
				vacant.insert(DomainCacheEntry::fixed_alias(canonical_name));
				
				Ok(())
			}
			
			Occupied(occupied) =>
			{
				use self::DomainCacheBuilderError::*;
				
				match occupied.get()
				{
					Left(_) => Err(AliasNameWasPreviouslyDefinedAsACanonicalName { alias_name: occupied.replace_key() }),
					
					Right(existing) => if existing.ne(&canonical_name)
					{
						Err(AliasNameWasPreviouslyDefinedWithADifferentTargetName { alias_name: occupied.replace_key(), previously_defined_alias: value.clone() })
					}
					else
					{
						Ok(())
					},
				}
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn never_valid_domain_names(&mut self, never_valid_domain_names: &HashSet<FullyQualifiedDomainName>)
	{
		for never_valid_domain_name in never_valid_domain_names.iter()
		{
			self.overwrite_never_valid_domain_name(never_valid_domain_name.clone())
		}
	}
	
	#[inline(always)]
	pub(crate) fn always_valid_domain_names(&mut self, top_level_domain_names: &HashSet<DomainTarget>, always_valid_locally_administered_domains: &HashSet<DomainTarget>)
	{
		self.overwrite_always_valid_domain_name(EfficientCaseFoldedName::root());
		
		for always_valid_domain_name in top_level_domain_names.iter()
		{
			self.overwrite_always_valid_domain_name(always_valid_domain_name.clone())
		}
		
		for always_valid_domain_name in always_valid_locally_administered_domains.iter()
		{
			self.overwrite_always_valid_domain_name(always_valid_domain_name.clone())
		}
	}
	
	#[inline(always)]
	pub(crate) fn always_valid_domain_names_but_subdomains_are_not_valid_domain_names(&mut self, always_valid_domain_names_but_subdomains_are_not_valid_domain_names: &HashMap<DomainTarget, QueryTypesCache>)
	{
		for (domain_name, query_types_cache) in always_valid_domain_names_but_subdomains_are_not_valid_domain_names.iter()
		{
			self.overwrite_always_valid_domain_name_but_subdomains_are_not_valid_domain_names(domain_name.clone(), query_types_cache.clone())
		}
	}
	
	#[inline(always)]
	pub(crate) fn local_machine_only_domain_names(&mut self, local_machine_only_domain_names: &HashSet<DomainTarget>)
	{
		for local_machine_only_domain_name in local_machine_only_domain_names.iter()
		{
			self.overwrite_local_machine_only_domain_name(local_machine_only_domain_name)
		}
	}
	
	#[inline(always)]
	pub(crate) fn never_valid_local_domain_names(&mut self, local_domain_names: &HashSet<DomainTarget>, always_valid_locally_administered_domains: &HashSet<DomainTarget>)
	{
		for never_valid_local_domain_name in local_domain_names.iter()
		{
			if always_valid_locally_administered_domains.contains(never_valid_local_domain_name)
			{
				continue
			}
			self.overwrite_never_valid_domain_name(never_valid_local_domain_name.clone())
		}
	}
	
	#[inline(always)]
	fn overwrite_always_valid_domain_name(&mut self, always_valid_domain_name: DomainTarget)
	{
		self.overwrite(always_valid_domain_name, DomainCacheEntry::always_valid_domain_name())
	}
	
	#[inline(always)]
	fn overwrite_always_valid_domain_name_but_subdomains_are_not_valid_domain_names(&mut self, always_valid_domain_name: DomainTarget, query_types_cache: QueryTypesCache)
	{
		self.overwrite(always_valid_domain_name, DomainCacheEntry::always_valid_domain_name_but_subdomains_are_not_valid_domain_names(query_types_cache))
	}
	
	#[inline(always)]
	fn overwrite_local_machine_only_domain_name(&mut self, local_machine_only_domain_name: &DomainTarget)
	{
		self.overwrite(local_machine_only_domain_name.clone(), DomainCacheEntry::fixed_local_machine_only());
		
		self.overwrite(EfficientCaseFoldedName::internet_protocol_version_4_pointer_unchecked(Ipv4Addr::LOCALHOST), DomainCacheEntry::fixed_pointer(local_machine_only_domain_name));
		self.overwrite(EfficientCaseFoldedName::internet_protocol_version_6_pointer_unchecked(Ipv6Addr::LOCALHOST), DomainCacheEntry::fixed_pointer(local_machine_only_domain_name));
	}
	
	#[inline(always)]
	fn overwrite_never_valid_domain_name(&mut self, never_valid_domain_name: DomainTarget)
	{
		self.overwrite(never_valid_domain_name, DomainCacheEntry::NeverValid)
	}
	
	#[inline(always)]
	fn overwrite(&mut self, domain_name: FullyQualifiedDomainName, entry: DomainCacehEntry)
	{
		self.map.insert(never_valid_domain_name, entry);
	}
	
	#[inline(always)]
	fn parse_hostaliases_line(&mut self, line_index: usize, line: &str, parse_options: &HostsParseOptions) -> Result<(), ParseHostsError>
	{
		use self::ParseHostsError::*;
		
		let line_without_comment = Self::parse_line_without_comment(line);
		
		if line_without_comment.is_empty()
		{
			return Ok(())
		}
		
		let mut fields = line_without_comment.split_ascii_whitespace();
		
		let alias_name = self.parse_alias_name(parse_options, fields.next().unwrap(), line_index, 0)?;
		let canonical_name = self.parse_hosts_canonical_name(parse_options, &mut fields, line_index)?;
		
		if fields.next.is_some()
		{
			return Err(MoreThanOneCanonicalNameInHostAliasesFile { line_index })
		}
		
		self.try_alias(&canonical_name, alias_name).map_err(|error| AliasName { line_index, alias_index, error })
	}
	
	#[inline(always)]
	fn parse_hosts_line(&mut self, line_index: usize, line: &str, parse_options: &HostsParseOptions) -> Result<(), ParseHostsError>
	{
		let line_without_comment = Self::parse_line_without_comment(line);
		
		if line_without_comment.is_empty()
		{
			return Ok(())
		}
		
		let mut fields = line_without_comment.split_ascii_whitespace();
		
		let internet_protocol_address = Self::parse_hosts_internet_protocol_address(&mut fields, line_index, parse_options)?;
		let canonical_name = self.parse_hosts_canonical_name(parse_options, &mut fields, line_index)?;
		
		self.try_potentially_blocked_internet_protocol_address(internet_protocol_address, &canonical_name, line_index).map_err(|error| ParseHostsError::CanonicalName { line_index, error })?;
		
		self.add_hosts_alias_entries(parse_options, internet_protocol_address, canonical_name, &mut fields, line_index)?;
		
		Ok(())
	}
	
	fn add_hosts_alias_entries<'whitespace>(&mut self, parse_options: &HostsParseOptions, internet_protocol_address: Option<IpAddr>, canonical_name: FullyQualifiedDomainName, fields: &mut SplitAsciiWhitespace<'whitespace>, line_index: usize) -> Result<(), ParseHostsError>
	{
		use self::ParseHostsError::*;
		for (alias_index, alias_name_field) in fields.enumerate()
		{
			let alias_name = self.parse_alias_name(parse_options, alias_name_field, line_index, alias_index)?;
			
			if parse_options.everything_is_canonical
			{
				self.try_potentially_blocked_internet_protocol_address(internet_protocol_address, &canonical_name, line_index).map_err(|error| AliasName { line_index, alias_index, error })?;
			}
			else
			{
				self.try_alias(&canonical_name, alias_name).map_err(|error| AliasName { line_index, alias_index, error })?;
			}
		}
		
		Ok(())
	}
	
	/// Some `/etc/hosts` files contains addresses like `fe80::1%lo0` which Rust's internet protocol address parser can't handle.
	fn parse_hosts_internet_protocol_address<'whitespace>(fields: &mut SplitAsciiWhitespace<'whitespace>, line_index: usize, parse_options: &HostsParseOptions) -> Result<Option<IpAddr>, ParseHostsError>
	{
		let internet_protocol_address_field = fields.next().unwrap();
		let without_suffix = internet_protocol_address_field.split('%').next().unwrap();
		let address = IpAddr::from_str(without_suffix).map_err(|error| ParseHostsError::InvalidInternetProtocolAddress { line_index, error })?;
		
		if parse_options.treat_unspecified_internet_protocol_version_4_address_as_never_valid
		{
			let is_block_list = address.is_unspecified();
			
			if is_block_list
			{
				Ok(None)
			}
			else
			{
				Ok(Some(address))
			}
		}
		else
		{
			Ok(Some(address))
		}
	}
	
	fn parse_hosts_canonical_name<'whitespace>(&self, parse_options: &HostsParseOptions, fields: &mut SplitAsciiWhitespace<'whitespace>, line_index: usize) -> Result<FullyQualifiedDomainName, ParseHostsError>
	{
		use self::ParseHostsError::*;
		
		let canonical_name_field = fields.next().ok_or(MissingCanonicalNameField { line_index })?;
		
		let canonical_search_name = SearchName::parse(canonical_name_field.as_bytes()).map_err(|error| InvalidCanonicalName { line_index, error })?;
		
		canonical_search_name.to_fully_qualified_domain_name(self.hosts_default_domain_name(parse_options)).map_err(|error| InvalidCanonicalName { line_index, error })
	}
	
	#[inline(always)]
	fn parse_alias_name(&self, parse_options: &HostsParseOptions, alias_name_field: &str, line_index: usize, alias_index: usize) -> Result<FullyQualifiedDomainName, ParseHostsError>
	{
		use self::ParseHostsError::*;
		
		let alias_search_name = SearchName::parse(alias_name_field.as_bytes()).map_err(|error| InvalidAliasName { line_index, alias_index, error })?;
		alias_search_name.to_fully_qualified_domain_name(self.hosts_default_domain_name(parse_options)).map_err(|error| InvalidAliasName { line_index, alias_index, error })
	}
	
	#[inline(always)]
	fn hosts_default_domain_name<'b: 'a>(&self, parse_options: &'b HostsParseOptions) -> &'b FullyQualifiedDomainName
	{
		parse_options.default_domain_name(&self.default_domain_name)
	}
	
	#[inline(always)]
	fn parse_line_without_comment(line: &str) -> &str
	{
		let line_without_leading_whitespace = line.trim_start();
		
		// Safe, because `#` is always a valid UTF-8 sequence.
		unsafe { from_utf8_unchecked(line_without_leading_whitespace.split_bytes_n(2, b'#').next().unwrap()) }
	}
}
