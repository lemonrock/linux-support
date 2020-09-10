// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuMemoryEncryptionInformationDiagnostics
{
	pub has_sme: bool,
	
	pub has_sev: bool,
	
	pub has_page_flush_msr: bool,
	
	pub has_sev_es: bool,
	
	pub physical_address_reduction: u8,
	
	pub c_bit_position: u8,
	
	pub max_encrypted_guests: u32,
	
	pub min_sev_no_es_asid: u32,
}

impl CpuMemoryEncryptionInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_memory_encryption_info().map(|memory_encryption_info| Self
		{
			has_sme: memory_encryption_info.has_sme(),
			
			has_sev: memory_encryption_info.has_sev(),
			
			has_page_flush_msr: memory_encryption_info.has_page_flush_msr(),
			
			has_sev_es: memory_encryption_info.has_sev_es(),
			
			physical_address_reduction: memory_encryption_info.physical_address_reduction(),
			
			c_bit_position: memory_encryption_info.c_bit_position(),
			
			max_encrypted_guests: memory_encryption_info.max_encrypted_guests(),
			
			min_sev_no_es_asid: memory_encryption_info.min_sev_no_es_asid(),
		})
	}
}
			

