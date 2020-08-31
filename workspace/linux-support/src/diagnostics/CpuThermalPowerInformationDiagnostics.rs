// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuThermalPowerInformationDiagnostics
{
	pub dts_irq_threshold: u8,
	
	pub has_dts: bool,
	
	pub has_turbo_boost: bool,
	
	pub has_arat: bool,
	
	pub has_pln: bool,
	
	pub has_ecmd: bool,
	
	pub has_ptm: bool,
	
	pub has_hwp: bool,
	
	pub has_hwp_notification: bool,
	
	pub has_hwp_energy_performance_preference: bool,
	
	pub has_hwp_package_level_request: bool,
	
	pub has_hdc: bool,
	
	pub has_turbo_boost3: bool,
	
	pub has_hwp_capabilities: bool,
	
	pub has_hwp_peci_override: bool,
	
	pub has_flexible_hwp: bool,
	
	pub has_hwp_fast_access_mode: bool,
	
	pub has_ignore_idle_processor_hwp_request: bool,
	
	pub has_hw_coord_feedback: bool,
	
	pub has_energy_bias_pref: bool,
}

impl CpuThermalPowerInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_thermal_power_info().map(|thermal_power_info| Self
		{
			dts_irq_threshold: thermal_power_info.dts_irq_threshold(),
			
			has_dts: thermal_power_info.has_dts(),
			
			has_turbo_boost: thermal_power_info.has_turbo_boost(),
			
			has_arat: thermal_power_info.has_arat(),
			
			has_pln: thermal_power_info.has_pln(),
			
			has_ecmd: thermal_power_info.has_ecmd(),
			
			has_ptm: thermal_power_info.has_ptm(),
			
			has_hwp: thermal_power_info.has_hwp(),
			
			has_hwp_notification: thermal_power_info.has_hwp_notification(),
			
			has_hwp_energy_performance_preference: thermal_power_info.has_hwp_energy_performance_preference(),
			
			has_hwp_package_level_request: thermal_power_info.has_hwp_package_level_request(),
			
			has_hdc: thermal_power_info.has_hdc(),
			
			has_turbo_boost3: thermal_power_info.has_turbo_boost3(),
			
			has_hwp_capabilities: thermal_power_info.has_hwp_capabilities(),
			
			has_hwp_peci_override: thermal_power_info.has_hwp_peci_override(),
			
			has_flexible_hwp: thermal_power_info.has_flexible_hwp(),
			
			has_hwp_fast_access_mode: thermal_power_info.has_hwp_fast_access_mode(),
			
			has_ignore_idle_processor_hwp_request: thermal_power_info.has_ignore_idle_processor_hwp_request(),
			
			has_hw_coord_feedback: thermal_power_info.has_hw_coord_feedback(),
			
			has_energy_bias_pref: thermal_power_info.has_energy_bias_pref(),
		})
	}
}
