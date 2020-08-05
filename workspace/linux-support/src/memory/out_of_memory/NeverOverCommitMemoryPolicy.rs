// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Policy used for never commmit memory.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NeverOverCommitMemoryPolicy
{
	/// Do not overcommit this amount of physical RAM + swap.
	NumberOfPhysicalRamKilobytes(Kilobytes),
	
	/// Do not overcommit this percentage of physical RAM + swap.
	///
	/// Linux defaults to `Self:DefaultPercentageOfPhysicalRamBytes`.
	/// Why this is signed and so large is beyond me.
	PercentageOfPhysicalRamKilobytes(Percentage),
}

impl Default for NeverOverCommitMemoryPolicy
{
	#[inline(always)]
	fn default() -> Self
	{
		NeverOverCommitMemoryPolicy::PercentageOfPhysicalRamKilobytes(Self::DefaultPercentageOfPhysicalRamBytes)
	}
}

impl NeverOverCommitMemoryPolicy
{
	const DefaultPercentageOfPhysicalRamBytes: Percentage = Percentage(50);
}
