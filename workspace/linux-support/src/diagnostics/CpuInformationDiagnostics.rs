// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuInformationDiagnostics
{
	pub cpu_supports_1gb_pages: bool,
	
	pub maximum_logical_processor_identifiers_per_package: u8,
	
	pub vendor: Option<String>,
	
	pub feature_information: Option<CpuFeatureInformationDiagnostics>,
	
	pub caches_information: Option<Vec<CpuCacheInformationDiagnostic>>,

	pub cache_parameters: Option<Vec<Option<CpuCacheParameterInformationDiagnostic>>>,

	pub monitor_mwait_information: Option<CpuMonitorMwaitInformationDiagnostics>,

	pub thermal_power_information: Option<CpuThermalPowerInformationDiagnostics>,
	
	pub extended_feature_information: Option<CpuExtendedFeatureInformationDiagnostics>,
	
	pub direct_cache_access_cap_value: Option<u32>,
	
	pub performance_monitoring_information: Option<CpuPerformanceMonitoringInformationDiagnostics>,

	pub extended_topology: Option<Vec<CpuExtendedTopologyLevelInformationDiagnostics>>,
	
	pub extended_state_information: Option<CpuExtendedStateInformationDiagnostics>,
	
	pub intel_resource_director_technology_information: CpuIntelResourceDirectorTechnologyInformation,

	pub intel_sgx_information: Option<CpuIntelSgxInformationDiagnostics>,

	pub processor_trace_information: Option<CpuProcessorTraceInformationDiagnostics>,

	pub timestamp_counter_information: Option<CpuTimestampCounterInformationDiagnostics>,

	pub procesor_frequency_information: Option<CpuProcessorFrequencyInformationDiagnostics>,

	pub deterministic_address_translation_information: Option<Vec<CpuDeterministicAddressTranslationInformationDiagonostic>>,

	pub system_on_chip_vendor_information: Option<CpuSystemOnChipVendorInformationDiagnostics>,

	pub hypervisor_information: Option<CpuHypervisorInformationDiagnostics>,

	pub extended_function_information: Option<CpuExtendedFunctionInformationDiagnostics>,
	
	pub memory_encryption_information: Option<CpuMemoryEncryptionInformationDiagnostics>,
}

impl CpuInformationDiagnostics
{
	fn gather() -> Self
	{
		let cpu_id = CpuId::new();
		
		Self
		{
			cpu_supports_1gb_pages: cpu_supports_1gb_pages(),
			
			maximum_logical_processor_identifiers_per_package: maximum_logical_processor_identifiers_per_package(),
			
			feature_information: CpuFeatureInformationDiagnostics::gather(&cpu_id),
			
			vendor: cpu_id.get_vendor_info().map(|vendor_info| vendor_info.as_string().to_string()),
		
			caches_information: cpu_id.get_cache_info().map(|cache_info_iter|
			{
				let mut caches_information = Vec::new();
				for cache_info in cache_info_iter
				{
					caches_information.push(CpuCacheInformationDiagnostic::gather(cache_info));
				}
				caches_information
			}),
			
			cache_parameters: cpu_id.get_cache_parameters().map(|cache_parameter_iter|
			{
				let mut cache_parameters_information = Vec::new();
				for cache_parameter in cache_parameter_iter
				{
					cache_parameters_information.push(CpuCacheParameterInformationDiagnostic::gather(cache_parameter));
				}
				cache_parameters_information
			}),
			
			monitor_mwait_information: CpuMonitorMwaitInformationDiagnostics::gather(&cpu_id),
			
			thermal_power_information: CpuThermalPowerInformationDiagnostics::gather(&cpu_id),
			
			extended_feature_information: CpuExtendedFeatureInformationDiagnostics::gather(&cpu_id),
			
			direct_cache_access_cap_value: cpu_id.get_direct_cache_access_info().map(|value| value.get_dca_cap_value()),
			
			performance_monitoring_information: CpuPerformanceMonitoringInformationDiagnostics::gather(&cpu_id),
		
			extended_topology: cpu_id.get_extended_topology_info().map(|extended_topology_iter|
			{
				let mut extended_topology_information = Vec::new();
				for extended_topology_level in extended_topology_iter
				{
					extended_topology_information.push(CpuExtendedTopologyLevelInformationDiagnostics::gather(extended_topology_level));
				}
				extended_topology_information
			}),
			
			extended_state_information: CpuExtendedStateInformationDiagnostics::gather(&cpu_id),
		
			intel_resource_director_technology_information: CpuIntelResourceDirectorTechnologyInformation::gather(&cpu_id),
			
			intel_sgx_information: CpuIntelSgxInformationDiagnostics::gather(&cpu_id),
			
			processor_trace_information: CpuProcessorTraceInformationDiagnostics::gather(&cpu_id),
			
			timestamp_counter_information: CpuTimestampCounterInformationDiagnostics::gather(&cpu_id),
			
			procesor_frequency_information: CpuProcessorFrequencyInformationDiagnostics::gather(&cpu_id),
		
			deterministic_address_translation_information: CpuDeterministicAddressTranslationInformationDiagonostic::gather(&cpu_id),
			
			system_on_chip_vendor_information: CpuSystemOnChipVendorInformationDiagnostics::gather(&cpu_id),
			
			hypervisor_information: CpuHypervisorInformationDiagnostics::gather(&cpu_id),
			
			extended_function_information: CpuExtendedFunctionInformationDiagnostics::gather(&cpu_id),
			
			memory_encryption_information: CpuMemoryEncryptionInformationDiagnostics::gather(&cpu_id),
		}
	}
}
