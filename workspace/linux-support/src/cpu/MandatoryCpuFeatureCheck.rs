// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A mandatory CPU feature check.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumMessage, EnumIter, IntoStaticStr)]
pub enum MandatoryCpuFeatureCheck
{
	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "General", detailed_message = "CPU architecture does not support 64-bit")]
	has_64bit_mode,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "General", detailed_message = "CPU architecture does not support 64-bit CAS")]
	has_cmpxchg8b,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "General", detailed_message = "CPU architecture does not support 64-bit fast syscalls")]
	has_syscall_sysret,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "General", detailed_message = "CPU architecture does not support the legacy CMOV instruction")]
	has_cmov,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "General", detailed_message = "CPU architecture does not support the CMPXCHG16B (128-bit CAS) instruction")]
	has_cmpxchg16b,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "General", detailed_message = "CPU architecture does not support the PCLMULQDQ instruction")]
	has_pclmulqdq,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "General", detailed_message = "CPU architecture does not have Read Model Specific Register (RDMSR) and Write Model Specific Register WRMSR")]
	has_msr,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "General", detailed_message = "CPU architecture does not have 'fsgsbase' instructions RDFSBASE, RDGSBASE, WRFSBASE and WRGSBASE")]
	has_fsgsbase,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "General", detailed_message = "CPU architecture does not have Direct Cache Access (DCA) for DMA writes")]
	has_dca,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Security-related", detailed_message = "CPU architecture does not support PCID (Essential for Meltdown vulnerability protection)")]
	has_pcid,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Security-related", detailed_message = "CPU architecture does not support Supervisor Model Execution Protection (SMEP)")]
	has_smep,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Security-related", detailed_message = "CPU architecture does not support (or does not have enabled) execute-disable bit (this may be due to Intel VT-d being disabled in the BIOS)")]
	has_execute_disable,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Security-related", detailed_message = "CPU architecture does not support (or does not have enabled) LAHF / SAHF (this may be due to Intel VT-d being disabled in the BIOS)")]
	has_lahf_sahf,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Memory and huge pages", detailed_message = "CPU architecture does not support Page Size Extensions; ie does not support huge pages")]
	has_pse,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Memory and huge pages", detailed_message = "CPU architecture does not support 36-Bit Page Size Extension; ie does not support huge pages")]
	has_pse36,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Memory and huge pages", detailed_message = "CPU architecture does not support Physical Address Extension; ie does not support huge pages")]
	has_pae,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Memory and huge pages", detailed_message = "CPU architecture does not support Page Global Bit")]
	has_pge,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Memory and huge pages", detailed_message = "CPU architecture does not support Page Attribute Table")]
	has_pat,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (x87 Floating Point)")]
	has_fpu,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (FXSAVE and FXRSTOR instructions)")]
	has_fxsave_fxstor,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (MMX)")]
	has_mmx,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (SSE)")]
	has_sse,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (SSE2)")]
	has_sse2,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (CLFLUSH instruction)")]
	has_clflush,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (SSE3)")]
	has_sse3,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (Suplemental SSE3, aka SSSE3)")]
	has_ssse3,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (MONITOR and MWAIT instructions)")]
	has_monitor_mwait,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (SSE4.1)")]
	has_sse41,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (SSE4.2)")]
	has_sse42,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Floating point and SIMD instruction sets", detailed_message = "CPU architecture does not support essential SIMD (POPCNT instruction)")]
	has_popcnt,

	// Timing related.
	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Timing related", detailed_message = "CPU architecture does not have an Advanced Programmable Interrupt Controller (APIC)")]
	has_apic,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Timing related", detailed_message = "CPU architecture does not have an x2 Advanced Programmable Interrupt Controller (x2APIC)")]
	has_x2apic,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Timing related", detailed_message = "CPU architecture does not have Thermal Monitor and Software Controlled Clock Facilities (ACPI)")]
	has_acpi,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Timing related", detailed_message = "CPU architecture does not support Time Stamp Counter (TSC)")]
	has_tsc,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Timing related", detailed_message = "CPU architecture does not support Time Stamp Counter (TSC) deadline timer")]
	has_tsc_deadline,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Timing related", detailed_message = "CPU architecture does not support (or does not have enabled) Read Time Stamp Counter and Processor ID (RDTSCP)")]
	has_rdtscp,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "Timing related", detailed_message = "CPU architecture does not support (or does not have enabled) invariant Time Stamp Counter (TSC)")]
	has_invariant_tsc,
}

#[cfg(target_arch = "x86_64")]
impl Check for MandatoryCpuFeatureCheck
{
	const Name: &'static str = "Mandatory CPU feature check";

	type CheckArguments = (FeatureInfo, ExtendedFunctionInfo, ExtendedFeatures);

	#[inline(always)]
	fn check(self, check_arguments: &Self::CheckArguments) -> bool
	{
		let &(ref feature_information, ref extended_function_information, ref extended_feature_information, ) = check_arguments;

		use self::MandatoryCpuFeatureCheck::*;

		match self
		{
			has_64bit_mode => extended_function_information.has_64bit_mode(),
			has_cmpxchg8b => feature_information.has_cmpxchg8b(),
			has_syscall_sysret => extended_function_information.has_syscall_sysret(),
			has_cmov => feature_information.has_cmov(),
			has_cmpxchg16b => feature_information.has_cmpxchg16b(),
			has_pclmulqdq => feature_information.has_pclmulqdq(),
			has_msr => feature_information.has_msr(),
			has_fsgsbase => extended_feature_information.has_fsgsbase(),
			has_dca => feature_information.has_dca(),
			has_pcid => feature_information.has_pcid(),
			has_smep => extended_feature_information.has_smep(),
			has_execute_disable => extended_function_information.has_execute_disable(),
			has_lahf_sahf => extended_function_information.has_lahf_sahf(),
			has_pse => feature_information.has_pse(),
			has_pse36 => feature_information.has_pse36(),
			has_pae => feature_information.has_pae(),
			has_pge => feature_information.has_pge(),
			has_pat => feature_information.has_pat(),
			has_fpu => feature_information.has_fpu(),
			has_fxsave_fxstor => feature_information.has_fxsave_fxstor(),
			has_mmx => feature_information.has_mmx(),
			has_sse => feature_information.has_sse(),
			has_sse2 => feature_information.has_sse2(),
			has_clflush => feature_information.has_clflush(),
			has_sse3 => feature_information.has_sse3(),
			has_ssse3 => feature_information.has_ssse3(),
			has_monitor_mwait => feature_information.has_monitor_mwait(),
			has_sse41 => feature_information.has_sse41(),
			has_sse42 => feature_information.has_sse42(),
			has_popcnt => feature_information.has_popcnt(),
			has_apic => feature_information.has_apic(),
			has_x2apic => feature_information.has_x2apic(),
			has_acpi => feature_information.has_acpi(),
			has_tsc => feature_information.has_tsc(),
			has_tsc_deadline => feature_information.has_tsc_deadline(),
			has_rdtscp => extended_function_information.has_rdtscp(),
			has_invariant_tsc => extended_function_information.has_invariant_tsc(),
		}
	}
}

#[cfg(not(target_arch = "x86_64"))]
impl Check for MandatoryCpuFeatureCheck
{
	type CheckArguments = ();

	#[inline(always)]
	fn check(self, check_arguments: &Self::CheckArguments) -> bool
	{
		true
	}
}
