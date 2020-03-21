#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum scmp_filter_attr
{
	#[doc(hidden)]
	_SCMP_FLTATR_MIN = 0,

	/// Default filter action (read only).
	SCMP_FLTATR_ACT_DEFAULT = 1,

	/// Bad architecture action.
	SCMP_FLTATR_ACT_BADARCH = 2,

	/// Set `NO_NEW_PRIVS` on filter load.
	SCMP_FLTATR_CTL_NNP = 3,

	/// Synchronize threads on filter load.
	SCMP_FLTATR_CTL_TSYNC = 4,

	/// Allow rules that specify syscall as `-1`.
	SCMP_FLTATR_API_TSKIP = 5,

	/// Log not-allowed actions.
	SCMP_FLTATR_CTL_LOG = 6,

	/// Disable Speculative Store Bypass mitigation.
	SCMP_FLTATR_CTL_SSB = 7,

	#[doc(hidden)]
	_SCMP_FLTATR_MAX,
}
