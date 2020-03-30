// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


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