// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A CString, but with a maximum size (including trailing nul terminator (`\0`) of 1024 bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct StringLinuxKernelModuleParameterValue(CString);

impl TryFrom<CString> for StringLinuxKernelModuleParameterValue
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: CString) -> Result<Self, Self::Error>
	{
		if value.as_byes_with_null().len() > Self::InclusiveMaximumLengthIncludingTrailingNulTerminator
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Self(value)
		}
	}
}

impl Into<CString> for StringLinuxKernelModuleParameterValue
{
	#[inline(always)]
	fn into(self) -> CString
	{
		self.0
	}
}

impl Deref for StringLinuxKernelModuleParameterValue
{
	type Target = CStr;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.as_c_str()
	}
}

impl StringLinuxKernelModuleParameterValue
{
	/// Maximum size.
	pub const InclusiveMaximumLengthIncludingTrailingNulTerminator: usize = 1024;
	
	const InclusiveMaximumLengthExcludingTrailingNulTerminator: usize = Self::InclusiveMaximumLengthIncludingTrailingNulTerminator - 1;
}

impl ModuleParameterValue for StringLinuxKernelModuleParameterValue
{
	#[inline(always)]
	fn parse_bytes(bytes: Box<[u8]>) -> io::Result<Self>
	{
		if bytes.len() > Self::InclusiveMaximumLengthExcludingTrailingNulTerminator
		{
			Err(io_error_invalid_data(ParseNumberError::TooLarge))
		}
		else
		{
			match CString::new(bytes)
			{
				Err(error) => Err(io_error_invalid_data(error)),
				
				Ok(c_string) => Ok(Self(c_string)),
			}
		}
	}
	
	#[inline(always)]
	fn write_value(&self, extant_parameter_file_path: PathBuf) -> io::Result<()>
	{
		use self::InverseBooleanModuleParameterValue::*;
		
		extant_parameter_file_path.write_value_then_line_feed(self.0.as_bytes())
	}
}
