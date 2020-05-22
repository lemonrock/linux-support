// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Populates a clean environment:-
///
/// * `IFS`.
/// * `RUST_BACKTRACE`.
/// * `PATH`.
/// * `USER` (if not None).
/// * `LOGNAME` (to the same value as `USER`).
/// * `HOME` (if not None).
/// * `SHELL` (if not None).
/// * `TZ` to `:/etc/zoneinfo/UTC` (actually `time_zone_file_path`); this is done to (a) make sure any hard compiled search paths in libc are not searched (b) to ensure the timezone is always UTC irrespective of the system and (c) in some cases, to reduce the number of syscalls that the libc might make.
#[inline(always)]
pub(crate) fn populate_clean_environment(enable_full_rust_stack_back_traces: bool, binary_paths: &BTreeSet<PathBuf>, user_name_home_directory_and_shell: Option<(Option<UserName>, PathBuf, PathBuf)>, time_zone_file_path: PathBuf) -> Result<(), JoinPathsError>
{
	clearenv_wrapper();

	const IFS: ConstCStr = ConstCStr(b"IFS\0");
	setenv_wrapper(IFS, ConstCStr(b"\t\n\0").as_cstr(), true);
	
	const RUST_BACKTRACE: ConstCStr = ConstCStr(b"RUST_BACKTRACE\0");
	const RustBacktraceTrue: ConstCStr = ConstCStr(b"1\0");
	const RustBacktraceFalse: ConstCStr = ConstCStr(b"0\0");
	let rust_backtrace_value = if enable_full_rust_stack_back_traces
	{
		RustBacktraceTrue
	}
	else
	{
		RustBacktraceFalse
	};
	setenv_wrapper(RUST_BACKTRACE, rust_backtrace_value.as_cstr(), true);
	
	if binary_paths.is_empty()
	{
		const PATH: ConstCStr = ConstCStr(b"PATH\0");
		setenv_wrapper(PATH, ConstCStr(b"\0").as_cstr(), true)
	}
	else
	{
		let path = join_paths(binary_paths.iter())?;
		set_var("PATH", &path);
	}

	if let Some((user_name, user_home_folder_path, shell)) = user_name_home_directory_and_shell
	{
		if let Some(user_name) = user_name
		{
			const USER: ConstCStr = ConstCStr(b"USER\0");
			setenv_wrapper(USER, user_name.as_ref(), true);

			const LOGNAME: ConstCStr = ConstCStr(b"LOGNAME\0");
			setenv_wrapper(LOGNAME, user_name.as_ref(), true);
		}

		const HOME: ConstCStr = ConstCStr(b"HOME\0");
		let user_home_folder_path = path_to_cstring(user_home_folder_path.as_ref());
		setenv_wrapper(HOME, user_home_folder_path.as_c_str(), true);

		const SHELL: ConstCStr = ConstCStr(b"SHELL\0");
		let shell = path_to_cstring(shell.as_ref());
		setenv_wrapper(SHELL, shell.as_c_str(), true);
	}

	{
		const TZ: ConstCStr = ConstCStr(b"TZ\0");
		let mut bytes = time_zone_file_path.to_c_string().into_bytes();
		bytes.insert(0, b':');
		let time_zone_file_path = unsafe { CString::from_vec_unchecked(bytes) };
		setenv_wrapper(TZ, time_zone_file_path.as_c_str(), true);
	}

	Ok(())
}
