// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuExtendedFunctionInformationDiagnostics
{
	pub processor_brand_string: Option<String>,
	
	pub extended_signature: Option<u32>,
	
	pub cache_line_size: Option<u8>,
	
	pub l2_associativity: Option<String>,
	
	pub cache_size: Option<u16>,
	
	pub physical_address_bits: Option<u8>,
	
	pub linear_address_bits: Option<u8>,
	
	pub has_invariant_tsc: bool,
	
	pub has_lahf_sahf: bool,
	
	pub has_lzcnt: bool,
	
	pub has_prefetchw: bool,
	
	pub has_syscall_sysret: bool,
	
	pub has_execute_disable: bool,
	
	pub has_1gib_pages: bool,
	
	pub has_rdtscp: bool,
	
	pub has_64bit_mode: bool,
}

impl CpuExtendedFunctionInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		use self::L2Associativity::*;
		
		cpu_id.get_extended_function_info().map(|extended_function_info| Self
		{
			processor_brand_string: extended_function_info.processor_brand_string().map(|value| value.to_string()),
			
			extended_signature: extended_function_info.extended_signature(),
			
			cache_line_size: extended_function_info.cache_line_size(),
			
			l2_associativity: extended_function_info.l2_associativity().map(|value| match value
			{
				Disabled => "Disabled",
				DirectMapped => "DirectMapped",
				TwoWay => "TwoWay",
				FourWay => "FourWay",
				EightWay => "EightWay",
				SixteenWay => "SixteenWay",
				FullyAssiciative => "FullyAssociative",
				Unknown => "Unknown",
			}.to_string()),
			
			cache_size: extended_function_info.cache_size(),
			
			physical_address_bits: extended_function_info.physical_address_bits(),
			
			linear_address_bits: extended_function_info.linear_address_bits(),
			
			has_invariant_tsc: extended_function_info.has_invariant_tsc(),
			
			has_lahf_sahf: extended_function_info.has_lahf_sahf(),
			
			has_lzcnt: extended_function_info.has_lzcnt(),
			
			has_prefetchw: extended_function_info.has_prefetchw(),
			
			has_syscall_sysret: extended_function_info.has_syscall_sysret(),
			
			has_execute_disable: extended_function_info.has_execute_disable(),
			
			has_1gib_pages: extended_function_info.has_1gib_pages(),
			
			has_rdtscp: extended_function_info.has_rdtscp(),
			
			has_64bit_mode: extended_function_info.has_64bit_mode(),
		
	})
	}
}
			

