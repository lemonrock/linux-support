// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuDeterministicAddressTranslationInformationDiagnostic
{
	pub cache_type: String,
	
	pub has_4k_entries: bool,
	
	pub has_2mb_entries: bool,
	
	pub has_4mb_entries: bool,
	
	pub has_1gb_entries: bool,
	
	pub is_fully_associative: bool,
	
	pub partitioning: u8,
	
	pub ways: u16,
	
	pub sets: u32,
	
	pub cache_level: u8,
	
	pub max_addressable_ids: u16,
}

impl CpuDeterministicAddressTranslationInformationDiagnostic
{
	fn gather(cpu_id: &CpuId) -> Option<Vec<Self>>
	{
		cpu_id.deterministic_address_translation_info().map(|iterator|
		{
			let mut items = Vec::new();
			for dat_info in iterator
			{
				use self::DatType::*;
				
				let cache_type = match dat_info.cache_type()
				{
					Null | Unknown => continue,
					DataTLB => "DataTLB",
					InstructionTLB => "InstructionTLB",
					UnifiedTLB => "UnifiedTLB",
				};
				
				items.push
				(
					Self
					{
						cache_type: cache_type.to_string(),
						
						has_4k_entries: dat_info.has_4k_entries(),
						
						has_2mb_entries: dat_info.has_2mb_entries(),
						
						has_4mb_entries: dat_info.has_4mb_entries(),
						
						has_1gb_entries: dat_info.has_1gb_entries(),
						
						is_fully_associative: dat_info.is_fully_associative(),
						
						partitioning: dat_info.partitioning(),
						
						ways: dat_info.ways(),
						
						sets: dat_info.sets(),
						
						cache_level: dat_info.cache_level(),
						
						max_addressable_ids: dat_info.max_addressable_ids(),
					}
				);
			}
			items
		})
	}
}
