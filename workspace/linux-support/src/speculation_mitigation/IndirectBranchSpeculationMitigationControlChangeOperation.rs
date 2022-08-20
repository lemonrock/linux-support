// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Change speculation mitigation.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(i32)]
pub enum IndirectBranchSpeculationMitigationControlChangeOperation
{
	#[allow(missing_docs)]
	SpeculationEnabled = PR_SPEC_ENABLE,
	
	#[allow(missing_docs)]
	SpeculationDisabled = PR_SPEC_DISABLE,
	
	#[allow(missing_docs)]
	ForceSpeculationDisabled = PR_SPEC_FORCE_DISABLE,
}

impl IndirectBranchSpeculationMitigationControlChangeOperation
{
	/// Returns:-
	///
	/// * `EPERM` if speculation mitigation has been force disabled.
	/// * `ERANGE` if an unsupported speculation mitigation strategy is used.
	#[inline(always)]
	pub fn change_for_current_thread(self) -> Result<(), SystemCallErrorNumber>
	{
		StoreBypassSpeculationMitigationControlChangeOperation::change(PR_SPEC_INDIRECT_BRANCH, self as i32)
	}
}
