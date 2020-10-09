// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Only supported for the Linux version of this driver, not the Intel supplied version!
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(i32)]
pub enum IntelIxgbevfDebugLevelModuleParameterValue
{
	/// Level 0.
	#[serde(rename = "0")] Off = 0,
	
	/// Level 1.
	#[serde(rename = "1")] _1 = 1,
	
	/// Level 2.
	#[serde(rename = "2")] _2 = 2,
	
	/// Level 3.
	#[serde(rename = "3")] _3 = 3,
	
	/// Level 4.
	#[serde(rename = "4")] _4 = 4,
	
	/// Level 5.
	#[serde(rename = "5")] _5 = 5,
	
	/// Level 6.
	#[serde(rename = "6")] _6 = 6,
	
	/// Level 7.
	#[serde(rename = "7")] _7 = 7,
	
	/// Level 8.
	#[serde(rename = "8")] _8 = 8,
	
	/// Level 9.
	#[serde(rename = "9")] _9 = 9,
	
	/// Level 10.
	#[serde(rename = "10")] _10 = 10,
	
	/// Level 11.
	#[serde(rename = "11")] _11 = 11,
	
	/// Level 12.
	#[serde(rename = "12")] _12 = 12,
	
	/// Level 13.
	#[serde(rename = "13")] _13 = 13,
	
	/// Level 14.
	#[serde(rename = "14")] _14 = 14,
	
	/// Level 15.
	#[serde(rename = "15")] _15 = 15,
	
	/// Level 16.
	#[serde(rename = "16")] All = 16,
}

impl Default for IntelIxgbevfDebugLevelModuleParameterValue
{
	#[inline(always)]
	fn default() -> Self
	{
		IntelIxgbevfDebugLevelModuleParameterValue::Off
	}
}

impl ModuleParameterValue for IntelIxgbevfDebugLevelModuleParameterValue
{
	#[inline(always)]
	fn parse_bytes(bytes: Box<[u8]>) -> io::Result<Self>
	{
		use self::IntelIxgbevfDebugLevelModuleParameterValue::*;
		
		let underlying = i32::parse_bytes(bytes)?;
		let parsed = if underlying > 16
		{
			All
		}
		else
		{
			unsafe { transmute(underlying as i32) }
		};
		Ok(parsed)
	}
	
	#[inline(always)]
	fn write_value(&self, extant_parameter_file_path: PathBuf) -> io::Result<()>
	{
		let value = (*self) as i32;
		value.write_value(extant_parameter_file_path)
	}
}
