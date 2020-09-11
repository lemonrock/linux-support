// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct CurrentProcessDiagnostics
{
	pub current_process_resource_limits: ResourceLimitsSet,
	
	pub current_process_miscellaneous_process_control: MiscellaneousProcessControlDiagnostics,
	
	pub current_process_personality: DiagnosticUnobtainableResult<PersonalityFlags>,
}

impl CurrentProcessDiagnostics
{
	fn gather() -> Self
	{
		Self
		{
			current_process_resource_limits: ResourceLimitsSet::current(),
			
			current_process_miscellaneous_process_control: MiscellaneousProcessControlDiagnostics::gather(),
			
			current_process_personality: PersonalityFlags::current().map_err(|_: ()| DiagnosticUnobtainable(format!("Personality flags unobtainable"))),
		}
	}
}
