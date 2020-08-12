// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A cgroup name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct CgroupName(OsString);

impl TryFrom<OsString> for CgroupName
{
	type Error = &'static str;
	
	#[inline(always)]
	fn try_from(value: OsString) -> Result<Self, Self::Error>
	{
		if unlikely!(value.is_empty())
		{
			return Err("Can not be empty")
		}
		
		let bytes = value.as_bytes();
		match memchr2(b'/', b'.', bytes)
		{
			None => Ok(Self(value)),
			
			Some(index) => match unsafe { *value.get_unchecked(index) }
			{
				b'/' => Err("Can not contain the directory separator '/'"),
				
				b'.' => if Controller.is_controller(&value[.. index])
				{
					Err("Can not use a prefix reserved for a controller (this isn't a perfect check)")
				}
				else
				{
					Ok(Self(value))
				}
			}
		}
	}
}

impl AsRef<Path> for CgroupName
{
	#[inline(always)]
	fn as_ref(&self) -> &Path
	{
		self.0.as_ref()
	}
}
