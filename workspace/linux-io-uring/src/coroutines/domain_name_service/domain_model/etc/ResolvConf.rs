// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub struct ResolvConf
{
	/// There will always be at least one name server (the default is `127.0.0.1`).
	nameservers: ArrayVec<[IpAddr; Self::MaximumNameservers]>,
	
	/// Search domains.
	///
	/// Uses a historic cap of 6.
	/// The musl libc limits to 256 bytes (including ASCII NULs).
	search_domains: ArrayVec<[EfficientCaseFoldedName; Self::MaximumSearchDomains]>,
	
	/// A sort list.
	sort_list: ArrayVec<[Either<(Ipv4Addr, Option<NonZeroU8>), (Ipv6Addr, Option<NonZeroU8>)>; Self::MaximumSortList]>,
	
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
	
	/// Ignored glibc option `debug`.
	debug: bool,
	
	/// Ignored glibc option `rotate`.
	rotate: bool,
	
	/// Ignored glibc option `no-check-names`.
	///
	/// We always check labels and names but are quite lenient.
	no_check_names: bool,
	
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
	
	/// Ignored glibc option `inet6`.
	#[deprecated]
	inet6: bool,
	
	/// Ignored glibc option `ip6-bytestring`.
	#[deprecated]
	ip6_bytestring: bool,
	
	/// Ignored glibc option `ip6-dotint`.
	#[deprecated]
	ip6_dotint: bool,
	
	/// Ignored glibc option `no-ip6-dotint`.
	#[deprecated]
	no_ip6_dotint: bool,
	
	/// Ignored glibc option `edns0`.
	#[deprecated]
	edns0: bool,
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
	#[inline(always)]
	pub fn new_from_environment(etc_path: &EtcPath) -> Result<Self, ParseEtcResolvConfError>
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
			this.search_domains.push(Self::default_search_domain()?);
			
		}
		
		Ok(this)
	}
	
	/// This uses the musl libc logic: "queries with fewer dots than the ndots configuration variable are processed with search first then tried literally (just like glibc), but those with at least as many dots as ndots are only tried in the global namespace (never falling back to search, which glibc would do if the name is not found in the global DNS namespace).
	/// This difference comes from a consistency requirement not to return different results subject to transient failures or to global DNS namespace changes outside of one’s control (addition of new TLDs)".
	pub fn search_list(&self, possibly_relative_name: &[u8]) -> Result<Vec<EfficientCaseFoldedName>, EfficientCaseFoldedNameParseError>
	{
		if possibly_relative_name.is_empty()
		{
			return Ok
			(
				vec!
				[
					EfficientCaseFoldedName::root()
				]
			)
		}
		
		let as_is = EfficientCaseFoldedName::from_byte_string_ending_with_optional_trailing_period(possibly_relative_name)?;
		
		let ends_with_a_period = possibly_relative_name.get_unchecked_value_safe(possibly_relative_name.len() - 1) == b'.';
		
		if ends_with_a_period
		{
			return Ok(vec![as_is])
		}
		
		let query_number_of_periods = possibly_relative_name.split_bytes(b'.').count() - 1;
		
		if query_number_of_periods < (self.ndots as usize)
		{
			let mut search_list = Vec::with_capacity(1 + self.search_domains.len());
			for search_domain in self.search_domains
			{
				search_list.push(search_domain.prepend_relative_name(&as_is));
			}
			search_list.push(as_is);
			Ok(search_list)
		}
		else
		{
			Ok(vec![as_is])
		}
	}
	
	#[inline(always)]
	fn defaultish(nameservers: ArrayVec<[IpAddr; Self::MaximumNameservers]>, search_domains: ArrayVec<[EfficientCaseFoldedName; Self::MaximumSearchDomains]>) -> Self
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
			
			no_ip6_dotint: false,
			
			edns0: false
		}
	}
	
	/// This uses the same logic as glibc and as documented in man 5 resolv.conf.
	#[inline(always)]
	fn default_domain_name() -> Result<EfficientCaseFoldedName, EfficientCaseFoldedNameParseError>
	{
		Ok(DefaultDomainNameChoice::uname_host_name()?.unwrap_or_else(EfficientCaseFoldedName::root))
	}
	
	fn parser(etc_path: &EtcPath, mut callback: impl for<'a> FnMut(&'a str, SplitWhitespace<'a>, usize) -> Result<(), ParseEtcResolvConfError>) -> Result<(), ParseEtcResolvConfError>
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
	fn parse_keyword<'a>(&mut self, keyword: &'a str, mut fields: SplitWhitespace<'a>, line_index: usize) -> Result<(), ParseEtcResolvConfError>
	{
		use self::ParseEtcResolvConfError::*;
		
		match keyword
		{
			"nameserver" =>
			{
				let raw_internet_protocol_address = fields.next().ok_or(MissingNameserverInternetProtocolAddress { line_index })?;
				let nameserver = IpAddr::from_str(raw_internet_protocol_address).map_err(|error| InvalidNameserverInternetProtocolAddress { line_index, error })?;
				let _not_an_error_to_have_more_than_3 = self.nameservers.try_push(nameserver);
			}
			
			"search" => self.parse_search_domains(fields, line_index, true),
			
			"domain" =>
			{
				let search_domain_name = fields.next().ok_or(MissingDomainName { line_index } )?;
				let search_domain = EfficientCaseFoldedName::from_byte_string_ending_with_optional_trailing_period(search_domain_name.as_bytes()).map_err(|error| InvalidDomainName { line_index, error })?;
				let _not_an_error_to_have_too_many = self.search_domains.try_push(search_domain);
			}
			
			"sortlist" =>
			{
				let mut has_at_least_one_entry = false;
				
				for (sort_list_pair_index, sort_list_pair) in fields.enumerate()
				{
					let mut split = sort_list_pair.splitn(2, '/');
					let internet_protocol_address = IpAddr::from_str(split.next().unwrap()).map_err(|error| InvalidSortListInternetProtocolAddress { line_index, sort_list_pair_index, error })?;
					
					let mask = split.next();
					use self::IpAddr::*;
					let sort_list_pair = match internet_protocol_address
					{
						V4(v4) => Left((v4, Self::internet_protocol_version4_mask(line_index, sort_list_pair_index, mask)?)),
						
						V6(v6) => Right((v6, Self::internet_protocol_version6_mask(line_index, sort_list_pair_index, mask)?)),
					};
					let _not_an_error_to_have_too_many = self.sort_list.try_push(sort_list_pair);
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
	
	fn parse_domain<'a>(keyword: &'a str, mut fields: SplitWhitespace<'a>, line_index: usize, domain: &mut Option<(EfficientCaseFoldedName, bool)>) -> Result<(), ParseEtcResolvConfError>
	{
		const HasDomain: bool = true;
		const HasSearch: bool = false;
		match domain
		{
			Some((_, HasDomain)) => return Ok(()),
			_ => (),
		}
		
		#[inline(always)]
		fn process(mut fields: SplitWhitespace<'a>, line_index: usize, domain: &mut Option<(EfficientCaseFoldedName, bool)>, has: bool) -> Result<(), ParseEtcResolvConfError>
		{
			use self::ParseEtcResolvConfError::*;
			
			let search_domain_name = fields.next().ok_or(MissingDomainName { line_index } )?;
			let search_domain = EfficientCaseFoldedName::from_byte_string_ending_with_optional_trailing_period(search_domain_name.as_bytes()).map_err(|error| InvalidDomainName { line_index, error })?;
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
		use self::ParseEtcResolvConfError::*;
		
		let mut has_at_least_one_entry = false;
		for (search_domain_index, search_domain_name) in fields.enumerate()
		{
			let search_domain = EfficientCaseFoldedName::from_byte_string_ending_with_optional_trailing_period(search_domain_name.as_bytes()).map_err(|error| InvalidSearchName { line_index, search_domain_index, error })?;
			let _not_an_error_to_have_too_many = self.search_domains.try_push(search_domain);
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
	fn internet_protocol_version6_mask(line_index: usize, sort_list_pair_index: usize, mask: Option<&str>) -> Result<Option<NonZeroU8>, ParseEtcResolvConfError>
	{
		Self::internet_protocol_version_mask::<Ipv6Addr, _>(line_index, sort_list_pair_index, mask, |internet_protocol_mask|
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
				b"debug" => option_should_have_not_a_value(&mut self.debug, line_index, option_name, option_value),
				
				b"ndots" => option_should_have_a_value(&mut self.ndots, line_index, option_name, option_value, 15, || 0, |value| value),
				
				b"timeout" => option_should_have_a_value(&mut self.timeout, line_index, option_name, option_value, 30, || new_non_zero_u8(1), |value| new_non_zero_u8(value)),
				
				b"attempts" => option_should_have_a_value(&mut self.attempts, line_index, option_name, option_value, 5, || new_non_zero_u8(1), |value| new_non_zero_u8(value)),
				
				b"rotate" => option_should_have_not_a_value(&mut self.rotate, line_index, option_name, option_value),
				
				b"no-check-names" => option_should_have_not_a_value(&mut self.no_check_names, line_index, option_name, option_value),
				
				b"inet6" => option_should_have_not_a_value(&mut self.inet6, line_index, option_name, option_value),
				
				b"ip6-bytestring" => option_should_have_not_a_value(&mut self.ip6_bytestring, line_index, option_name, option_value),
				
				b"ip6-dotint" => option_should_have_not_a_value(&mut self.ip6_dotint, line_index, option_name, option_value),
				
				b"no-ip6-dotint" => option_should_have_not_a_value(&mut self.no_ip6_dotint, line_index, option_name, option_value),
				
				b"edns0" => option_should_have_not_a_value(&mut self.edns0, line_index, option_name, option_value),
				
				b"single-request" => option_should_have_not_a_value(&mut self.single_request, line_index, option_name, option_value),
				
				b"single-request-reopen" => option_should_have_not_a_value(&mut self.single_request_reopen, line_index, option_name, option_value),
				
				b"no-tld-query" => option_should_have_not_a_value(&mut self.no_tld_query, line_index, option_name, option_value),
				
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
			self.search_domains.clear();
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
	fn as_default_domain_name_choice(etc_path: &EtcPath) -> EfficientCaseFoldedName
	{
		use self::ParseEtcResolvConfError::EnvironmentVariableIsNotUtf8;
		
		if let Some(value) = var_os(Self::LOCALDOMAIN)
		{
			if let Ok(local_domain) = value.into_string()
			{
				let first = local_domain.split_whitespace().next().unwrap();
				if let Ok(domain_name) = EfficientCaseFoldedName::from_byte_string_ending_with_optional_trailing_period(first.as_bytes())
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
		
		Self::default_domain_name().ok_or_else(EfficientCaseFoldedName::root)
	}
}
