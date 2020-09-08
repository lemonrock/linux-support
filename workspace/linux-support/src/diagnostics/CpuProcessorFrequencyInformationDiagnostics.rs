// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuProcessorFrequencyInformationDiagnostics
{
	pub processor_base_frequency: u16,
	
	pub processor_max_frequency: u16,
	
	pub bus_frequency: u16,
}

impl CpuProcessorFrequencyInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_processor_frequency_info().map(|processor_frequency_info| Self
		{
			processor_base_frequency: processor_frequency_info.processor_base_frequency(),
			
			processor_max_frequency: processor_frequency_info.processor_max_frequency(),
			
			bus_frequency: processor_frequency_info.bus_frequency(),
		})
	}
}
