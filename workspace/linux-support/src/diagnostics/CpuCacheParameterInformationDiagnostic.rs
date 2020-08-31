// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuCacheParameterInformationDiagnostic
{
	pub cache_type: String,
	
	pub level: NonZeroU8,
	
	pub is_self_initializing: bool,
	
	pub is_fully_associative: bool,
	
	pub max_cores_for_cache: usize,
	
	pub max_cores_for_package: usize,
	
	pub coherency_line_size: usize,
	
	pub physical_line_partitions: usize,
	
	pub associativity: usize,
	
	pub sets: usize,
	
	pub is_write_back_invalidate: bool,
	
	pub is_inclusive: bool,
	
	pub has_complex_indexing: bool,
}

impl CpuCacheParameterInformationDiagnostic
{
	#[inline(always)]
	fn gather(cache_parameter_info: CacheParameter) -> Option<Self>
	{
		use self::CacheType::*;
		
		Some
		(
			Self
			{
				cache_type: match cache_parameter_info.cache_type()
				{
					Data => "Data",
					Instruction => "Data",
					Unified => "Unified",
					Null => return None,
					Reserved => return None,
				}.to_string(),
				
				level: match cache_parameter_info.level()
				{
					0 => return None,
					non_zero => unsafe { NonZeroU8::new_unchecked(non_zero) },
				},
				
				is_self_initializing: cache_parameter_info.is_self_initializing(),
			
				is_fully_associative: cache_parameter_info.is_fully_associative(),
				
				max_cores_for_cache: cache_parameter_info.max_cores_for_cache(),
				
				max_cores_for_package: cache_parameter_info.max_cores_for_package(),
				
				coherency_line_size: cache_parameter_info.coherency_line_size(),
				
				physical_line_partitions: cache_parameter_info.physical_line_partitions(),
				
				associativity: cache_parameter_info.associativity(),
				
				sets: cache_parameter_info.sets(),
				
				is_write_back_invalidate: cache_parameter_info.is_write_back_invalidate(),
				
				is_inclusive: cache_parameter_info.is_inclusive(),
				
				has_complex_indexing: cache_parameter_info.has_complex_indexing(),
			}
		)
	}
}
