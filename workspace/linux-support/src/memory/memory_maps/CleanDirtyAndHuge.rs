// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Clean, dirty and huge page usage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CleanDirtyAndHuge
{
	#[allow(missing_docs)]
	pub clean: Kilobyte,

	#[allow(missing_docs)]
	pub dirty: Kilobyte,

	#[allow(missing_docs)]
	pub hugetlb: Kilobyte,
}

impl AddAssign<&Self> for CleanDirtyAndHuge
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: &Self)
	{
		self.clean += rhs.clean;
		self.dirty += rhs.dirty;
		self.hugetlb += rhs.hugetlb;
	}
}
