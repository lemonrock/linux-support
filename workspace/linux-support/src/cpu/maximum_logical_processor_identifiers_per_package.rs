// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Maximum logical processor identifiers (ie HyperThreads) per package (usually NUMA node).
#[cfg(target_arch = "x86_64")]
pub fn maximum_logical_processor_identifiers_per_package() -> u8
{
	CpuId::new().get_feature_info().unwrap().max_logical_processor_ids()
}
