// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuExtendedFeatureInformationDiagnostics
{
	pub has_fsgsbase: bool,
	
	pub has_tsc_adjust_msr: bool,
	
	pub has_bmi1: bool,
	
	pub has_hle: bool,
	
	pub has_avx2: bool,
	
	pub has_fdp: bool,
	
	pub has_smep: bool,
	
	pub has_bmi2: bool,
	
	pub has_rep_movsb_stosb: bool,
	
	pub has_invpcid: bool,
	
	pub has_rtm: bool,
	
	pub has_rdtm: bool,
	
	pub has_fpu_cs_ds_deprecated: bool,
	
	pub has_mpx: bool,
	
	pub has_rdta: bool,
	
	pub has_rdseed: bool,
	
	pub has_rdseet: bool,
	
	pub has_adx: bool,
	
	pub has_smap: bool,
	
	pub has_clflushopt: bool,
	
	pub has_processor_trace: bool,
	
	pub has_sha: bool,
	
	pub has_sgx: bool,
	
	pub has_avx512f: bool,
	
	pub has_avx512dq: bool,
	
	pub has_avx512_ifma: bool,
	
	pub has_avx512pf: bool,
	
	pub has_avx512er: bool,
	
	pub has_avx512cd: bool,
	
	pub has_avx512bw: bool,
	
	pub has_avx512vl: bool,
	
	pub has_clwb: bool,
	
	pub has_prefetchwt1: bool,
	
	pub has_umip: bool,
	
	pub has_pku: bool,
	
	pub has_ospke: bool,
	
	pub has_rdpid: bool,
	
	pub has_sgx_lc: bool,
	
	pub mawau_value: u8,
}

impl CpuExtendedFeatureInformationDiagnostics
{
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		cpu_id.get_extended_feature_info().map(|extended_features| Self
		{
			has_fsgsbase: extended_features.has_fsgsbase(),
			
			has_tsc_adjust_msr: extended_features.has_tsc_adjust_msr(),
			
			has_bmi1: extended_features.has_bmi1(),
			
			has_hle: extended_features.has_hle(),
			
			has_avx2: extended_features.has_avx2(),
			
			has_fdp: extended_features.has_fdp(),
			
			has_smep: extended_features.has_smep(),
			
			has_bmi2: extended_features.has_bmi2(),
			
			has_rep_movsb_stosb: extended_features.has_rep_movsb_stosb(),
			
			has_invpcid: extended_features.has_invpcid(),
			
			has_rtm: extended_features.has_rtm(),
			
			has_rdtm: extended_features.has_rdtm(),
			
			has_fpu_cs_ds_deprecated: extended_features.has_fpu_cs_ds_deprecated(),
			
			has_mpx: extended_features.has_mpx(),
			
			has_rdta: extended_features.has_rdta(),
			
			has_rdseed: extended_features.has_rdseed(),
			
			has_rdseet: extended_features.has_rdseet(),
			
			has_adx: extended_features.has_adx(),
			
			has_smap: extended_features.has_smap(),
			
			has_clflushopt: extended_features.has_clflushopt(),
			
			has_processor_trace: extended_features.has_processor_trace(),
			
			has_sha: extended_features.has_sha(),
			
			has_sgx: extended_features.has_sgx(),
			
			has_avx512f: extended_features.has_avx512f(),
			
			has_avx512dq: extended_features.has_avx512dq(),
			
			has_avx512_ifma: extended_features.has_avx512_ifma(),
			
			has_avx512pf: extended_features.has_avx512pf(),
			
			has_avx512er: extended_features.has_avx512er(),
			
			has_avx512cd: extended_features.has_avx512cd(),
			
			has_avx512bw: extended_features.has_avx512bw(),
			
			has_avx512vl: extended_features.has_avx512vl(),
			
			has_clwb: extended_features.has_clwb(),
			
			has_prefetchwt1: extended_features.has_prefetchwt1(),
			
			has_umip: extended_features.has_umip(),
			
			has_pku: extended_features.has_pku(),
			
			has_ospke: extended_features.has_ospke(),
			
			has_rdpid: extended_features.has_rdpid(),
			
			has_sgx_lc: extended_features.has_sgx_lc(),
			
			mawau_value: extended_features.mawau_value(),
		})
	}
}
			

