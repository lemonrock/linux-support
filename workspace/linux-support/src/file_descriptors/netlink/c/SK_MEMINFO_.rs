// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[derive(EnumIter, EnumCount)]
pub enum SK_MEMINFO_
{
	#[serde(rename = "Receive Memory Allocated")] SK_MEMINFO_RMEM_ALLOC = 0,
	
	#[serde(rename = "Receive Buffer")] SK_MEMINFO_RCVBUF = 1,
	
	#[serde(rename = "Write Memory Allocated")] SK_MEMINFO_WMEM_ALLOC = 2,
	
	#[serde(rename = "Send Buffer")] SK_MEMINFO_SNDBUF = 3,
	
	#[serde(rename = "Forward Allocated")] SK_MEMINFO_FWD_ALLOC = 4,
	
	#[serde(rename = "Write Memory Queued")] SK_MEMINFO_WMEM_QUEUED = 5,
	
	#[serde(rename = "Opt Memory")] SK_MEMINFO_OPTMEM = 6,
	
	/// May not be present.
	#[serde(rename = "Backlog")] SK_MEMINFO_BACKLOG = 7,
	
	/// May not be present.
	#[serde(rename = "Drops")] SK_MEMINFO_DROPS = 8,
}

impl SK_MEMINFO_
{
	#[allow(dead_code)]
	const SK_MEMINFO_VARS: usize = Self::COUNT;
}
