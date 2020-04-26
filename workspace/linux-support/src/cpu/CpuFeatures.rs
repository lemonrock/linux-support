// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// CPU Features.
#[derive(Debug)]
pub struct CpuFeatures
{
	/// ie per NUMA socket.
	#[cfg(target_arch = "x86_64")] pub maximum_logical_processor_identifiers_per_package: u8,

	/// Has 1Gb Huge Pages.
	#[cfg(target_arch = "x86_64")] pub has_1gb_huge_pages: bool,
}

impl CpuFeatures
{
	/// New instance.
	#[inline(always)]
	pub fn new() -> Self
	{
		#[cfg(target_arch = "x86_64")]
		{
			let cpu_id = CpuId::new();
			return Self
			{
				maximum_logical_processor_identifiers_per_package: cpu_id.get_feature_info().unwrap().max_logical_processor_ids(),
				has_1gb_huge_pages: cpu_id.get_extended_function_info().unwrap().has_1gib_pages()
			}
		}
		#[cfg(not(target_arch = "x86_64"))]
		{
			return Self
			{
			}
		}
	}
}
