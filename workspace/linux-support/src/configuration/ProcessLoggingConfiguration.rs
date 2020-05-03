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
	/// Issues a warning (currently to syslog).
	///
	/// NOTE: Using `syslog()` takes a hit of calling the `getpid()` system call and then the `send()` system call for every log message.
	#[inline(always)]
	pub fn warn(name: &str, message: String)
	{
		let name = to_c_string_robustly(name);
		let message = to_c_string_robustly(message);
		unsafe { syslog(LOG_WARNING, b"%s:%s\0".as_ptr() as *const _ as *const _, name.as_ptr(), message.as_ptr()) };
	}

	/// Start logging.
	///
	/// Strictly speaking `identity` can be 31 characters excluding any trailing NUL.
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
	pub fn stop_logging(&self)
	{
		unsafe { closelog() }
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

	/// NOTE: Using `syslog()` takes a hit of calling the `getpid()` system call and then the `send()` system call for every log message.
	#[inline(always)]
	pub(crate) fn caught_panic(thread_name: &str, thread_id: ThreadId, source_file: &str, line_number: u32, column_number: u32, cause: &str, backtrace: String)
	{
		let thread_name = to_c_string_robustly(thread_name);
		let source_file = to_c_string_robustly(source_file);
		let cause = to_c_string_robustly(cause);
		let backtrace = to_c_string_robustly(&backtrace);
		unsafe { syslog(LOG_CRIT, b"ThreadName:%s:ThreadId:%llu:File:%s:Line:%u:Column:%u:Cause:%sBacktrace:%s\0".as_ptr() as *const _ as *const _, thread_name.as_ptr(), thread_id, source_file.as_ptr(), line_number, column_number, cause.as_ptr(), backtrace.as_ptr()) }
	}

	#[inline(always)]
	const fn enable_full_rust_stack_back_traces_default() -> bool
	{
		true
	}
}
