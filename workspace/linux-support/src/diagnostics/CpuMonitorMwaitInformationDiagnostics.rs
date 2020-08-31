// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuMonitorMwaitInformationDiagnostics
{
	pub smallest_monitor_line: u16,
	
	pub largest_monitor_line: u16,
	
	pub extensions_supported: bool,
	
	pub interrupts_as_break_event: bool,
	
	pub supported_c0_states: u16,
	
	pub supported_c1_states: u16,
	
	pub supported_c2_states: u16,
	
	pub supported_c3_states: u16,
	
	pub supported_c4_states: u16,
	
	pub supported_c5_states: u16,
	
	pub supported_c6_states: u16,
	
	pub supported_c7_states: u16,
}

impl CpuMonitorMwaitInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_monitor_mwait_info().map(|monitor_mwait_info| Self
		{
			smallest_monitor_line: monitor_mwait_info.smallest_monitor_line(),
			
			largest_monitor_line: monitor_mwait_info.largest_monitor_line(),
			
			extensions_supported: monitor_mwait_info.extensions_supported(),
			
			interrupts_as_break_event: monitor_mwait_info.interrupts_as_break_event(),
			
			supported_c0_states: monitor_mwait_info.supported_c0_states(),
			
			supported_c1_states: monitor_mwait_info.supported_c1_states(),
			
			supported_c2_states: monitor_mwait_info.supported_c2_states(),
			
			supported_c3_states: monitor_mwait_info.supported_c3_states(),
			
			supported_c4_states: monitor_mwait_info.supported_c4_states(),
			
			supported_c5_states: monitor_mwait_info.supported_c5_states(),
			
			supported_c6_states: monitor_mwait_info.supported_c6_states(),
			
			supported_c7_states: monitor_mwait_info.supported_c7_states(),
		})
	}
}
