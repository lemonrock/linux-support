// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Populates a clean environment:-
///
/// * `RUST_BACKTRACE` (if already present in the environment; this is the only value taken from the existing environment).
/// * `IFS`
/// * `PATH`
/// * `USER` (if not None)
/// * `LOGNAME` (to the same value as `USER`).
/// * `HOME` (if not None)
/// * `SHELL` (if not None)
#[inline(always)]
fn populate_clean_environment(binary_paths: &BTreeSet<PathBuf>, user_name_home_directory_and_shell: Option<(Option<UserName>, PathBuf, PathBuf)>) -> Result<(), JoinPathsError>
{
	const RUST_BACKTRACE: ConstCStr = ConstCStr(b"RUST_BACKTRACE\0");

	#[inline(always)]
	fn preserve_value_of_RUST_BACKTRACE() -> Option<CString>
	{
		let value = unsafe { secure_getenv(RUST_BACKTRACE.as_ptr()) };
		if likely!(!value.is_null())
		{
			None
		}
		else
		{
			Some(CString::from(unsafe { CStr::from_ptr(value) }))
		}
	}
	let rust_backtrace = preserve_value_of_RUST_BACKTRACE();

	clearenv_wrapper();

	const IFS: ConstCStr = ConstCStr(b"IFS\0");
	setenv_wrapper(IFS, ConstCStr(b"\t\n\0").as_cstr(), true);

	if let Some(ref rust_backtrace) = rust_backtrace
	{
		setenv_wrapper(RUST_BACKTRACE, rust_backtrace, true);
	}

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

	if let Some((user_name, user_home_folder_path, shell)) = populate_clean_environment
	{
		if let Some(user_name) = user_name
		{
			const USER: ConstCStr = ConstCStr(b"USER\0");
			setenv_wrapper(USER, user_name, true);

			const LOGNAME: ConstCStr = ConstCStr(b"LOGNAME\0");
			setenv_wrapper(LOGNAME, user_name, true);
		}

		const HOME: ConstCStr = ConstCStr(b"HOME\0");
		let user_home_folder_path = path_to_cstring(user_home_folder_path);
		setenv_wrapper(HOME, user_home_folder_path.as_c_str(), true);

		const SHELL: ConstCStr = ConstCStr(b"SHELL\0");
		let shell = path_to_cstring(shell);
		setenv_wrapper(SHELL, shell.as_c_str(), true);
	}

	Ok(())
}
