// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Default domain name choice.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum DefaultDomainNameChoice
{
	/// Uses the logic for determing the search list in `/etc/resolv.conf`.
	///
	/// Order in preference (first match wins):-
	///
	/// * Prefers the first search domain of the environment variable `LOCALDOMAIN`;
	/// * Then prefers the first occurrence of `domain` in `/etc/resolv.conf`;
	/// * Then prefers the first search domain of the first occurrence of `search` in `/etc/resolv.conf`.
	/// * Then uses `LinuxKernelHostName`.
	/// * Then uses root (`.`).
	EtcResolvConf,
	
	/// Just use `.`.
	Root,
	
	/// `/proc/sys/kernel/hostname`; should be the same as retrieved via `gethostname()` and `uname()`.
	///
	/// Maximum of 64 characters.
	LinuxKernelHostName,
	
	/// `/proc/sys/kernel/domainname`; should be the same as retrieved via `gethostname()` and `uname()`.
	///
	/// Historically this was used for the NIS and YP (Yellow Pages) domain name.
	///
	/// Maximum of 64 characters.
	LinuxKernelDomainName,
	
	/// Should be the same as that retrieved via `gethostname()` and `uname()` but does not require `/proc`.
	UNameHostName,
	
	/// Should be the same at that retrieved via `getdomainname()` and `uname()` but does not require `/proc`.
	///
	/// Historically this was used for the NIS and YP (Yellow Pages) domain name.
	UNameDomainName,
	
	/// Can contain comment lines.
	///
	/// Often absent.
	///
	/// Maximum of 64 characters.
	EtcDnsdomainname,
	
	/// Call `dnsdomainname` on the `PATH`.
	DnsdomainnameBinary,
	
	/// `gethostbyname()`.
	///
	/// This is a bad idea as it can make a DNS query insecurely via libc.
	///
	/// Implementation is based on the BusyBox binary `dnsdomainname` (which is just `hostname -d`).
	///
	/// In the BusyBox binary, the major steps are:-
	///
	/// * Retrieves the uts nodename using `uname()`;
	/// * Looks up the uts nodename using the obsolete `gethostbyname()`, which invokes the libc's DNS machinery (in musl libc, this calls `__lookup_name()`).
	/// * In musl, this is hardcoded to then check `name_from_hosts()` which looks for an alias that matches a canonical name in `/etc/hosts` before going off to run a DNS query.
	///
	/// If the glibc library is used, the `HOSTALIASES` environment variable will be honoured (see <https://linux.die.net/man/7/hostname>).
	GetHostByName,
	
	/// Specific.
	Fixed(FullyQualifiedDomainName),

	/// Use the first occurrence of the `trim` keyword in `/etc/host.conf` (with any values in the environment variables taking preference).
	///
	/// This file is rare and the setting is rarely used.
	EtcHostConfTrim,
}

impl Default for DefaultDomainNameChoice
{
	#[inline(always)]
	fn default() -> Self
	{
		DefaultDomainNameChoice::EtcResolvConf
	}
}

impl DefaultDomainNameChoice
{
	/// As a last result, the domain name will be root (`.`).
	///
	/// Errors are swallowed.
	#[inline(always)]
	pub fn resolve_until_one_works<'a>(choices: impl Iterator<Item=&'a Self> + 'a, proc_path: &ProcPath, etc_path: &EtcPath) -> FullyQualifiedDomainName
	{
		for choice in choices
		{
			if let Ok(Some(domain_name)) = choice.resolve(proc_path, etc_path)
			{
				return domain_name
			}
		}
		
		FullyQualifiedDomainName::root()
	}
	
	/// Resolves the default domain name.
	///
	/// Any recoverable errors in processing are swallowed and `None` is returned.
	pub fn resolve(&self, proc_path: &ProcPath, etc_path: &EtcPath) -> Result<Option<FullyQualifiedDomainName>, EfficientCaseFoldedNameParseError>
	{
		use self::DefaultDomainNameChoice::*;
		
		match self
		{
			EtcResolvConf => Self::resolv_conf(etc_path),
			
			Root => Ok(Some(FullyQualifiedDomainName::root())),
			
			LinuxKernelHostName => Self::linux_kernel_host_name(proc_path),
			
			LinuxKernelDomainName => Self::linux_kernel_domain_name(proc_path),
			
			UNameHostName => Self::uname_host_name(),
			
			UNameDomainName => Self::uname_domain_name(),
			
			EtcDnsdomainname => Self::etc_dnsdomainname(etc_path),
			
			GetHostByName => Self::get_host_by_name(),
			
			Fixed(ref name) => Ok(Some(name.clone())),
			
			EtcHostConfTrim => Self::etc_host_conf_trim(etc_path),
		}
	}
	
	#[inline(always)]
	fn linux_kernel_host_name(proc_path: &ProcPath) -> Result<Option<FullyQualifiedDomainName>, EfficientCaseFoldedNameParseError>
	{
		match linux_support::linux_kernel_version::LinuxKernelHostName::new(proc_path)
		{
			Err(_) => Ok(None),
			
			Ok(None) => Ok(None),
			
			Ok(Some(linux_kernel_host_name)) =>
			{
				let name = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(&linux_kernel_host_name[..])?;
				Ok(name.parent())
			}
		}
	}
	
	#[inline(always)]
	fn linux_kernel_domain_name(proc_path: &ProcPath) -> Result<Option<FullyQualifiedDomainName>, EfficientCaseFoldedNameParseError>
	{
		match linux_support::linux_kernel_version::LinuxKernelDomainName::new(proc_path)
		{
			Err(_) => Ok(None),
			
			Ok(None) => Ok(None),
			
			Ok(Some(linux_kernel_domain_name)) =>
			{
				let name = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(&linux_kernel_domain_name[..])?;
				Ok(Some(name))
			}
		}
	}
	
	#[inline(always)]
	fn uname_host_name() -> Result<Option<FullyQualifiedDomainName>, EfficientCaseFoldedNameParseError>
	{
		DefaultHostNameChoice::use_uname(|buffer|
		{
			let nodename: [u8; DefaultHostNameChoice::Size] = unsafe { transmute(buffer.nodename) };
			let length = memchr(0x00, &nodename[..]).unwrap_or(Self::Size);
			
			let valid = &nodename[.. length];
			let possibly_fully_qualified_host_name = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(valid)?;
			Ok(possibly_fully_qualified_host_name.parent())
		})
	}
	
	#[inline(always)]
	fn uname_domain_name() -> Result<Option<FullyQualifiedDomainName>, EfficientCaseFoldedNameParseError>
	{
		DefaultHostNameChoice::use_uname(|buffer|
		{
			let domainname: [u8; DefaultHostNameChoice::Size] = unsafe { transmute(buffer.domainname) };
			let length = memchr(0x00, &domainname[..]).unwrap_or(Self::Size);
			
			let valid = &domainname[.. length];
			let possibly_fully_qualified_host_name = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(valid)?;
			Ok(Some(possibly_fully_qualified_host_name))
		})
	}
	
	#[inline(always)]
	fn etc_dnsdomainname(etc_path: &EtcPath) -> Result<Option<FullyQualifiedDomainName>, EfficientCaseFoldedNameParseError>
	{
		let file_path = etc_path.hostname();
		if unlikely!(!file_path.exists())
		{
			return Ok(None)
		}
		
		if let Ok(result) = file_path.read_raw()
		{
			for line in result.split_bytes(b'\n')
			{
				if line.is_empty()
				{
					continue
				}
				if line.get_unchecked_value_safe(0) == b'#'
				{
					continue
				}
				return Ok(Some(FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(line)?))
			}
		}
		
		Ok(None)
	}
	
	/// This uses the same logic as the `hostname` binary.
	#[inline(always)]
	fn get_host_by_name() -> Result<Option<FullyQualifiedDomainName>, EfficientCaseFoldedNameParseError>
	{
		let nodename = Self::use_uname(|buffer| buffer.nodename);
		if nodename.get_unchecked_value_safe(0) = 0x00
		{
			return Ok(None)
		}
		
		let result: *mut hostent = unsafe { gethostbyname(nodename.as_ptr()) };
		if result.is_null()
		{
			return Ok(None)
		}
		
		let host_name = unsafe
		{
			let host = & * result;
			CStr::from_ptr(host.h_name)
		};
		
		Ok(Some(FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(host_name.to_bytes())?))
	}
	
	#[inline(always)]
	fn resolv_conf(etc_path: &EtcPath) -> Result<Option<FullyQualifiedDomainName>, EfficientCaseFoldedNameParseError>
	{
		Ok(Some(ResolvConf::as_default_domain_name_choice(etc_path)))
	}
	
	#[inline(always)]
	fn etc_host_conf_trim(etc_path: &EtcPath) -> Result<Option<FullyQualifiedDomainName>, EfficientCaseFoldedNameParseError>
	{
		if let Ok(host_conf) = HostConf::from_environment()
		{
			if !host_conf.trim_domains.is_empty()
			{
				return Ok(Some(host_conf.trim_domains.get_unchecked_safe(0).clone()))
			}
		}
		
		Ok(None)
	}
}
