/// Enable or disable transparent huge pages.
#[inline(always)]
pub fn adjust_transparent_huge_pages(enable_transparent_huge_pages: bool)
{
	let value = if enable_transparent_huge_pages
	{
		1
	}
	else
	{
		0
	};
	unsafe { prctl(PR_SET_THP_DISABLE, value as c_ulong) };
}
