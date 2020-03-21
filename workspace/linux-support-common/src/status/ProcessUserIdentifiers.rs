/// User identifiers (UIDs).
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessUserIdentifiers
{
	/// Real user identifier (UID).
	pub real: uid_t,

	/// Effective user identifier (UID).
	pub effective: uid_t,

	/// Saved set user identifier (UID).
	pub saved_set: uid_t,

	/// File system user identifier (UID).
	pub file_system: uid_t,
}