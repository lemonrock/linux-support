// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A compiled CPU feature check.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumMessage, EnumIter, IntoStaticStr)]
#[serde(deny_unknown_fields)]
pub enum CompiledCpuFeatureCheck
{
	#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "AVX instructions compiled for, but not supported by, CPU")]
	has_avx,

	#[cfg(all(target_arch = "x86_64", target_feature = "f16c"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "F16C instructions compiled for, but not supported by, CPU")]
	has_f16c,

	#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "AVX2 instructions compiled for, but not supported by, CPU")]
	has_avx2,

	#[cfg(all(target_arch = "x86_64", target_feature = "has_fma"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "FMA3 instructions compiled for, but not supported by, CPU")]
	has_fma,

	#[cfg(all(target_arch = "x86_64", target_feature = "hle"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "HLE transaction memory extension (TSX) instructions compiled for, but not supported by, CPU")]
	has_hle,

	#[cfg(all(target_arch = "x86_64", target_feature = "rtm"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "RTM transaction memory extension (TSX) instructions compiled for, but not supported by, CPU")]
	has_rtm,

	#[cfg(all(target_arch = "x86_64", target_feature = "bmi1"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "BMI1 instructions compiled for, but not supported by, CPU")]
	has_bmi1,

	#[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "BMI2 instructions compiled for, but not supported by, CPU")]
	has_bmi2,

	#[cfg(all(target_arch = "x86_64", target_feature = "adx"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "ADX instructions compiled for, but not supported by, CPU")]
	has_adx,

	// Properly `target_feature = "rdrand"`, but not correctly encoded by Rust.
	#[cfg(all(target_arch = "x86_64", target_feature = "rdrand"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "RDRAND instruction compiled for, but not supported by, CPU")]
	has_rdrand,

	#[cfg(all(target_arch = "x86_64", target_feature = "rdseed"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "RDSEED instruction compiled for, but not supported by, CPU")]
	has_rdseed,

	#[cfg(all(target_arch = "x86_64", target_feature = "movbe"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "MOVBE instruction compiled for, but not supported by, CPU")]
	has_movbe,

	#[cfg(all(target_arch = "x86_64", target_feature = "lzcnt"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "LZCNT instruction compiled for, but not supported by, CPU")]
	has_lzcnt,

	#[cfg(all(target_arch = "x86_64", target_feature = "clflushopt"))]
	#[allow(missing_docs)]
	#[strum(message = "compiled_feature", detailed_message = "CLFLUSHOPT instruction compiled for, but not supported by, CPU")]
	has_clflushopt,

	#[doc(hidden)]
	_dummy_to_fix_bug_in_EnumMessage_macro,
}

#[cfg(target_arch = "x86_64")]
impl Check for CompiledCpuFeatureCheck
{
	const Name: &'static str = "Compiled CPU feature check";

	type CheckArguments = (FeatureInfo, ExtendedFunctionInfo, ExtendedFeatures);

	#[inline(always)]
	fn check(self, check_arguments: &Self::CheckArguments) -> bool
	{
		#[allow(unused_variables)] let &(ref feature_information, ref _extended_function_information, ref extended_feature_information) = check_arguments;

		#[allow(unused_imports)] use self::CompiledCpuFeatureCheck::*;

		match self
		{
			#[cfg(target_feature = "avx")] has_avx => feature_information.has_avx(),
			#[cfg(target_feature = "f16c")] has_f16c => feature_information.has_f16c(),
			#[cfg(target_feature = "avx2")] has_avx2 => extended_feature_information.has_avx2(),
			#[cfg(target_feature = "has_fma")] has_fma => feature_information.has_fma(),
			#[cfg(target_feature = "hle")] has_hle => feature_information.has_hle(),
			#[cfg(target_feature = "rtm")] has_rtm => feature_information.has_rtm(),
			#[cfg(target_feature = "bmi1")] has_bmi1 => feature_information.has_bmi1(),
			#[cfg(target_feature = "bmi2")] has_bmi2 => feature_information.has_bmi2(),
			#[cfg(target_feature = "adx")] has_adx => extended_feature_information.has_adx(),
			#[cfg(target_feature = "rdrand")] has_rdrand => feature_information.has_rdrand(), // Properly `target_feature = "rdrand"`, but not correctly encoded by Rust.
			#[cfg(target_feature = "rdseed")] has_rdseed => feature_information.has_rdseet(), // `has_rdseet()` (sic) - typo in `raw_cpuid` crate.
			#[cfg(target_feature = "movbe")] has_movbe => feature_information.has_movbe(),
			#[cfg(target_feature = "lzcnt")] has_lzcnt => feature_information.has_lzcnt(),
			#[cfg(target_feature = "clflushopt")] has_clflushopt => extended_feature_information.has_clflushopt(),
			_dummy_to_fix_bug_in_EnumMessage_macro => true,
		}
	}
}

#[cfg(not(target_arch = "x86_64"))]
impl Check for CompiledCpuFeatureCheck
{
	type CheckArguments = ();

	#[inline(always)]
	fn check(self, check_arguments: &Self::CheckArguments) -> bool
	{
		true
	}
}
