// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An user identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct UserIdentifier(uid_t);

impl From<uid_t> for UserIdentifier
{
	#[inline(always)]
	fn from(value: uid_t) -> Self
	{
		Self(value)
	}
}

impl Into<uid_t> for UserIdentifier
{
	#[inline(always)]
	fn into(self) -> uid_t
	{
		self.0
	}
}

impl Into<i32> for UserIdentifier
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self.0 as i32
	}
}

impl FromBytes for UserIdentifier
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn from_bytes(value: &[u8]) -> Result<Self, Self::Error>
	{
		Ok(Self(uid_t::parse_decimal_number(value)?))
	}
}

impl Default for UserIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::current_real()
	}
}

impl UserOrGroupIdentifier for UserIdentifier
{
	const Zero: Self = UserIdentifier(0);

	const FileName: &'static str = "uid_map";

	const root: Self = Self(0);

	#[inline(always)]
	fn current_real() -> Self
	{
		Self(unsafe { getuid() })
	}

	#[inline(always)]
	fn current_effective() -> Self
	{
		Self(unsafe { geteuid() })
	}

	#[allow(deprecated)]
	#[inline(always)]
	fn current_real_effective_and_saved_set() -> (Self, Self, Self)
	{
		let mut real: uid_t = unsafe { uninitialized() };
		let mut effective: uid_t = unsafe { uninitialized() };
		let mut saved_set: uid_t = unsafe { uninitialized() };
		let result = unsafe { getresuid(&mut real, &mut effective, &mut saved_set) };
		if likely!(result == 0)
		{
			(Self(real), Self(effective), Self(saved_set))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EFAULT => panic!("Invalid address for real, effective or saved_set argument pointer"),
				unexpected @ _ => panic!("Unexpected error `{}` from `getresuid()`", unexpected),
			}
		}
		else
		{
			panic!("Unexpectec result `{}` from `getresuid()`", result)
		}
	}

	#[inline(always)]
	fn set_real_effective_and_saved_set(real: Option<Self>, effective: Option<Self>, saved_set: Option<Self>)
	{
		#[inline(always)]
		fn into_i32(value: Option<UserIdentifier>) -> u32
		{
			value.map_or(-1i32 as u32, |value| value.into())
		}

		let result = unsafe { setresuid(into_i32(real), into_i32(effective), into_i32(saved_set)) };
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EAGAIN => panic!("uid does not match the current UID and this call would bring that user ID over its `RLIMIT_NPROC` resource limit"),
				EPERM => panic!("The calling process is not privileged (did not have the `CAP_SETUID` capability) and tried to change the IDs to values that are not permitted."),

				unknown @ _ => panic!("Unknown error `{}` from `setresuid()`", unknown),
			}
		}
		else
		{
			panic!("Invalid result `{}` from setresuid()", result)
		}
	}

	#[inline(always)]
	fn set_file_system(self) -> Self
	{
		Self((unsafe { setfsuid(self.0) }) as uid_t)
	}
}

impl UserIdentifier
{
	/// Gets the user name, home directory and shell from `/etc/passwd`, or nothing if `/etc/passwd` does not exist, can not be parsed or doesn't have a relevant entry.
	///
	/// If user name contained an interior nul, returns `None` for user name.
	#[inline(always)]
	pub fn user_name_home_directory_and_shell(self, etc_path: &EtcPath) -> Option<(Option<UserName>, PathBuf, PathBuf)>
	{
		UserAndGroupChoice::etc_passwd_record_for_user_identifier(etc_path, self, |etc_passwd_record| Ok(
		(
			etc_passwd_record.user_name().ok(),
			etc_passwd_record.home_directory(),
			etc_passwd_record.shell(),
		))).ok()
	}
	
	
	/// ?May be `None` if audit is not running?
	///
	/// Also known as audit user id.
	#[inline(always)]
	pub fn audit_login(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<Option<Self>>
	{
		let value: u32 = proc_path.process_file_path(process_identifier, "loginuid").read_value()?;
		if value == u32::MAX
		{
			Ok(None)
		}
		else if value <= (i32::MAX as u32)
		{
			Ok(Some(Self(value)))
		}
		else
		{
			unreachable!("Invalid value `{}` in /proc/{:?}/loginuid", value, process_identifier)
		}
	}
	
	/// ?May be `None` if audit is not running?
	///
	/// Also known as audit user id.
	///
	/// Typically set by a `PAM` module or login-like daemon such as `SSH`.
	#[inline(always)]
	pub fn set_audit_login(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice, value: Option<Self>) -> io::Result<()>
	{
		let value: u32 = match value
		{
			None => u32::MAX,
			Some(value) => value.into(),
		};
		
		proc_path.process_file_path(process_identifier, "loginuid").write_value(UnpaddedDecimalInteger(value))
	}
}
