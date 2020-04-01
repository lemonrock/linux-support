// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// To CString robustly.
#[inline(always)]
pub fn to_c_string_robustly<T: Into<Vec<u8>>>(string: T) -> CString
{
	#[inline(always)]
	fn substitute_for_bad_c_string() -> CString
	{
		CString::new("?").unwrap()
	}

	CString::new(string).unwrap_or_else(|_| substitute_for_bad_c_string())
}
