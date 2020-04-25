// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An object that can be used with a configuration file (eg via Serde) to configure a daemon.
///
/// The following are done:-
///
/// * umask is set to just the current user
/// * Checks are made to check the program is not running with the set uid bit set ('setuid' or 'suid').
/// * A PID file is created
/// * standard in is redirected to `/dev/null`.
/// * standard out is redirected to `/dev/null`.
/// * standard error is redirected to `/dev/null`.
/// * `fprintf` and friends using the `FILE` API are redirected to syslog on Linux (this is probably also possible to implement for FreeBSD - see <https://mischasan.wordpress.com/2011/05/25/redirecting-stderr-to-syslog/>).
/// * Double forking and a new session are created.
/// * Real and effective user and group ids are changed.
/// * Additional groups from `/etc/group`, if any, are assigned.
/// * Environment variables are populated if missing (`IFS`, `PATH`)
/// * User environment variables are overwritten (`HOME`, `LOGNAME`, `USER`).
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Daemonize
{
	/// A folder path in which to put a PID file.
	///
	/// This uses the processes' name for the actual file base name.
	///
	/// Defaults to `/var/run`.
	#[serde(default = "Daemonize::pid_folder_path_default")] pub pid_folder_path: PathBuf,

	/// Defaults to program name.
	///
	/// Used to name a PID file.
	#[serde(default = "Daemonize::program_name_default")] pub program_name: String,

	#[serde(skip)] pid_file_path: UnsafeCell<Option<PathBuf>>,
}

impl Default for Daemonize
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			pid_folder_path: Self::pid_folder_path_default(),
			program_name: Self::program_name_default(),
			pid_file_path: UnsafeCell::new(None),
		}
	}
}

impl OriginalRealUserAndGroupIdentifierUser for Daemonize
{
	fn create_pid_file_before_switching_user_and_group(&self, effective_user_identifier: UserIdentifier, effective_group_identifier: GroupIdentifier)
	{
		let pid_file_path = self.pid_file_path();
		let pid_file_path_string = pid_file_path.to_c_string();

		let file_descriptor = unsafe { open(pid_file_path_string.as_ptr(), O_CREAT | O_WRONLY, (S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH) as u32) };
		assert!(file_descriptor >= 0, "Could not create PID file '{:?}' because '{}'", &pid_file_path_string, Self::os_error());
		assert_eq!(unsafe { fchown(file_descriptor, effective_user_identifier.into(), effective_group_identifier.into()) }, 0, "Could not change ownership of PID file '{:?}' because '{}'", &pid_file_path_string, Self::os_error());
		unsafe { close(file_descriptor) };
	}
}

impl Daemonize
{
	/// Daemonizes the current process.
	///
	/// Returns an object that needs to have `clean_up()` called on it just before process exit.
	#[inline(always)]
	pub fn daemonize(&self, dev_path: &DevPath) -> DaemonizeCleanUpOnExit
	{
		Self::fork();

		Self::create_a_new_process_group_and_session_detach_controlling_terminal();

		Self::fork();

		self.redirect_standard_in_out_and_error(dev_path);

		self.populate_pid_file_when_running(&self.pid_file_path());

		DaemonizeCleanUpOnExit
		{
			pid_file_path: self.pid_file_path().to_path_buf()
		}
	}

	#[inline(always)]
	fn populate_pid_file_when_running(&self, pid_file_path: &Path)
	{
		pid_file_path.write_value(process::id()).expect("Could not write to PID file");
	}

	#[inline(always)]
	fn create_a_new_process_group_and_session_detach_controlling_terminal()
	{
		let result = unsafe { setsid() };
		if likely!(result > 0)
		{
			return
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EPERM => panic!("The process group ID of any process equals the PID of the calling process. Thus, in particular, setsid() fails if the calling process is already a process group leader."),

				unknown @ _ => panic!("Unknown error code `{}` fron `setsid()`", unknown),
			}
		}
		else
		{
			panic!("Unknown result `{}` from `setsid()`", result)
		}
	}

	#[inline(always)]
	fn fork()
	{
		const ForkedToChild: i32 = 0;

		match unsafe { fork() }
		{
			ForkedToChild => (),
			-1 => panic!("Fork failed with {}", Self::os_error()),
			_child_process_id_returned_to_parent @ _ => process::exit(0),
		}
	}

	#[inline(always)]
	fn redirect_standard_in_out_and_error(&self, dev_path: &DevPath)
	{
		let dev_null = dev_path.null().as_os_str().os_str_to_c_string();

		Self::redirect_to_dev_null(&iostdin(), &dev_null);
		Self::redirect_to_dev_null(&iostdout(), &dev_null);
		Self::redirect_to_dev_null(&iostderr(), &dev_null);

		redirect_standard_out_and_standard_error_to_syslog()
	}

	#[inline(always)]
	fn redirect_to_dev_null<A: AsRawFd>(a: &A, dev_null: &CStr)
	{
		let file_descriptor = a.as_raw_fd();
		let null_file_descriptor = unsafe { open(dev_null.as_ptr(), O_WRONLY) };
		assert!(null_file_descriptor >= 0, "Could not open /dev/null because '{}'", Self::os_error());
		assert_eq!(unsafe { dup2(null_file_descriptor, file_descriptor)}, 0, "Could not dup2 because '{}'", Self::os_error());
		assert_eq!(unsafe { close(null_file_descriptor) }, 0, "Could not close null file descriptor because '{}'", Self::os_error());
	}

	#[inline(always)]
	fn pid_file_path(&self) -> &Path
	{
		let value = unsafe { &mut * self.pid_file_path.get() };
		if unlikely!(value.is_none())
		{
			let path = self.pid_folder_path.clone().append(format!("{}.pid", &self.program_name));
			value.replace(path);
		}
		value.as_ref().unwrap()
	}

	#[inline(always)]
	fn os_error() -> io::Error
	{
		io::Error::last_os_error()
	}

	#[inline(always)]
	fn pid_folder_path_default() -> PathBuf
	{
		PathBuf::from("/var/run")
	}

	#[inline(always)]
	fn program_name_default() -> String
	{
		get_program_name()
	}
}
