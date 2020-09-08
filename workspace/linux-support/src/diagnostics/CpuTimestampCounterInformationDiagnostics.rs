// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuTimestampCounterInformationDiagnostics
{
	pub denominator: u32,
	
	pub numerator: u32,
	
	pub nominal_frequency: u32,
	
	pub tsc_frequency: Option<u64>,
}

impl CpuTimestampCounterInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_tsc_info().map(|tsc_info| Self
		{
			denominator: tsc_info.denominator(),
			
			numerator: tsc_info.numerator(),
			
			nominal_frequency: tsc_info.nominal_frequency(),
			
			tsc_frequency: tsc_info.tsc_frequency(),
		})
	}
}
