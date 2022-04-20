// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An error.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum IntoArrayError<T: Debug, const N: usize>
{
	#[allow(missing_docs)]
	HasSpilledToHeap(ConstSmallVec<T, N>),
	
	#[allow(missing_docs)]
	StackTooShort(ConstSmallVec<T, N>),
}

impl<T: Debug, const N: usize> Display for IntoArrayError<T, N>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<T: Debug, const N: usize> error::Error for IntoArrayError<T, N>
{
}
