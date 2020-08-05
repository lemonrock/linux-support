// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Socket priority used when congestion occurs.
///
/// Only effective if the chosen queuing discipline (`QueuingDisciplineAlgorithm` / `qdisc`) supports the concept of priority.
///
/// For example, `bfifo` and `pfifo` do not use the concept of priority.
/// For example, `pfifo_fast` does.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum QueuingDisciplineSendPriority
{
	#[serde(rename = "0")] _0 = 0,
	
	#[serde(rename = "1")] _1 = 1,
	
	#[serde(rename = "2")] _2 = 2,
	
	#[serde(rename = "3")] _3 = 3,
	
	#[serde(rename = "4")] _4 = 4,
	
	#[serde(rename = "5")] _5 = 5,
	
	#[serde(rename = "6")] _6 = 6,
}

impl Default for QueuingDisciplineSendPriority
{
	#[inline(always)]
	fn default() -> Self
	{
		QueuingDisciplineSendPriority::_0
	}
}
