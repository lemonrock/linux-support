// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub struct HostConf
{
	/// Order.
	///
	/// Typical values might be `bind`, `hosts` and `nis`.
	order: IndexSet<Box<[u8]>>,
	
	/// Ignored glibc option `multi`; support for more than one address in a hosts file is always enabled.
	multi: bool,
	
	/// Ignored glibc option `reorder`.
	reorder: bool,
	
	/// Ignored option `nospoof`.
	///
	/// Not supported by glibc.
	#[deprecated]
	nospoof: bool,
	
	/// Ignored option `alert`.
	///
	/// Not supported by glibc.
	#[deprecated]
	alert: bool,
	
	/// Search domains.
	///
	/// Uses a historic cap of 4 (`TRIMDOMAINS_MAX`).
	trim_domains: ArrayVec<FullyQualifiedDomainName, HostConf::MaximumTrimDomains>,
}

impl Default for HostConf
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			order: IndexSet::empty(),
		
			multi: false,
			
			reorder: false,
		
			nospoof: false,
		
			alert: false,
		
			trim_domains: ArrayVec::new(),
		}
	}
}

impl HostConf
{
	/// `TRIMDOMAINS_MAX`.
	pub const MaximumTrimDomains: usize = 4;
	
	#[inline(always)]
	pub fn from_environment() -> Result<Self, ParseEtcHostConfError>
	{
		let mut this = Self::default();
		
		let host_conf_file_path = if let Some(file_path) = var_os("RESOLV_HOST_CONF")
		{
			PathBuf::from(file_path)
		}
		else
		{
			etc_path.host_conf()
		};
		this.parse_etc_host_conf(&host_conf_file_path)?;
		
		Self::environment_variable_boolean_option("RESOLV_MULTI", &mut this.multi)?;
		
		Self::environment_variable_boolean_option("RESOLV_REORDER", &mut this.reorder)?;
		
		this.environment_variable_trim("RESOLV_ADD_TRIM_DOMAINS", false)?;
		
		this.environment_variable_trim("RESOLV_OVERRIDE_TRIM_DOMAINS", true)?;
		
		Ok(this)
	}
	
	#[inline(always)]
	fn environment_variable_trim(&mut self, name: &'static str, clear: bool) -> Result<(), ParseEtcHostConfError>
	{
		use self::ParseEtcHostConfError::*;
		
		if let Some(value) = var_os(name)
		{
			if clear
			{
				self.trim_domains.clear()
			}
			
			let bytes = value.as_bytes();
			if !bytes.is_empty()
			{
				let mut fields = bytes.split(|byte| byte.is_ascii_whitespace());
				
				let trim_domain_name = fields.next().ok_or(InvalidEnvironmentVariable { name, value } )?;
				let trim_domain = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(search_domain_name.as_bytes()).map_err(|error| EnvironmentVariableInvalidTrimName { name, error })?;
				this.push_trim_domain(trim_domain);
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn environment_variable_boolean_option(name: &'static str, field: &mut bool) -> Result<(), ParseEtcHostConfError>
	{
		if let Some(value) = var_os(name)
		{
			let boolean_value = match multi_option.as_bytes()
			{
				b"on" => true,
				
				b"off" => false,
				
				_ => return Err(ParseEtcHostConfError::InvalidEnvironmentVariable { name, value })
			};
			*field = boolean_value
		}
		
		Ok(())
	}
	
	/// Parse.
	pub fn parse_etc_host_conf(&mut self, host_conf_file_path: &Path) -> Result<(), ParseEtcHostConfError>
	{
		let mut duplicated_keywords = HashSet::with_capacity(5);
		
		if !host_conf_file_path.exists()
		{
			return Ok(())
		}
		
		let raw = host_conf_file_path.read_raw()?;
		let string: String = String::from_utf8(raw.into_vec())?;
		
		for (line_index, line) in string.split_terminator(b'\n').enumerate()
		{
			let line = line.trim();
			
			let line = line.split('#').next().unwrap().trim();
			
			if line.is_empty()
			{
				continue
			}
			
			let mut fields = line.split_whitespace();
			let keyword = fields.next().unwrap();
			
			if duplicated_keywords.insert(keyword.to_string())
			{
				self.parse_keyword(keyword, fields, line_index)?;
			}
			else
			{
				return Err(ParseEtcHostConfError::DuplicateKeyword { keyword, line_index })
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn push_trim_domain(&mut self, trim_domain: FullyQualifiedDomainName)
	{
		if self.trim_domains.contains(&trim_domain)
		{
			return
		}
		let _not_an_error_to_have_too_many = self.trim_domains.try_push(trim_domain);
	}
	
	#[inline(always)]
	fn parse_keyword<'whitespace>(&mut self, keyword: &'whitespace str, mut fields: SplitWhitespace<'whitespace>, line_index: usize) -> Result<(), ParseEtcHostConfError>
	{
		use self::ParseEtcHostConfError::*;
		
		#[inline(always)]
		fn boolean_value<'whitespace>(fields: &mut SplitWhitespace<'whitespace>, line_index: usize, field: &mut bool, keyword: &'static str) -> Result<(), ParseEtcHostConfError>
		{
			let value = match fields.next().ok_or(MissingValue { keyword, line_index })?
			{
				"on" => true,
				
				"off" => false,
				
				value @ _ => return Err(ValueNotBoolean { keyword, line_index, value: value.to_string() })
			};
			
			*field = value;
			
			Ok(())
		}
		
		match keyword
		{
			"order" =>
			{
				let mut has_at_least_one = false;
				for field in fields
				{
					self.order.insert(field);
					has_at_least_one = true;
				}
				
				if unlikely!(!has_at_least_one)
				{
					return Err(MissingValue { keyword: "order", line_index })
				}
			}
			
			"multi" => boolean_value(&mut field, line_index, &mut self.multi, "multi")?,
			
			"reorder" => boolean_value(&mut field, line_index, &mut self.reorder, "reorder")?,
			
			"nospoof" => boolean_value(&mut field, line_index, &mut self.nospoof, "nospoof")?,
			
			"alert" => boolean_value(&mut field, line_index, &mut self.alert, "alert")?,
			
			"trim" =>
			{
				let trim_domain_name = fields.next().ok_or(MissingValue { keyword: "trim", line_index } )?;
				let trim_domain = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(search_domain_name.as_bytes()).map_err(|error| InvalidTrimName { line_index, error })?;
				self.push_trim_domain(trim_domain)
			}
			
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
}
