// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HugePageSizeMemoryDiagnostic
{
	pub can_have_a_dynamic_huge_page_pool: bool,
	
	pub is_a_gigantic_huge_page: bool,
	
	pub global_huge_page_pool_size: Option<GlobalHugePagePoolSize>,
	
	pub global_huge_page_pool_statistics: Option<HugePagePoolStatistics>,
	
	pub memory_policy_global_huge_pages: Option<u64>,
}

impl HugePageSizeMemoryDiagnostic
{
	fn gather(sys_path: &SysPath, huge_page_size: HugePageSize) -> Self
	{
		Self
		{
			can_have_a_dynamic_huge_page_pool: huge_page_size.can_have_a_dynamic_huge_page_pool(),
			is_a_gigantic_huge_page: huge_page_size.is_a_gigantic_huge_page(),
			global_huge_page_pool_size: huge_page_size.global_huge_page_pool_size(sys_path),
			global_huge_page_pool_statistics: huge_page_size.global_huge_page_pool_statistics(sys_path),
			memory_policy_global_huge_pages: huge_page_size.memory_policy_global_huge_pages(sys_path),
		}
	}
}
