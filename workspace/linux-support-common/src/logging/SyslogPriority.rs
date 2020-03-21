/// Defaults to `debug` for debug builds and `warning` for production builds.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[allow(missing_docs)]
#[repr(i32)]
pub enum SyslogPriority
{
	emergency = LOG_EMERG,
	alert = LOG_ALERT,
	critical = LOG_CRIT,
	error = LOG_ERR,
	warning = LOG_WARNING,
	notice = LOG_NOTICE,
	info = LOG_INFO,
	debug = LOG_DEBUG,
}

impl Default for SyslogPriority
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::SyslogPriority::*;

		if cfg!(debug_assertions)
		{
			debug
		}
		else
		{
			warning
		}
	}
}

impl SyslogPriority
{
	/// Maximum priority to log upto.
	#[inline(always)]
	pub fn log_upto(self) -> i32
	{
		(1 << ((self as i32) + 1)) - 1
	}
}
