/// Needs to be called after process change.
#[inline(always)]
pub fn disable_dumpable()
{
	unsafe { prctl(PR_SET_DUMPABLE, 0 as c_ulong) };
}
