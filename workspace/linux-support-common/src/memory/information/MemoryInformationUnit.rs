// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory information unit.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryInformationUnit
{
	/// KiloByte.
	KiloByte,
	
	/// Numeric count.
	Count,
}

impl MemoryInformationUnit
{
	#[inline(always)]
	pub(crate) fn ends_with(&self) -> &'static str
	{
		use self::MemoryInformationUnit::*;

		match *self
		{
			KiloByte => " kB",
			Count => "",
		}
	}
}
