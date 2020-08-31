// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuCacheInformationDiagnostic
{
	pub num: u8,
	
	pub description: String,
	
	pub type_: String,
}

impl CpuCacheInformationDiagnostic
{
	#[inline(always)]
	fn gather(cache_info: CacheInfo) -> Self
	{
		use self::CacheInfoType::*;
		
		Self
		{
			num: cache_info.num,
			
			description: cache_info.desc().to_string(),
			
			type_: match cache_info.typ
			{
				General => "General",
				Cache => "Cache",
				TLB => "Cache",
				STLB => "STLB",
				DTLB => "DTLB",
				Prefetch => "Prefetch",
			}.to_string(),
		}
	}
}
