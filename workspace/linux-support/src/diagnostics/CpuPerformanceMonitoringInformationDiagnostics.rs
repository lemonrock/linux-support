// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuPerformanceMonitoringInformationDiagnostics
{
	pub version_id: u8,
	
	pub number_of_counters: u8,
	
	pub counter_bit_width: u8,
	
	pub ebx_length: u8,
	
	pub fixed_function_counters: u8,
	
	pub fixed_function_counters_bit_width: u8,
	
	pub has_any_thread_deprecation: bool,
	
	pub is_core_cyc_ev_unavailable: bool,
	
	pub is_inst_ret_ev_unavailable: bool,
	
	pub is_ref_cycle_ev_unavailable: bool,
	
	pub is_cache_ref_ev_unavailable: bool,
	
	pub is_ll_cache_miss_ev_unavailable: bool,
	
	pub is_branch_inst_ret_ev_unavailable: bool,
	
	pub is_branch_midpred_ev_unavailable: bool,
}

impl CpuPerformanceMonitoringInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_performance_monitoring_info().map(|performance_monitoring_info| Self
		{
			version_id: performance_monitoring_info.version_id(),

			number_of_counters: performance_monitoring_info.number_of_counters(),

			counter_bit_width: performance_monitoring_info.counter_bit_width(),

			ebx_length: performance_monitoring_info.ebx_length(),

			fixed_function_counters: performance_monitoring_info.fixed_function_counters(),

			fixed_function_counters_bit_width: performance_monitoring_info.fixed_function_counters_bit_width(),

			has_any_thread_deprecation: performance_monitoring_info.has_any_thread_deprecation(),

			is_core_cyc_ev_unavailable: performance_monitoring_info.is_core_cyc_ev_unavailable(),

			is_inst_ret_ev_unavailable: performance_monitoring_info.is_inst_ret_ev_unavailable(),

			is_ref_cycle_ev_unavailable: performance_monitoring_info.is_ref_cycle_ev_unavailable(),

			is_cache_ref_ev_unavailable: performance_monitoring_info.is_cache_ref_ev_unavailable(),

			is_ll_cache_miss_ev_unavailable: performance_monitoring_info.is_ll_cache_miss_ev_unavailable(),

			is_branch_inst_ret_ev_unavailable: performance_monitoring_info.is_branch_inst_ret_ev_unavailable(),

			is_branch_midpred_ev_unavailable: performance_monitoring_info.is_branch_midpred_ev_unavailable(),
		})
	}
}
