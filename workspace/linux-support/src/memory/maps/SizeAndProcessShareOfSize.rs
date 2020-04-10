// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Size, and the process' share of size.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SizeAndProcessShareOfSize
{
	#[allow(missing_docs)]
	pub size: Kilobyte,

	/// Will never be larger than `size`.
	pub process_share_of_size: Kilobyte,
}

impl AddAssign<&Self> for SizeAndProcessShareOfSize
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: &Self)
	{
		self.size += rhs.size;
		self.process_share_of_size += rhs.process_share_of_size;
	}
}
