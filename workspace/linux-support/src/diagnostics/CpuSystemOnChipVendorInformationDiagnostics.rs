// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuSystemOnChipVendorInformationDiagnostics
{
	pub soc_vendor_id: u16,
	
	pub project_id: u32,
	
	pub stepping_id: u32,
	
	pub vendor_brand: String,
	
	pub vendor_attributes: Option<Vec<(u32, u32, u32, u32)>>,
}

impl CpuSystemOnChipVendorInformationDiagnostics
{
	#[inline(always)]
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_soc_vendor_info().map(|soc_vendor_info| Self
		{
			soc_vendor_id: soc_vendor_info.get_soc_vendor_id(),
			
			project_id: soc_vendor_info.get_project_id(),
			
			stepping_id: soc_vendor_info.get_stepping_id(),
			
			vendor_brand: soc_vendor_info.get_vendor_brand().as_string().to_string(),
			
			vendor_attributes: soc_vendor_info.get_vendor_attributes().map(|iterator|
			{
				let mut vendor_attributes = Vec::new();
				for cpu_id_result in iterator
				{
					vendor_attributes.push((cpu_id_result.eax, cpu_id_result.ebx, cpu_id_result.ecx, cpu_id_result.edx))
				}
				vendor_attributes
			}),
		})
	}
}
