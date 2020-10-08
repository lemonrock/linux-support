// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parameter name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct LinuxKernelModuleParameterName(CString);

impl From<CString> for LinuxKernelModuleParameterName
{
	#[inline(always)]
	fn from(value: CString) -> Self
	{
		LinuxKernelModuleParameterName(value)
	}
}

impl Into<CString> for LinuxKernelModuleParameterName
{
	#[inline(always)]
	fn into(self) -> CString
	{
		self.0
	}
}

impl Deref for LinuxKernelModuleParameterName
{
	type Target = CStr;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.as_c_str()
	}
}
