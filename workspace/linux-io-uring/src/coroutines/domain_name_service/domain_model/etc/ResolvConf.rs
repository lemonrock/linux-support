// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub struct ResolvConf
{
	/// There will always be at least one name server (the default is `127.0.0.1`).
	nameservers: ArrayVec<IpAddr, Self::MaximumNameservers>,
	
	/// Search domains.
	///
	/// Uses a historic cap of 6.
	/// The musl libc limits to 256 bytes (including ASCII NULs).
	///
	/// It is good security practice to avoid search domains that are not locally administered (viz, domains should either be one of the recognized local domains or a domain that is owned and administered by us).
	search_domains: ArrayVec<FullyQualifiedDomainName, Self::MaximumSearchDomains>,
	
	/// A sort list.
	///
	/// If the netmask is missing, it "default to the natural netmask of the net" according to the NetBSD man pages.
	///
	/// Does that mean it defaults to the CIDR mask? No such thing exists for Internet Protocol version 6.
	sort_list: ArrayVec<(Ipv4Addr, Option<NonZeroU8>), Self::MaximumSortList>,
	
	/// Default is 1.
	///
	/// Maximum is 15 (ie a nibble).
	ndots: u8,

	/// Default is 5 seconds.
	///
	/// Maximum is 30 seconds.
	timeout_in_seconds: NonZeroU8,

	/// Default is 2.
	///
	/// Maximum is 5.
	attempts: NonZeroU8,
	
	/// Ignored glibc option `rotate`.
	rotate: bool,
	
	/// Ignored glibc option ` single-request`.
	single_request: bool,
	
	/// Ignored glibc option ` single-request-reopen`.
	single_request_reopen: bool,
	
	/// Ignored glibc option `no-tld-query`.
	no_tld_query: bool,
	
	/// Ignored glibc option `use-vc`.
	///
	/// We always use TCP.
	use_vc: bool,
	
	/// Ignored glibc option `no-reload`.
	no_reload: bool,
	
	/// Ignored glibc option `trust-ad`.
	///
	/// We always set the `AD` bit.
	trust_ad: bool,
	
	/// Ignored glibc option `edns0`.
	edns0: bool,
	
	/// Ignored glibc option `debug`.
	#[deprecated]
	debug: bool,
	
	/// Ignored glibc option `no-check-names`.
	///
	/// We always check labels and names but are quite lenient.
	#[deprecated]
	no_check_names: bool,
	
	/// Ignored glibc option `inet6`.
	#[deprecated]
	inet6: bool,
	
	/// Ignored glibc option `ip6-bytestring`.
	#[deprecated]
	ip6_bytestring: bool,
	
	/// Ignored glibc option `ip6-dotint`.
	#[deprecated]
	ip6_dotint: bool,
}

impl ResolvConf
{
	/// `MAXNS`.
	pub const MaximumNameservers: usize = 3;
	
	/// Maximum number of sort list pairs.
	pub const MaximumSortList: usize = 10;
	
	/// Maximum number of search domains.
	pub const MaximumSearchDomains: usize = 6;
	
	const DefaultNameserver: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
	
	/// New instance.
	///
	/// Ordinarily `default_domain_name` is found using `DefaultDomainNameChoice::uname_host_name()` with a fallback to `FullyQualifiedDomainName::root()`.
	#[inline(always)]
	pub fn new_from_environment(etc_path: &EtcPath, default_domain_name: &FullyQualifiedDomainName) -> Result<Self, ParseEtcResolvConfError>
	{
		use self::ParseEtcResolvConfError::*;
		
		let mut this: ResolvConf = ResolvConf::defaultish(ArrayVec::new(), ArrayVec::new());
		
		this.parse_etc_resolv_conf(etc_path)?;
		
		this.apply_environment_variable_overrides()?;
		
		if this.nameservers.is_empty()
		{
			this.nameservers.push(Self::DefaultNameserver)
		}
		
		if this.search_domains.is_empty()
		{
			this.search_domains.push(default_domain_name.clone());
			
		}
		
		Ok(this)
	}
	
	#[inline(always)]
	pub(crate) fn always_valid_locally_administered_domains(&self) -> HashSet<FullyQualifiedDomainName>
	{
		let mut always_valid_locally_administered_domains = HashSet::with_capacity(self.search_domains.len());
		for search_domain in self.search_domains
		{
			let search_domain: &FullyQualifiedDomainName = search_domain;
			always_valid_locally_administered_domains.insert(search_domain.clone());
		}
		always_valid_locally_administered_domains
	}
	
	#[inline(always)]
	fn number_of_periods(&self) -> u8
	{
		self.ndots
	}
	
	#[inline(always)]
	fn defaultish(nameservers: ArrayVec<IpAddr, Self::MaximumNameservers>, search_domains: ArrayVec<FullyQualifiedDomainName, Self::MaximumSearchDomains>) -> Self
	{
		Self
		{
			nameservers,
			
			search_domains,
			
			sort_list: ArrayVec::new(),
			
			ndots: 1,
			
			timeout_in_seconds: new_non_zero_u8(5),
			
			attempts: new_non_zero_u8(2),
			
			debug: false,
			
			rotate: false,
			
			no_check_names: false,
			
			single_request: false,
			
			single_request_reopen: false,
			
			no_tld_query: false,
			
			use_vc: false,
			
			no_reload: false,
			
			trust_ad: false,
			
			inet6: false,
			
			ip6_bytestring: false,
			
			ip6_dotint: false,
			
			edns0: false
		}
	}
	
	fn parser(etc_path: &EtcPath, mut callback: impl for<'whitespace> FnMut(&'whitespace str, SplitWhitespace<'whitespace>, usize) -> Result<(), ParseEtcResolvConfError>) -> Result<(), ParseEtcResolvConfError>
	{
		use self::ParseEtcResolvConfError::*;
		
		let etc_resolv_conf_file_path = etc_path.resolv_conf();
		if !etc_resolv_conf_file_path.exists()
		{
			return Ok(())
		}
		
		let raw = etc_resolv_conf_file_path.read_raw()?;
		let string: String = String::from_utf8(raw.into_vec())?;
		
		for (line_index, line) in string.split_terminator(b'\n').enumerate()
		{
			if line.is_empty()
			{
				continue
			}
			
			match line.as_bytes().get_unchecked_value_safe(0)
			{
				b'#' | b';' => continue,
				
				_ => (),
			}
			
			let mut fields = line.split_whitespace();
			let keyword = fields.next().unwrap();
			callback(keyword, fields, line_index)?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn push_nameserver(&mut self, nameserver: IpAddr)
	{
		if self.nameservers.contains(&nameserver)
		{
			return
		}
		let _not_an_error_to_have_too_many = self.nameservers.try_push(nameserver);
	}
	
	#[inline(always)]
	fn push_search_domain(&mut self, search_domain: FullyQualifiedDomainName)
	{
		if self.search_domains.contains(&search_domain)
		{
			return
		}
		let _not_an_error_to_have_too_many = self.search_domains.try_push(search_domain);
	}
	
	#[inline(always)]
	fn push_sort_list_pair(&mut self, sort_list_pair: (Ipv4Addr, Option<NonZeroU8>))
	{
		if self.sort_list.contains(&sort_list_pair)
		{
			return
		}
		let _not_an_error_to_have_too_many = self.sort_list.try_push(sort_list_pair);
	}
	
	#[inline(always)]
	fn parse_keyword<'whitespace>(&mut self, keyword: &'whitespace str, mut fields: SplitWhitespace<'whitespace>, line_index: usize) -> Result<(), ParseEtcResolvConfError>
	{
		use self::ParseEtcResolvConfError::*;
		
		match keyword
		{
			"nameserver" =>
			{
				let raw_internet_protocol_address = fields.next().ok_or(MissingNameserverInternetProtocolAddress { line_index })?;
				let nameserver = IpAddr::from_str(raw_internet_protocol_address).map_err(|error| InvalidNameserverInternetProtocolAddress { line_index, error })?;
				self.push_nameserver(nameserver)
			}
			
			"search" => self.parse_search_domains(fields, line_index, true),
			
			"domain" =>
			{
				self.search_domains.clear();
				
				let search_domain_name = fields.next().ok_or(MissingDomainName { line_index } )?;
				let search_domain = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(search_domain_name.as_bytes()).map_err(|error| InvalidDomainName { line_index, error })?;
				self.push_search_domain(search_domain)
			}
			
			"sortlist" =>
			{
				let mut has_at_least_one_entry = false;
				
				for (sort_list_pair_index, sort_list_pair) in fields.enumerate()
				{
					let mut split = sort_list_pair.splitn(2, |character| *character == '/' || *character == '&');
					
					let internet_protocol_address = Ipv4Addr::from_str(split.next().unwrap()).map_err(|error| InvalidSortListInternetProtocolVersion4Address { line_index, sort_list_pair_index, error })?;
					
					let netmask = Self::internet_protocol_version4_mask(line_index, sort_list_pair_index, split.next())?;
					
					self.push_sort_list_pair((internet_protocol_address, netmask))
				}
				
				if !has_at_least_one_entry
				{
					return Err(MissingSortList { line_index })
				}
			}
			
			"options" => self.parse_options(fields, line_index),
			
			_ => (),
		}
		
		Ok(())
	}
	
	fn parse_domain<'whitespace>(keyword: &'whitespace str, mut fields: SplitWhitespace<'whitespace>, line_index: usize, domain: &mut Option<(FullyQualifiedDomainName, bool)>) -> Result<(), ParseEtcResolvConfError>
	{
		const HasDomain: bool = true;
		const HasSearch: bool = false;
		match domain
		{
			Some((_, HasDomain)) => return Ok(()),
			_ => (),
		}
		
		#[inline(always)]
		fn process(mut fields: SplitWhitespace<'whitespace>, line_index: usize, domain: &mut Option<(FullyQualifiedDomainName, bool)>, has: bool) -> Result<(), ParseEtcResolvConfError>
		{
			use self::ParseEtcResolvConfError::*;
			
			let search_domain_name = fields.next().ok_or(MissingDomainName { line_index } )?;
			let search_domain = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(search_domain_name.as_bytes()).map_err(|error| InvalidDomainName { line_index, error })?;
			*domain = Some((search_domain, has));
			Ok(())
		}
		
		match keyword
		{
			"domain" => process(fields, line_index, domain, HasDomain),
			
			"search" => process(fields, line_index, domain, HasSearch),
			
			_ => (),
		}
		
		Ok(())
	}
	
	fn parse_etc_resolv_conf(&mut self, etc_path: &EtcPath) -> Result<(), ParseEtcResolvConfError>
	{
		Self::parser(etc_path, |keyword, fields, line_index| self.parse_keyword(keyword, fields, line_index))
	}
	
	#[inline(always)]
	fn parse_search_domains(&mut self, mut fields: SplitWhitespace, line_index: usize, force_at_least_one_entry: bool) -> Result<(), ParseEtcResolvConfError>
	{
		self.search_domains.clear();
		
		use self::ParseEtcResolvConfError::*;
		
		let mut has_at_least_one_entry = false;
		for (search_domain_index, search_domain_name) in fields.enumerate()
		{
			let search_domain = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(search_domain_name.as_bytes()).map_err(|error| InvalidSearchName { line_index, search_domain_index, error })?;
			self.push_search_domain(search_domain);
			has_at_least_one_entry = true;
		}
		
		if likely!(has_at_least_one_entry)
		{
			Ok(())
		}
		else
		{
			if force_at_least_one_entry
			{
				Err(MissingSearchList { line_index })
			}
			else
			{
				Ok(())
			}
		}
	}
	
	#[inline(always)]
	fn internet_protocol_version4_mask(line_index: usize, sort_list_pair_index: usize, mask: Option<&str>) -> Result<Option<NonZeroU8>, ParseEtcResolvConfError>
	{
		Self::internet_protocol_version_mask::<Ipv4Addr, _>(line_index, sort_list_pair_index, mask, |internet_protocol_mask|
		{
			let octets = internet_protocol_mask.octets();
			Self::octets_to_set_bits(line_index, sort_list_pair_index, &octets[..])
		})
	}
	
	#[inline(always)]
	fn internet_protocol_version_mask<IPA: FromStr<Err=AddrParseError>, F: FnOnce(IPA) -> Result<Option<NonZeroU8>, ParseEtcResolvConfError>>(line_index: usize, sort_list_pair_index: usize, mask: Option<&str>, f: F) -> Result<Option<NonZeroU8>, ParseEtcResolvConfError>
	{
		use self::ParseEtcResolvConfError::*;
		
		match mask
		{
			None => Ok(None),
			
			Some(mask) =>
			{
				let internet_protocol_mask = IPA::from_str(mask).map_err(|error| ParseEtcResolvConfError::InvalidSortListInternetProtocolMask { line_index, sort_list_pair_index, error })?;
				f(internet_protocol_mask)
			}
		}
	}
	
	#[inline(always)]
	fn octets_to_set_bits(line_index: usize, sort_list_pair_index: usize, octets: &[u8]) -> Result<Option<NonZeroU8>, ParseEtcResolvConfError>
	{
		const BitsInAByte: u8 = 8;
		
		let mut set_bits = 0;
		for octet in octets
		{
			let octet = *octet;
			let leading_ones = octet.leading_ones() as u8;
			
			if leading_ones == BitsInAByte
			{
				set_bits += BitsInAByte;
				continue
			}
			
			if leading_ones == 0
			{
				break
			}
			
			let mask = (1 << (BitsInAByte - leading_ones)) - 1;
			
			if unlikely!(octet & mask != 0b0000_0000)
			{
				return Err(ParseEtcResolvConfError::InvalidSortListInternetProtocolMaskSetBits { line_index, sort_list_pair_index })
			}
			set_bits += leading_ones;
			break
		}
		Ok(NonZeroU8::new(set_bits))
	}
	
	#[allow(deprecated)]
	fn parse_options(&mut self, mut fields: SplitWhitespace, line_index: usize) -> Result<(), ParseEtcResolvConfError>
	{
		use self::ParseEtcResolvConfError::*;
		
		#[inline(always)]
		fn option_should_have_not_a_value(field: &mut bool, line_index: usize, option_name: &[u8], option_value: Option<&[u8]>) -> Result<(), ParseEtcResolvConfError>
		{
			if option_value.is_some()
			{
				return Err(OptionShouldNotHaveAValue { line_index, option_name: option_name.to_vec().into_boxed_slice() })
			}
			*field = true;
			Ok(())
		}
		
		#[inline(always)]
		fn option_should_have_not_a_value_inverted(field: &mut bool, line_index: usize, option_name: &[u8], option_value: Option<&[u8]>) -> Result<(), ParseEtcResolvConfError>
		{
			if option_value.is_some()
			{
				return Err(OptionShouldNotHaveAValue { line_index, option_name: option_name.to_vec().into_boxed_slice() })
			}
			*field = false;
			Ok(())
		}
		
		#[inline(always)]
		fn option_should_have_a_value<V>(field: &mut V, line_index: usize, option_name: &[u8], option_value: Option<&[u8]>, maximum: u8, for_zero: impl FnOnce() -> V, convert: impl FnOnce(u8) -> V) -> Result<(), ParseEtcResolvConfError>
		{
			let option_value = option_value.ok_or(OptionShouldHaveAValue { line_index, option_name: option_name.to_vec().into_boxed_slice() })?;
			let value = u64::parse_decimal_number(option_value).map_err(|error| OptionValueNotADecimalNumber { line_index, option_name: option_name.to_vec().into_boxed_slice(), error })?;
			
			let field_value = if value == 0
			{
				for_zero()
			}
			else if value < (maximum as u64)
			{
				convert(value as u8)
			}
			else
			{
				convert(maximum)
			};
			
			*field = field_value;
			
			Ok(())
		}
		
		for option_definition in fields
		{
			let mut split = option_definition.split_bytes_n(2, b':');
			let option_name = split.next().unwrap();
			let option_value = split.next();
			
			match option_name
			{
				b"ndots" => option_should_have_a_value(&mut self.ndots, line_index, option_name, option_value, 15, || 0, |value| value),
				
				b"timeout" => option_should_have_a_value(&mut self.timeout, line_index, option_name, option_value, 30, || new_non_zero_u8(1), |value| new_non_zero_u8(value)),
				
				b"attempts" => option_should_have_a_value(&mut self.attempts, line_index, option_name, option_value, 5, || new_non_zero_u8(1), |value| new_non_zero_u8(value)),
				
				b"rotate" => option_should_have_not_a_value(&mut self.rotate, line_index, option_name, option_value),
				
				b"debug" => option_should_have_not_a_value(&mut self.debug, line_index, option_name, option_value),
				b"no-check-names" => option_should_have_not_a_value(&mut self.no_check_names, line_index, option_name, option_value),
				b"inet6" => option_should_have_not_a_value(&mut self.inet6, line_index, option_name, option_value),
				b"ip6-bytestring" => option_should_have_not_a_value(&mut self.ip6_bytestring, line_index, option_name, option_value),
				b"ip6-dotint" => option_should_have_not_a_value(&mut self.ip6_dotint, line_index, option_name, option_value),
				b"no-ip6-dotint" => option_should_have_not_a_value_inverted(&mut self.ip6_dotint, line_index, option_name, option_value),
				
				b"edns0" => option_should_have_not_a_value(&mut self.edns0, line_index, option_name, option_value),
				
				b"single-request" => option_should_have_not_a_value(&mut self.single_request, line_index, option_name, option_value),
				
				b"single-request-reopen" => option_should_have_not_a_value(&mut self.single_request_reopen, line_index, option_name, option_value),
				
				b"no-tld-query" => option_should_have_not_a_value(&mut self.no_tld_query, line_index, option_name, option_value),
				b"no_tld_query" => option_should_have_not_a_value(&mut self.no_tld_query, line_index, option_name, option_value),
				
				b"use-vc" => option_should_have_not_a_value(&mut self.use_vc, line_index, option_name, option_value),
				
				b"no-reload" => option_should_have_not_a_value(&mut self.no_reload, line_index, option_name, option_value),
				
				b"trust-ad" => option_should_have_not_a_value(&mut self.trust_ad, line_index, option_name, option_value),
				
				_ => Ok(()),
			}?;
		}
		
		Ok(())
	}
	
	const LOCALDOMAIN: &'static str = "LOCALDOMAIN";
	
	const RES_OPTIONS: &'static str = "RES_OPTIONS";
	
	/// Uses the values of `LOCALDOMAIN` and `RES_OPTIONS` to override values set by parsing `/etc/resolv.conf`.
	fn apply_environment_variable_overrides(&mut self) -> Result<(), ParseEtcResolvConfError>
	{
		use self::ParseEtcResolvConfError::EnvironmentVariableIsNotUtf8;
		
		if let Some(value) = var_os(Self::LOCALDOMAIN)
		{
			let local_domain = value.into_string().map_err(|_| Err(EnvironmentVariableIsNotUtf8 { name: Self::LOCALDOMAIN }))?;
			self.parse_search_domains(local_domain.split_whitespace(), 0, false)
		}
		
		if let Some(value) = var_os("RES_OPTIONS")
		{
			let options = value.into_string().map_err(|_| Err(EnvironmentVariableIsNotUtf8 { name: Self::RES_OPTIONS }))?;
			self.parse_options(options.split_whitespace(), 0)
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn as_default_domain_name_choice(etc_path: &EtcPath) -> FullyQualifiedDomainName
	{
		if let Some(value) = var_os(Self::LOCALDOMAIN)
		{
			if let Ok(local_domain) = value.into_string()
			{
				let first = local_domain.split_whitespace().next().unwrap();
				if let Ok(domain_name) = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(first.as_bytes())
				{
					return domain_name
				}
			}
		}
		
		let mut domain = None;
		let result = ResolvConf::parser(etc_path, |keyword, fields, line_index| ResolvConf::parse_domain(keyword, fields, line_index, &mut domain));
		if result.is_ok()
		{
			if let Some((domain_name, _has)) = domain
			{
				return domain_name
			}
		}
		
		/// This uses the same logic as glibc and as documented in man 5 resolv.conf.
		if let Ok(Some(domain_name)) = DefaultDomainNameChoice::uname_host_name()
		{
			return domain_name
		}
		
		FullyQualifiedDomainName::root()
	}
}
