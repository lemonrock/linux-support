// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Canonical chain error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CanonicalChainError
{
	/// We limit our canonical chain (contains our maximum chain length).
	TooManyCanonicalNamesInChain(usize),
	
	/// We rely on canonical chain links been sorted.
	///
	/// RFC 6672, Section 3.2 Server Algorithm Step 3.A. implies that canonical chain links will be sorted.
	CanonicalNamesNotSorted,
	
	/// This avoids a CNAME loop.
	CanonicalNameChainCanNotIncludeQueryNameAsItCreatesALoop,
	
	/// This avoids a CNAME loop.
	AddingNameToCanonicalNameChainCreatesALoop,

	/// Duplicate DNAME.
	MoreThanOneDNAMEWithTheSameOwnerName,
}

impl Display for CanonicalChainError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CanonicalChainError
{
}
