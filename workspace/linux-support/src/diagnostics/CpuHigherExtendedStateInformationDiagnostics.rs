// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuHigherExtendedStateInformationDiagnostics
{
	pub subleaf: u32,
	
	pub size: u32,
	
	pub offset: u32,
	
	pub is_in_ia32_xss: bool,
	
	pub is_in_xcr0: bool,
	
	pub is_compacted_form: bool,
}

impl CpuHigherExtendedStateInformationDiagnostics
{
	fn gather(extended_state: ExtendedState) -> Self
	{
		Self
		{
			subleaf: extended_state.subleaf,
		
			size: extended_state.size(),
			
			offset: extended_state.offset(),
			
			is_in_ia32_xss: extended_state.is_in_ia32_xss(),
			
			is_in_xcr0: extended_state.is_in_xcr0(),
			
			is_compacted_form: extended_state.is_compacted_format(),
		}
	}
}
