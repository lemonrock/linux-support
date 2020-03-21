/// Asserts that the effective user id (`uid`) is root.
///
/// Takes a necessity to explain why the user must be root.
#[inline(always)]
pub fn assert_effective_user_id_is_root(necessity: &str)
{
	let effective_user_id = unsafe { geteuid() };
	assert_eq!(effective_user_id, 0, "Effective User Id (euid) is not root (0). Necessity: {}", necessity);
}
