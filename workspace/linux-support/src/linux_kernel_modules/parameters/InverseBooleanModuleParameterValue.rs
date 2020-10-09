// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Boolean where true is 'no'!
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u8)]
pub enum InverseBooleanModuleParameterValue
{
	No = 1,
	
	Yes = 0,
}

impl Default for InverseBooleanModuleParameterValue
{
	#[inline(always)]
	fn default() -> Self
	{
		InverseBooleanModuleParameterValue::Yes
	}
}

impl ModuleParameterValue for InverseBooleanModuleParameterValue
{
	#[inline(always)]
	fn parse_bytes(bytes: Box<[u8]>) -> io::Result<Self>
	{
		use self::InverseBooleanModuleParameterValue::*;
		
		match &bytes[..]
		{
			b"N" => Ok(No),
			
			b"Y" => Ok(Yes),
			
			_ => Err(io_error_invalid_data(ParseNumberError::OutOfRange))
		}
	}
	
	#[inline(always)]
	fn write_value(&self, extant_parameter_file_path: PathBuf) -> io::Result<()>
	{
		use self::InverseBooleanModuleParameterValue::*;
		
		let value = match self
		{
			No => b"N\n",
			
			Yes => b"Y\n",
		};
		extant_parameter_file_path.write_value(&value[..])
	}
}
