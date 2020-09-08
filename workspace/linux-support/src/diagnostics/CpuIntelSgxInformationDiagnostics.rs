// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuIntelSgxInformationDiagnostics
{
	pub has_sgx1: bool,
	
	pub has_sgx2: bool,
	
	pub has_enclv_leaves_einvirtchild_edecvirtchild_esetcontext: bool,
	
	pub has_encls_leaves_etrackc_erdinfo_eldbc_elduc: bool,
	
	pub miscselect: u32,
	
	pub max_enclave_size_non_64bit: u8,
	
	pub max_enclave_size_64bit: u8,
	
	pub secs_attributes: (u64, u64),

	pub enclave_page_cache_sections: Vec<CpuIntelEnclavePageCacheDiagnostic>,
}

impl CpuIntelSgxInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_sgx_info().map(|sgx_info| Self
		{
			has_sgx1: sgx_info.has_sgx1(),
			
			has_sgx2: sgx_info.has_sgx2(),
			
			has_enclv_leaves_einvirtchild_edecvirtchild_esetcontext: sgx_info.has_enclv_leaves_einvirtchild_edecvirtchild_esetcontext(),
			
			has_encls_leaves_etrackc_erdinfo_eldbc_elduc: sgx_info.has_encls_leaves_etrackc_erdinfo_eldbc_elduc(),
			
			miscselect: sgx_info.miscselect(),
			
			max_enclave_size_non_64bit: sgx_info.max_enclave_size_non_64bit(),
			
			max_enclave_size_64bit: sgx_info.max_enclave_size_64bit(),
			
			secs_attributes: sgx_info.secs_attributes(),
			
			enclave_page_cache_sections:
			{
				let mut enclave_page_cache_sections = Vec::new();
				
				for sgx_section_info in sgx_info.iter()
				{
					match sgx_section_info
					{
						SgxSectionInfo::Epc(epc_section) => enclave_page_cache_sections.push(CpuIntelEnclavePageCacheDiagnostic { physical_base: epc_section.physical_base(), size: epc_section.size() }),
					}
				}
				
				enclave_page_cache_sections
			}
		})
	}
}
