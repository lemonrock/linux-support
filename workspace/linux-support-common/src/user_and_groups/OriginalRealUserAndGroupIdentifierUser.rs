/// Typically used to create a PID file in `/var/run` before switching user and group.
pub trait OriginalRealUserAndGroupIdentifierUser
{
	/// Do something with the effective user and effective group identifiers before switching user and group.
	fn create_pid_file_before_switching_user_and_group(&self, effective_user_identifier: uid_t, effective_group_identifier: gid_t);
}
