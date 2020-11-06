// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Hash)]
#[derive(EnumCount, EnumIter, IntoStaticStr)]
pub enum FaxEnumServiceSubType
{
	#[allow(missing_docs)]
	tel,
}

impl ToEnumUriScheme for FaxEnumServiceSubType
{
	#[inline(always)]
	fn to_uri_scheme(self) -> EnumUriScheme
	{
		use self::FaxEnumServiceSubType::*;
		
		match self
		{
			tel => EnumUriScheme::tel,
		}
	}
}

impl Default for FaxEnumServiceSubType
{
	#[inline(always)]
	fn default() -> Self
	{
		FaxEnumServiceSubType::tel
	}
}
