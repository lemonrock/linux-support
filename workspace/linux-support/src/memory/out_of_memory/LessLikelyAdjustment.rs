// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i8)]
pub enum LessLikelyAdjustment
{
	#[serde(rename = "1")] _1 = -1,
	
	#[serde(rename = "2")] _2 = -2,
	
	#[serde(rename = "3")] _3 = -3,
	
	#[serde(rename = "4")] _4 = -4,
	
	#[serde(rename = "5")] _5 = -5,
	
	#[serde(rename = "6")] _6 = -6,
	
	#[serde(rename = "6")] _7 = -7,
	
	#[serde(rename = "6")] _8 = -8,
	
	#[serde(rename = "6")] _9 = -9,
	
	#[serde(rename = "6")] _10 = -10,
	
	#[serde(rename = "6")] _11 = -11,
	
	#[serde(rename = "6")] _12 = -12,
	
	#[serde(rename = "6")] _13 = -13,
	
	#[serde(rename = "6")] _14 = -14,
	
	#[serde(rename = "6")] _15 = -15,
	
	#[serde(rename = "6")] _16 = -16,
}
