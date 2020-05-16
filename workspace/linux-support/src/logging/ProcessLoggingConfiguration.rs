// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Logging configuration.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessLoggingConfiguration
{
	/// Defaults to `auth`.
	#[serde(default)] pub syslog_facility: SyslogFacility,

	/// Defaults to `debug` for debug builds and `warning` for production builds.
	#[serde(default)] pub syslog_priority: SyslogPriority,

	/// When a panic occurs that isn't caught capture a full stack back trace.
	#[serde(default = "ProcessLoggingConfiguration::enable_full_rust_stack_back_traces_default")] pub enable_full_rust_stack_back_traces: bool,
}

impl Default for ProcessLoggingConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			syslog_facility: SyslogFacility::default(),
			syslog_priority: SyslogPriority::default(),
			enable_full_rust_stack_back_traces: Self::enable_full_rust_stack_back_traces_default(),
		}
	}
}

impl ProcessLoggingConfiguration
{
	/// Start logging.
	///
	/// Strictly speaking `identity` can be 31 characters excluding any trailing NUL.
	///
	/// Under the covers opens the `/dev/log` Unix socket.
	///
	/// A better technique is to probably send modern syslog datagrams (RFC 5424 / 5426) to localhost:514 and use rsyslog.
	/// This would allow us to control the `/dev` path as we do elsewhere.
	/// It would also allow us to send messages >1024 bytes.
	///
	/// It would require per-thread loggers as socket writes are not atomic for large messages.
	#[inline(always)]
	pub fn start_logging(&self, running_interactively_so_also_log_to_standard_error: bool, identity: &ProcessName)
	{
		self.configure_rust_stack_back_traces();
		self.configure_syslog(running_interactively_so_also_log_to_standard_error, identity);
	}

	/// Stop logging.
	///
	/// Not really very important; just closes a file descriptor.
	#[inline(always)]
	pub fn stop_logging()
	{
		unsafe { closelog() }
	}

	/// NOTE: Using `syslog()` takes a hit of calling the `getpid()` system call and then the `send()` system call for every log message.
	///
	/// `priority` is, say `warning`.
	///
	/// `self.start_logging()` must have been called before this method.
	pub fn syslog(priority: SyslogPriority, message: String)
	{
		Self::syslog_c_string_message(priority, unsafe { CString::from_vec_unchecked(message.into_bytes()) })
	}

	/// NOTE: Using `syslog()` takes a hit of calling the `getpid()` system call and then the `send()` system call for every log message.
	///
	/// `priority` is, say `warning`.
	///
	/// `self.start_logging()` must have been called before this method.
	pub fn syslog_c_string_message(priority: SyslogPriority, message: CString)
	{
		unsafe { syslog(priority as i32, b"%s\0".as_ptr() as *const c_char, message.as_ptr()) };
	}

	/// Redirect `FILE*` standard out and `FILE*` standard error to syslog.
	///
	/// NOTE: Using `syslog()` takes a hit of calling the `getpid()` system call and then the `send()` system call for every log message.
	///
	/// `self.start_logging()` must have been called before this method.
	#[inline(always)]
	pub fn redirect_file_standard_out_and_file_standard_error_to_syslog()
	{
		#[inline(always)]
		fn redirect_to_syslog(original: &mut *const FILE, callback: cookie_write_function_t)
		{
			let mut functions = cookie_io_functions_t::default();
			functions.write = callback;

			let file = unsafe { fopencookie(null_mut(), b"w\0".as_ptr() as *const c_char, functions) };
			assert!(!file.is_null(), "file is null from `fopencookie()`");
			*original = file;
			let result = unsafe { setvbuf(*original as *mut _, null_mut(), _IOLBF, 0) };
			assert_eq!(result, 0, "`setvbuf()` returned `{}`", result)
		}

		redirect_to_syslog(unsafe { &mut stdout }, Self::write_standard_out_to_syslog);
		redirect_to_syslog(unsafe { &mut stderr }, Self::write_standard_error_to_syslog);
	}

	/// Used to support redirecting lib c `FILE*` pointer to standard out to syslog.
	#[inline(always)]
	unsafe extern "C" fn write_standard_out_to_syslog(_cookie: *mut c_void, data: *const c_char, length: size_t) -> ssize_t
	{
		Self::write_file_pointer_data_to_syslog(SyslogPriority::notice, data, length)
	}

	/// Used to support redirecting lib c `FILE*` pointer to standard error to syslog.
	#[inline(always)]
	unsafe extern "C" fn write_standard_error_to_syslog(_cookie: *mut c_void, data: *const c_char, length: size_t) -> ssize_t
	{
		Self::write_file_pointer_data_to_syslog(SyslogPriority::error, data, length)
	}

	#[inline(always)]
	fn write_file_pointer_data_to_syslog(priority: SyslogPriority, data: *const c_char, length: size_t) -> ssize_t
	{
		// Calling code is from C, and it is undefined behaviour to pass a Rust panic across to C.
		let result = catch_unwind
		(||
			 {
				 let message = unsafe { CString::from_vec_unchecked(from_raw_parts(data as *const u8, length).to_vec()) };
				 Self::syslog_c_string_message(priority, message);
			 }
		);
		forget(result);

		length as ssize_t
	}

	#[inline(always)]
	fn configure_rust_stack_back_traces(&self)
	{
		let setting = if self.enable_full_rust_stack_back_traces
		{
			"1"
		}
		else
		{
			"0"
		};
		set_var("RUST_BACKTRACE", setting)
	}

	#[inline(always)]
	fn configure_syslog(&self, running_interactively_so_also_log_to_standard_error: bool, identity: &ProcessName)
	{
		unsafe { setlogmask(self.syslog_priority.log_upto()) };

		let mut log_options = LOG_PID | LOG_NDELAY;

		if running_interactively_so_also_log_to_standard_error
		{
			log_options |= LOG_PERROR;
		}

		let identity = identity.as_ref();
		unsafe { openlog(identity.as_ptr(), log_options, self.syslog_facility as i32) }
	}

	#[inline(always)]
	const fn enable_full_rust_stack_back_traces_default() -> bool
	{
		true
	}
}