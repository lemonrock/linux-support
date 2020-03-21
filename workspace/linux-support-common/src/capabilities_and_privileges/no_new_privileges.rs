/// Privilege protection.
#[inline(always)]
pub fn no_new_privileges()
{
	unsafe { prctl(PR_SET_NO_NEW_PRIVS, 1 as c_ulong, 0 as c_ulong, 0 as c_ulong, 0 as c_ulong) };
}
