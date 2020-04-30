// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An object that can be used with a configuration file (eg via Serde) to configure a daemon.
///
/// The following are done:-
///
/// * standard in is redirected to `/dev/null`.
/// * standard out is redirected to `/dev/null`.
/// * standard error is redirected to `/dev/null`.
/// * `fprintf` and friends using the `FILE` API are redirected to syslog on Linux.
/// * Double forking and a new session are created.
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

impl Daemonize
{


}
