// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct GroupsDiagnostics
{
	pub real: GroupIdentifierDiagnostic,
	
	pub effective: GroupIdentifierDiagnostic,
	
	pub saved_set: GroupIdentifierDiagnostic,

	pub supplementary: Vec<GroupIdentifierDiagnostic>,
}

impl GroupsDiagnostics
{
	fn gather(etc_path: &EtcPath) -> Self
	{
		let (real, effective, saved_set) = GroupIdentifier::current_real_effective_and_saved_set();
		
		let supplementary_group_identifiers = Groups::current_supplementary_group_identifiers();
		
		Self
		{
			real: GroupIdentifierDiagnostic::new(etc_path, real),
			effective: GroupIdentifierDiagnostic::new(etc_path, effective),
			saved_set: GroupIdentifierDiagnostic::new(etc_path, saved_set),
			supplementary:
			{
				let mut supplementary = Vec::with_capacity(supplementary_group_identifiers.len());
				for supplementary_group_identifiers in supplementary_group_identifiers
				{
					supplementary.push(GroupIdentifierDiagnostic::new(etc_path, supplementary_group_identifiers))
				}
				supplementary
			},
		}
	}
}
