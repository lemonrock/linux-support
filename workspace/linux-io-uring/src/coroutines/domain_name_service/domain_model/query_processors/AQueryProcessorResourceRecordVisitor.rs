// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct AQueryProcessorResourceRecordVisitor<'cache: 'message, 'message>
{
	query_name: &'message CaseFoldedName<'cache>,
	
	records: Records<'cache, Ipv4Addr>,
}

impl<'cache: 'message, 'message> ResourceRecordVisitor<'message> for AQueryProcessorResourceRecordVisitor<'cache, 'message>
{
	type Error = Infallible;
	
	type Finished = Records<'cache, Ipv4Addr>;
	
	#[inline(always)]
	fn finished(self) -> Self::Finished
	{
		self.present
	}
	
	#[inline(always)]
	fn A(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: Ipv4Addr) -> Result<(), Self::Error>
	{
		self.records.store_unprioritized_and_unweighted(&name, cache_until, record);
		Ok(())
	}
}
