// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuFeatureInformationDiagnostics
{
	pub extended_family_id: u8,
	
	pub extended_model_id: u8,
	
	pub family_id: u8,
	
	pub model_id: u8,
	
	pub stepping_id: u8,
	
	pub brand_index: u8,
	
	pub cflush_cache_line_size: u8,
	
	pub initial_local_apic_id: u8,
	
	pub max_logical_processor_ids: u8,
	
	pub has_sse3: bool,
	
	pub has_pclmulqdq: bool,
	
	pub has_ds_area: bool,
	
	pub has_monitor_mwait: bool,
	
	pub has_cpl: bool,
	
	pub has_vmx: bool,
	
	pub has_smx: bool,
	
	pub has_eist: bool,
	
	pub has_tm2: bool,
	
	pub has_ssse3: bool,
	
	pub has_cnxtid: bool,
	
	pub has_fma: bool,
	
	pub has_cmpxchg16b: bool,
	
	pub has_pdcm: bool,
	
	pub has_pcid: bool,
	
	pub has_dca: bool,
	
	pub has_sse41: bool,
	
	pub has_sse42: bool,
	
	pub has_x2apic: bool,
	
	pub has_movbe: bool,
	
	pub has_popcnt: bool,
	
	pub has_tsc_deadline: bool,
	
	pub has_aesni: bool,
	
	pub has_xsave: bool,
	
	pub has_oxsave: bool,
	
	pub has_avx: bool,
	
	pub has_f16c: bool,
	
	pub has_rdrand: bool,
	
	pub has_hypervisor: bool,
	
	pub has_fpu: bool,
	
	pub has_vme: bool,
	
	pub has_de: bool,
	
	pub has_pse: bool,
	
	pub has_tsc: bool,
	
	pub has_msr: bool,
	
	pub has_pae: bool,
	
	pub has_mce: bool,
	
	pub has_cmpxchg8b: bool,
	
	pub has_apic: bool,
	
	pub has_sysenter_sysexit: bool,
	
	pub has_mtrr: bool,
	
	pub has_pge: bool,
	
	pub has_mca: bool,
	
	pub has_cmov: bool,
	
	pub has_pat: bool,
	
	pub has_pse36: bool,
	
	pub has_psn: bool,
	
	pub has_clflush: bool,
	
	pub has_ds: bool,
	
	pub has_acpi: bool,
	
	pub has_mmx: bool,
	
	pub has_fxsave_fxstor: bool,
	
	pub has_sse: bool,
	
	pub has_sse2: bool,
	
	pub has_ss: bool,
	
	pub has_htt: bool,
	
	pub has_tm: bool,
	
	pub has_pbe: bool,
}

impl CpuFeatureInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_feature_info().map(|feature_info| Self
		{
			extended_family_id: feature_info.extended_family_id(),
			
			extended_model_id: feature_info.extended_model_id(),
			
			family_id: feature_info.family_id(),
			
			model_id: feature_info.model_id(),
			
			stepping_id: feature_info.stepping_id(),
			
			brand_index: feature_info.brand_index(),
			
			cflush_cache_line_size: feature_info.cflush_cache_line_size(),
			
			initial_local_apic_id: feature_info.initial_local_apic_id(),
			
			max_logical_processor_ids: feature_info.max_logical_processor_ids(),

			has_sse3: feature_info.has_sse3(),

			has_pclmulqdq: feature_info.has_pclmulqdq(),

			has_ds_area: feature_info.has_ds_area(),

			has_monitor_mwait: feature_info.has_monitor_mwait(),

			has_cpl: feature_info.has_cpl(),

			has_vmx: feature_info.has_vmx(),

			has_smx: feature_info.has_smx(),

			has_eist: feature_info.has_eist(),

			has_tm2: feature_info.has_tm2(),

			has_ssse3: feature_info.has_ssse3(),

			has_cnxtid: feature_info.has_cnxtid(),

			has_fma: feature_info.has_fma(),

			has_cmpxchg16b: feature_info.has_cmpxchg16b(),

			has_pdcm: feature_info.has_pdcm(),

			has_pcid: feature_info.has_pcid(),

			has_dca: feature_info.has_dca(),

			has_sse41: feature_info.has_sse41(),

			has_sse42: feature_info.has_sse42(),

			has_x2apic: feature_info.has_x2apic(),

			has_movbe: feature_info.has_movbe(),

			has_popcnt: feature_info.has_popcnt(),

			has_tsc_deadline: feature_info.has_tsc_deadline(),

			has_aesni: feature_info.has_aesni(),

			has_xsave: feature_info.has_xsave(),

			has_oxsave: feature_info.has_oxsave(),

			has_avx: feature_info.has_avx(),

			has_f16c: feature_info.has_f16c(),

			has_rdrand: feature_info.has_rdrand(),

			has_hypervisor: feature_info.has_hypervisor(),

			has_fpu: feature_info.has_fpu(),

			has_vme: feature_info.has_vme(),

			has_de: feature_info.has_de(),

			has_pse: feature_info.has_pse(),

			has_tsc: feature_info.has_tsc(),

			has_msr: feature_info.has_msr(),

			has_pae: feature_info.has_pae(),

			has_mce: feature_info.has_mce(),

			has_cmpxchg8b: feature_info.has_cmpxchg8b(),

			has_apic: feature_info.has_apic(),

			has_sysenter_sysexit: feature_info.has_sysenter_sysexit(),

			has_mtrr: feature_info.has_mtrr(),

			has_pge: feature_info.has_pge(),

			has_mca: feature_info.has_mca(),

			has_cmov: feature_info.has_cmov(),

			has_pat: feature_info.has_pat(),

			has_pse36: feature_info.has_pse36(),

			has_psn: feature_info.has_psn(),

			has_clflush: feature_info.has_clflush(),

			has_ds: feature_info.has_ds(),

			has_acpi: feature_info.has_acpi(),

			has_mmx: feature_info.has_mmx(),

			has_fxsave_fxstor: feature_info.has_fxsave_fxstor(),

			has_sse: feature_info.has_sse(),

			has_sse2: feature_info.has_sse2(),

			has_ss: feature_info.has_ss(),

			has_htt: feature_info.has_htt(),

			has_tm: feature_info.has_tm(),

			has_pbe: feature_info.has_pbe(),
		})
	}
}
