/// Group identifiers (GIDs).
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessGroupIdentifiers
{
	/// Real group identifier (GID).
	pub real: gid_t,

	/// Effective group identifier (GID).
	pub effective: gid_t,

	/// Saved set group identifier (GID).
	pub saved_set: gid_t,

	/// File system group identifier (GID).
	pub file_system: gid_t,
}