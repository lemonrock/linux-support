// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct QueryTypeCache<ORs: OwnedRecords<OR>, OR: OwnedRecord>
{
	cache_until: CacheUntil,
	
	/// If `None`, then this is a negatively cached `NoData` answer.
	data: Option<ORs>
}

impl<ORs: OwnedRecords<OR>, OR: OwnedRecord> QueryTypeCache<ORs, OR>
{
	#[inline(always)]
	pub(crate) fn data(cache_until: CacheUntil, records: Records) -> Option<Self>
	{
		Some
		(
			Self
			{
				cache_until,
				data: Some(records)
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn no_data(negative_cache_until: CacheUntil) -> Option<Self>
	{
		Some
		(
			Self
			{
				cache_until: negative_cache_until,
				data: None
			}
		)
	}
}
