// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum scmp_compare
{
	#[doc(hidden)]
	_SCMP_CMP_MIN = 0,

	/// Not equal to.
	SCMP_CMP_NE = 1,

	/// Less than.
	SCMP_CMP_LT = 2,

	/// Less than or equal to.
	SCMP_CMP_LE = 3,

	/// Equal to.
	SCMP_CMP_EQ = 4,

	/// Greater than or equal to.
	SCMP_CMP_GE = 5,

	/// Greater than.
	SCMP_CMP_GT = 6,

	/// Masked equality.
	SCMP_CMP_MASKED_EQ = 7,

	#[doc(hidden)]
	_SCMP_CMP_MAX,
}
