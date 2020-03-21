/// Path to CString.
#[inline(always)]
pub fn path_to_cstring(path: &Path) -> CString
{
	CString::new(path.as_os_str().as_bytes()).unwrap()
}
