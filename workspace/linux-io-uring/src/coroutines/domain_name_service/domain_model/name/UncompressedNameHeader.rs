// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
struct UncompressedNameHeader
{
	/// This *includes* the root label.
	number_of_labels: u8,

	/// This *includes* the root label.
	name_length: u8,
}

impl UncompressedNameHeader
{
	#[inline(always)]
	fn new(number_of_labels: u8, name_length: u8) -> Self
	{
		Self
		{
			number_of_labels,
			name_length,
		}
	}
}
