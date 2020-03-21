/// An extension trait for `OsStr`.
pub trait OsStrExtMore
{
	/// Converts as `OsStr` to a `CString`.
	fn os_str_to_c_string(&self) -> CString;
}

impl OsStrExtMore for OsStr
{
	#[inline(always)]
	fn os_str_to_c_string(&self) -> CString
	{
		CString::new(self.as_bytes()).expect("os_str should not contain interior ASCII NULs")
	}
}
