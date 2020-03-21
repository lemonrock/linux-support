/// Speculation store ('Spectre' vulnerability) bypass status.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpeculationStoreBypassStatus
{
	/// Linux errored internally with `EINVAL`!
	Unknown,

	/// `prctl(PR_SPEC_STORE_BYPASS)` is `PR_SPEC_NOT_AFFECTED`.
	NotVulnerable,

	/// `prctl(PR_SPEC_STORE_BYPASS)` is `PR_SPEC_PRCTL | PR_SPEC_FORCE_DISABLE`.
	ThreadForceMitigated,

	/// `prctl(PR_SPEC_STORE_BYPASS)` is `PR_SPEC_PRCTL | PR_SPEC_DISABLE`.
	ThreadMitigated,

	/// `prctl(PR_SPEC_STORE_BYPASS)` is `PR_SPEC_PRCTL | PR_SPEC_ENABLE`.
	ThreadVulnerable,

	/// `prctl(PR_SPEC_STORE_BYPASS)` is `PR_SPEC_DISABLE`.
	GloballyMitigated,

	/// `prctl(PR_SPEC_STORE_BYPASS)` is any other value to those above.
	Vulnerable,
}