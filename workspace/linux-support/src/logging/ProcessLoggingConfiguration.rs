// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Logging configuration.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessLoggingConfiguration
{
	/// Used as the default value for third-party libraries that use the libc syslog or `FILE*` interface.
	///
	/// Defaults to `auth`.
	#[serde(default)] pub libc_syslog_facility: KnownFacility,
	
	/// Used as the default level for third-party libraries that use the libc syslog interface.
	///
	/// Defaults to `debug` for debug builds and `warning` for production builds.
	#[serde(default)] pub libc_syslog_level: Severity,
	
	/// Used as the default level for third-party libraries that use writes to standard out via `FILE*` pointers.
	///
	/// Defaults to `debug` for debug builds and `warning` for production builds.
	#[serde(default)] pub redirect_FILE_standard_out: Severity,
	
	/// Used as the default level for third-party libraries that use writes to standard error via `FILE*` pointers.
	///
	/// Defaults to `debug` for debug builds and `warning` for production builds.
	#[serde(default)] pub redirect_FILE_standard_error: Severity,

	/// Size of per-thread logging buffer.
	#[serde(default = "ProcessLoggingConfiguration::logging_buffer_size_default")] pub logging_buffer_size: NonZeroUsize,
	
	/// Private Enterprise Numbers (PEN).
	///
	/// See <https://www.iana.org/assignments/enterprise-numbers/enterprise-numbers> for the full list.
	///
	/// Used for RFC 5424 syslog.
	///
	/// Defaults to `Reserved` (0).
	#[serde(default)] pub private_enterprise_number: PrivateEnterpriseNumber,
}

impl Default for ProcessLoggingConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			libc_syslog_facility: Default::default(),
			libc_syslog_level: Default::default(),
			redirect_FILE_standard_out: Default::default(),
			redirect_FILE_standard_error: Default::default(),
			logging_buffer_size: ProcessLoggingConfiguration::logging_buffer_size_default(),
			private_enterprise_number: Default::default(),
		}
	}
}

impl ProcessLoggingConfiguration
{
	/// Configure logging.
	#[inline(always)]
	pub fn configure_logging(&self, additional_logging_configuration: &mut impl AdditionalLoggingConfiguration, dev_path: &DevPath, running_interactively_so_also_log_to_standard_error: bool, internet_protocol_addresses: &[IpAddr], host_name: Option<&LinuxKernelHostName>, domain_name: Option<&LinuxKernelDomainName>, process_name: &ProcessName, boot_identifier: &BootIdentifierUniversallyUniqueIdentifier) -> Result<(), ProcessLoggingConfigurationError>
	{
		let configuration = StaticLoggingConfiguration::new(self.logging_buffer_size, dev_path, host_name, domain_name, internet_protocol_addresses, &self.private_enterprise_number, process_name)?;
		unsafe { configuration.configure() };
		
		unsafe { LocalSyslogSocket::configure_per_thread_local_syslog_socket()? }
		
		self.configure_syslog_for_legacy_third_party_libraries_that_use_syslog_interface(running_interactively_so_also_log_to_standard_error, process_name);
		
		additional_logging_configuration.configure(host_name, domain_name, internet_protocol_addresses, process_name, boot_identifier).map_err(|cause| ProcessLoggingConfigurationError::AdditionalLoggingConfigurationFailed(cause))?;
		
		Ok(())
	}
	
	/// Legacy `syslog()` is problematic:-
	///
	/// * It makes two syscalls, `getpid()` and `send()` for every message logged;
	/// * It is dubious as to whether it is truly thread safe;
	/// * It does not support re-trying in some circumstances which are recoverable;
	/// * It is hardcoded to `/dev/log`.
	#[inline(always)]
	fn configure_syslog_for_legacy_third_party_libraries_that_use_syslog_interface(&self, running_interactively_so_also_log_to_standard_error: bool, identity: &ProcessName)
	{
		unsafe { setlogmask(self.libc_syslog_level.log_upto()) };
		
		let mut log_options = LOG_PID | LOG_NDELAY;
		
		if running_interactively_so_also_log_to_standard_error
		{
			log_options |= LOG_PERROR;
		}
		
		let identity: &CStr = identity.as_ref();
		unsafe { openlog(identity.as_ptr(), log_options, self.libc_syslog_facility as u8 as i32) }
	}
	
	/// `self.configure_logging()` must have been called before this method.
	#[inline(always)]
	pub fn redirect_FILE_standard_out_and_file_standard_error_to_log(&self, host_name: Option<&LinuxKernelHostName>, process_name: &ProcessName)
	{
		#[inline(always)]
		fn redirect_to_log<MT: MessageTemplate>(original: &mut *const FILE, message_template: MT, callback: unsafe extern "C" fn(&MT, data: *const c_char, length: size_t) -> ssize_t)
		{
			let message_template = Rc::new(message_template);
			
			let mut functions = cookie_io_functions_t::default();
			functions.write = unsafe { transmute(callback) };

			let cookie = message_template.deref();
			
			let file = unsafe { fopencookie(cookie as *const MT as *mut MT as *mut c_void, b"w\0".as_ptr() as *const c_char, functions) };
			assert!(!file.is_null(), "file is null from `fopencookie()`");
			*original = file;
			let result = unsafe { setvbuf(*original as *mut _, null_mut(), _IOLBF, 0) };
			assert_eq!(result, 0, "`setvbuf()` returned `{}`", result);
			
			forget(message_template);
		}
		
		redirect_to_log(unsafe { &mut stdio::stdout }, Rfc3164MessageTemplate::new(self.libc_syslog_facility, self.redirect_FILE_standard_out, host_name, process_name), Self::write_file_pointer_data_to_log);
		redirect_to_log(unsafe { &mut stdio::stderr }, Rfc3164MessageTemplate::new(self.libc_syslog_facility, self.redirect_FILE_standard_error, host_name, process_name), Self::write_file_pointer_data_to_log);
	}
	
	/// Used to support redirecting lib c `FILE*` pointer to standard out to syslog.
	///
	/// Only used if a linked C library uses it.
	unsafe extern "C" fn write_file_pointer_data_to_log<MT: MessageTemplate + RefUnwindSafe>(message_template: &MT, data: *const c_char, length: size_t) -> ssize_t
	{
		// Calling code is from C, and it is undefined behaviour to pass a Rust panic across to C.
		let result = catch_unwind
		(
		||
			{
				let bytes = from_raw_parts(data as *const u8, length);
				let message = String::from_utf8_lossy(bytes);
				LocalSyslogSocket::syslog_falling_back_to_standard_error(message_template, &message)
			}
		);
		forget(result);

		length as ssize_t
	}
	
	#[inline(always)]
	const fn logging_buffer_size_default() -> NonZeroUsize
	{
		new_non_zero_usize(8192)
	}
}
