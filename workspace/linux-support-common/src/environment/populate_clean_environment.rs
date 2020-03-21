// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Populate a new set of environment variables, ensuring all previous variables have been removed.
#[inline(always)]
pub fn populate_clean_environment(binary_paths: &BTreeSet<PathBuf>, effective_user_name: &CStr, effective_user_home_folder_path: &CStr)
{
	const RUST_BACKTRACE: ConstCStr = ConstCStr(b"RUST_BACKTRACE\0");

	#[inline(always)]
	fn preserve_value_of_RUST_BACKTRACE() -> Option<CString>
	{
		let value = unsafe { getenv(RUST_BACKTRACE.as_ptr()) };
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

	if let Some(ref rust_backtrace) = rust_backtrace
	{
		setenv_wrapper(RUST_BACKTRACE, rust_backtrace, true);
	}

	const IFS: ConstCStr = ConstCStr(b"IFS\0");
	setenv_wrapper(IFS, ConstCStr(b"\t\n\0").as_cstr(), true);

	let path = join_paths(binary_paths.iter()).expect("Could not join paths because `binary_paths` contained a path with a colon in it");
	set_var("PATH", &path);

	const USER: ConstCStr = ConstCStr(b"USER\0");
	setenv_wrapper(USER, effective_user_name, true);

	const LOGNAME: ConstCStr = ConstCStr(b"USER\0");
	setenv_wrapper(LOGNAME, effective_user_name, true);

	const HOME: ConstCStr = ConstCStr(b"HOME\0");
	setenv_wrapper(HOME, effective_user_home_folder_path, true);
}
