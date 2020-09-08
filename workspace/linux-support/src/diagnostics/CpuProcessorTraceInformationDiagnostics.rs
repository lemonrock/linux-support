// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuProcessorTraceInformationDiagnostics
{
	pub has_rtit_cr3_match: bool,
	
	pub has_configurable_psb_and_cycle_accurate_mode: bool,
	
	pub has_ip_tracestop_filtering: bool,
	
	pub has_mtc_timing_packet_coefi_suppression: bool,
	
	pub has_ptwrite: bool,
	
	pub has_power_event_trace: bool,
	
	pub has_topa: bool,
	
	pub has_topa_maximum_entries: bool,
	
	pub has_single_range_output_scheme: bool,
	
	pub has_trace_transport_subsystem: bool,
	
	pub has_lip_with_cs_base: bool,
	
	pub configurable_address_ranges: u8,
	
	pub supported_mtc_period_encodings: u16,
	
	pub supported_cycle_threshold_value_encodings: u16,
	
	pub supported_psb_frequency_encodings: u16,
}

impl CpuProcessorTraceInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_processor_trace_info().map(|processor_trace_info| Self
		{
			has_rtit_cr3_match: processor_trace_info.has_rtit_cr3_match(),
			
			has_configurable_psb_and_cycle_accurate_mode: processor_trace_info.has_configurable_psb_and_cycle_accurate_mode(),
			
			has_ip_tracestop_filtering: processor_trace_info.has_ip_tracestop_filtering(),
			
			has_mtc_timing_packet_coefi_suppression: processor_trace_info.has_mtc_timing_packet_coefi_suppression(),
			
			has_ptwrite: processor_trace_info.has_ptwrite(),
			
			has_power_event_trace: processor_trace_info.has_power_event_trace(),
			
			has_topa: processor_trace_info.has_topa(),
			
			has_topa_maximum_entries: processor_trace_info.has_topa_maximum_entries(),
			
			has_single_range_output_scheme: processor_trace_info.has_single_range_output_scheme(),
			
			has_trace_transport_subsystem: processor_trace_info.has_trace_transport_subsystem(),
			
			has_lip_with_cs_base: processor_trace_info.has_lip_with_cs_base(),
			
			configurable_address_ranges: processor_trace_info.configurable_address_ranges(),
			
			supported_mtc_period_encodings: processor_trace_info.supported_mtc_period_encodings(),
			
			supported_cycle_threshold_value_encodings: processor_trace_info.supported_cycle_threshold_value_encodings(),
			
			supported_psb_frequency_encodings: processor_trace_info.supported_psb_frequency_encodings(),
		})
	}
}
