// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuIntelResourceDirectorTechnologyInformation
{
	pub rmid_range: Option<u32>,

	pub layer_3_cache_monitoring: Option<CpuLayer3CacheMonitoringIntelResourceDirectorTechnologyInformation>,

	pub layer_2_cache_allocation: Option<CpuLayer3CacheAllocationIntelResourceDirectorTechnologyInformation>,

	pub layer_3_cache_allocation: Option<CpuLayer2CacheAllocationIntelResourceDirectorTechnologyInformation>,

	pub memory_bandwidth_allocation: Option<>,
}

impl CpuIntelResourceDirectorTechnologyInformation
{
	fn gather(cpu_id: &CpuId) -> Self
	{
		let (rmid_range, layer_3_cache_monitoring) = match cpu_id.get_rdt_monitoring_info()
		{
			None => (None, None),
			
			Some(rdt_monitoring_info) => match rdt_monitoring_info.l3_cache_monitoring()
			{
				None => (Some(rdt_monitoring_info.rmid_range), None),
				
				Some(l3_cache_monitoring_info) => (Some(rdt_monitoring_info.rmid_range), Some
				(
					CpuLayer3CacheMonitoringIntelResourceDirectorTechnologyInformation
					{
						cache_conversion_factor: l3_cache_monitoring_info.conversion_factor(),
					
						cache_maximum_rmid_range: l3_cache_monitoring_info.maximum_rmid_range(),
						
						cache_has_occupancy_monitoring: l3_cache_monitoring_info.has_occupancy_monitoring(),
						
						cache_has_total_bandwidth_monitoring: l3_cache_monitoring_info.has_total_bandwidth_monitoring(),
						
						cache_has_local_bandwidth_monitoring: l3_cache_monitoring_info.has_local_bandwidth_monitoring(),
					}
				))
			}
		};
		
		let (layer_2_cache_allocation, layer_3_cache_allocation, memory_bandwidth_allocation) = match cpu_id.get_rdt_allocation_info()
		{
			None => (None, None, None),
			
			Some(rdt_allocation_info) =>
			{
				(
					rdt_allocation_info.l2_cat().map(|l2_cat_info| CpuLayer2CacheAllocationIntelResourceDirectorTechnologyInformation
					{
						capacity_mask_length: l2_cat_info.capacity_mask_length(),
						
						isolation_bitmap: l2_cat_info.isolation_bitmap(),
						
						highest_cos: l2_cat_info.highest_cos(),
					}),
					
					rdt_allocation_info.l3_cat().map(|l3_cat_info| CpuLayer3CacheAllocationIntelResourceDirectorTechnologyInformation
					{
						capacity_mask_length: l3_cat_info.capacity_mask_length(),
						
						isolation_bitmap: l3_cat_info.isolation_bitmap(),
						
						highest_cos: l3_cat_info.highest_cos(),
					
						has_code_data_prioritization: l3_cat_info.has_code_data_prioritization(),
					}),
					
					rdt_allocation_info.memory_bandwidth_allocation.map(|mem_bw_allocation_info| CpuMemoryBandwidthAllocationIntelResourceDirectorTechnologyInformation
					{
						max_hba_throttling: mem_bw_allocation_info.max_hba_throttling(),
					
						highest_cos: mem_bw_allocation_info.highest_cos(),
					
						has_linear_response_delay: mem_bw_allocation_info.has_linear_response_delay(),
					})
				)
			}
		};
		
		Self
		{
			rmid_range,
			
			layer_3_cache_monitoring,
		
			layer_2_cache_allocation,
			
			layer_3_cache_allocation,
		
			memory_bandwidth_allocation,
		}
	}
}
