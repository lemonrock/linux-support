// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// NOTE: This does not use the Rust functions (eg `set_var_os`), as we do not want to convert from a libc-supplied string to an `OsString`.
#[inline(always)]
fn setenv_wrapper(name: ConstCStr, value: &CStr, overwrite: bool)
{
	let result = unsafe { setenv(name.as_ptr(), value.as_ptr(), overwrite as i32) };
	if likely!(result == 0)
	{
		return
	}
	else if likely!(result == -1)
	{
		match errno().0
		{
			EINVAL => panic!("name is NULL, points to a string of length 0, or contains an '=' character"),
			ENOMEM => panic!("Insufficient memory to add a new variable to the environment"),

			unknown @ _ => panic!("Unknown errno from `setenv()` of `{}`", unknown),
		}
	}
	else
	{
		panic!("`setenv()` returned an expected result `{}`", result)
	}
}
