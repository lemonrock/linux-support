// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuExtendedStateInformationDiagnostics
{
	pub xcr0_supports_legacy_x87: bool,
	
	pub xcr0_supports_sse_128: bool,
	
	pub xcr0_supports_avx_256: bool,
	
	pub xcr0_supports_mpx_bndregs: bool,
	
	pub xcr0_supports_mpx_bndcsr: bool,
	
	pub xcr0_supports_avx512_opmask: bool,
	
	pub xcr0_supports_avx512_zmm_hi256: bool,
	
	pub xcr0_supports_avx512_zmm_hi16: bool,
	
	pub xcr0_supports_pkru: bool,
	
	pub ia32_xss_supports_pt: bool,
	
	pub ia32_xss_supports_hdc: bool,
	
	pub xsave_area_size_enabled_features: bool,
	
	pub xsave_area_size_supported_features: bool,
	
	pub has_xsaveopt: bool,
	
	pub has_xsavec: bool,
	
	pub has_xgetbv: bool,
	
	pub has_xsaves_xrstors: bool,
	
	pub xsave_size: bool,

	pub higher_extended_states_information: Vec<CpuHigherExtendedStateInformationDiagnostics>,
}

impl CpuExtendedStateInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_extended_state_info().map(|extended_state_info| Self
		{
			xcr0_supports_legacy_x87: extended_state_info.xcr0_supports_legacy_x87(),
			
			xcr0_supports_sse_128: extended_state_info.xcr0_supports_sse_128(),
			
			xcr0_supports_avx_256: extended_state_info.xcr0_supports_avx_256(),
			
			xcr0_supports_mpx_bndregs: extended_state_info.xcr0_supports_mpx_bndregs(),
			
			xcr0_supports_mpx_bndcsr: extended_state_info.xcr0_supports_mpx_bndcsr(),
			
			xcr0_supports_avx512_opmask: extended_state_info.xcr0_supports_avx512_opmask(),
			
			xcr0_supports_avx512_zmm_hi256: extended_state_info.xcr0_supports_avx512_zmm_hi256(),
			
			xcr0_supports_avx512_zmm_hi16: extended_state_info.xcr0_supports_avx512_zmm_hi16(),
			
			xcr0_supports_pkru: extended_state_info.xcr0_supports_pkru(),
			
			ia32_xss_supports_pt: extended_state_info.ia32_xss_supports_pt(),
			
			ia32_xss_supports_hdc: extended_state_info.ia32_xss_supports_hdc(),
			
			xsave_area_size_enabled_features: extended_state_info.xsave_area_size_enabled_features(),
			
			xsave_area_size_supported_features: extended_state_info.xsave_area_size_supported_features(),
			
			has_xsaveopt: extended_state_info.has_xsaveopt(),
			
			has_xsavec: extended_state_info.has_xsavec(),
			
			has_xgetbv: extended_state_info.has_xgetbv(),
			
			has_xsaves_xrstors: extended_state_info.has_xsaves_xrstors(),
			
			xsave_size: extended_state_info.xsave_size(),
		
			higher_extended_states_information:
			{
				let mut higher_extended_states = Vec::new();
				for extended_state in extended_state_info.iter()
				{
					higher_extended_states.push(CpuHigherExtendedStateInformationDiagnostics::gather(extended_state));
				}
				higher_extended_states
			}
		})
	}
}


