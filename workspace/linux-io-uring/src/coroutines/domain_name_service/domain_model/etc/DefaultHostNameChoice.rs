// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DefaultHostNameChoice
{
	/// `/proc/sys/kernel/hostname`; should be the same as that retrieved via `gethostname()` and `uname()`.
	///
	/// Maximum of 64 characters.
	LinuxKernelHostName,
	
	/// Should be the as that retrieved via `gethostname()` and `uname()` but does not require `/proc`.
	UNameHostName,
	
	/// Can contain comment lines (see <https://www.freedesktop.org/software/systemd/man/hostname.html>)!
	///
	/// Maximum of 64 characters.
	EtcHostname,
	
	/// `gethostbyname()`.
	///
	/// This is a bad idea as it can make a DNS query insecurely via libc.
	///
	/// Implementation is based on the BusyBox binary `hostname`.
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
	Fixed(HostNameLabel),
}

impl DefaultHostNameChoice
{
	const Size: usize = 65;
	
	/// As a last result, the host name will be `localhost`.
	///
	/// Errors are swallowed.
	#[inline(always)]
	pub fn resolve_until_one_works(choices: &[Self], proc_path: &ProcPath, etc_path: &EtcPath) -> HostNameLabel
	{
		for choice in choices
		{
			if let Ok(Some(host_name_label)) = choice.resolve(proc_path, etc_path)
			{
				return host_name_label
			}
		}
		
		HostNameLabel::localhost()
	}
	
	/// Resolves the default host name.
	///
	/// Any recoverable errors in processing are swallowed and `None` is returned.
	pub fn resolve(&self, proc_path: &ProcPath, etc_path: &EtcPath) -> Result<Option<HostNameLabel>, EfficientCaseFoldedNameParseError>
	{
		use self::DefaultHostNameChoice::*;
		
		match self
		{
			LinuxKernelHostName => Self::linux_kernel_host_name(),
			
			UNameHostName => Self::uname_host_name(),
			
			EtcHostname => Self::etc_hostname(etc_path),
			
			GetHostByName => Self::get_host_by_name(),
			
			Fixed(ref host_name_label) => Ok(Some(host_name_label.clone())),
		}
	}
	
	#[inline(always)]
	fn linux_kernel_host_name(proc_path: &ProcPath) -> Result<Option<HostNameLabel>, EfficientCaseFoldedNameParseError>
	{
		match linux_support::linux_kernel_version::LinuxKernelHostName::new(proc_path)
		{
			Err(_) => Ok(None),
			
			Ok(None) => Ok(None),
			
			Ok(Some(linux_kernel_host_name)) =>
			{
				let name = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(&linux_kernel_host_name[..])?;
				Ok(name.host_name())
			}
		}
	}
	
	#[inline(always)]
	fn uname_host_name() -> Result<Option<HostNameLabel>, EfficientCaseFoldedNameParseError>
	{
		Self::use_uname(|buffer|
		{
			let nodename: [u8; Self::Size] = unsafe { transmute(buffer.nodename) };
			let length = memchr(0x00, &nodename[..]).unwrap_or(Self::Size);
			
			let valid = &nodename[.. length];
			let possibly_fully_qualified_host_name = FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(valid)?;
			Ok(possibly_fully_qualified_host_name.host_name())
		})
	}
	
	#[inline(always)]
	fn etc_hostname(etc_path: &EtcPath) -> Result<Option<HostNameLabel>, EfficientCaseFoldedNameParseError>
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
				return Ok(FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(line)?.host_name())
			}
		}
		
		Ok(None)
	}
	
	/// This uses the same logic as the `hostname` binary.
	#[inline(always)]
	fn get_host_by_name() -> Result<Option<HostNameLabel>, EfficientCaseFoldedNameParseError>
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
		
		Ok(FullyQualifiedDomainName::from_byte_string_ending_with_optional_trailing_period(host_name.to_bytes())?.host_name())
	}
	
	#[inline(always)]
	fn use_uname<R, F: FnOnce(utsname) -> R>(user: F) -> R
	{
		let mut buffer: utsname = unsafe_uninitialized();
		let result = unsafe { uname(&mut buffer) };
		if likely!(result == 0)
		{
			user(buffer)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EFAULT => unreachable_code(format_args!("Invald buffer")),
				
				unexpected @ _ => unreachable_code(format_args!("Unexpected error `{}` from `uname()`", result))
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result `{}` from `uname()`", result))
		}
	}
}
