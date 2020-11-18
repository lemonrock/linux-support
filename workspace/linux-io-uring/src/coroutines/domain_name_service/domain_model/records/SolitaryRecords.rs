// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// For record type:-
///
/// * `SOA`.
/// * Possibly `DNAME`.
/// * Not used for `CNAME`.
#[derive(Clone)]
pub(crate) struct SolitaryRecords<OR: OwnedRecord + Ord + Clone>
{
	pub(crate) negative_cache_until: NegativeCacheUntil,
	pub(crate) record: OR,
}

impl<OR: OwnedRecord + Ord + Clone> SolitaryRecords<OR>
{
	#[inline(always)]
	pub(crate) const fn new(negative_cache_until: NegativeCacheUntil, record: OR) -> Self
	{
		Self
		{
			negative_cache_until,
			record
		}
	}
}
