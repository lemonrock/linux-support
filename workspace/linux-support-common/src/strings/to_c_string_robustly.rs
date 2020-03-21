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
