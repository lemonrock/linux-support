// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn redirect_to_syslog(original: &mut *const FILE, callback: cookie_write_function_t)
{
	const w: ConstCStr = ConstCStr(b"w\0");

	let mut functions = cookie_io_functions_t::default();
	functions.write = callback;

	let file = unsafe { fopencookie(null_mut(), w.as_ptr(), functions) };
	assert!(!file.is_null(), "file is null from `fopencookie()`");
	*original = file;
	let result = unsafe { setvbuf(*original as *mut _, null_mut(), _IOLBF, 0) };
	assert_eq!(result, 0, "`setvbuf()` returned `{}`", result)
}
